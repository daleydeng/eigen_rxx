{% set mats = ["Matrix2d", "Matrix3d", "Matrix4d"] -%}
{% set vecs = ["Vector2d", "Vector3d", "Vector4d"] -%}
{% set sizes = {"Matrix2d": 4, "Matrix3d": 9, "Matrix4d": 16, "Vector2d": 2, "Vector3d": 3, "Vector4d": 4} -%}
{% set vmats = ["MatrixXd"] -%}
{% set fixed = vecs + mats -%}

use std::ops::{Add, Sub, Mul};
use std::marker::PhantomData;

{% for i in fixed + vmats -%}
{% if i not in sizes -%}
#[repr(C)]
#[derive(Debug)]
pub struct {{i}} {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
}
{% else %}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub data: [f64; {{ sizes[i] }}]
}
{% endif %}

unsafe impl cxx::ExternType for {{i}} {
    type Id = cxx::type_id!("rxx::{{i}}");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{i}}<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_{{i}}<'_> {
    type Id = cxx::type_id!("rxx::Map_{{i}}");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{i}}_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_{{i}}_const<'_> {
    type Id = cxx::type_id!("rxx::Map_{{i}}_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{i}}_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_{{i}}_stride<'_> {
    type Id = cxx::type_id!("rxx::Map_{{i}}_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{i}}_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_{{i}}_const_stride<'_> {
    type Id = cxx::type_id!("rxx::Map_{{i}}_const_stride");
    type Kind = cxx::kind::Trivial;
}

{% endfor %}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Quaterniond {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

unsafe impl cxx::ExternType for Quaterniond {
    type Id = cxx::type_id!("rxx::Quaterniond");
    type Kind = cxx::kind::Trivial;
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct AngleAxisd {
    pub axis: Vector3d,
    pub angle: f64,
}

unsafe impl cxx::ExternType for AngleAxisd {
    type Id = cxx::type_id!("rxx::AngleAxisd");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge(namespace = "rxx")]
mod ffi {
    unsafe extern "C++" {
        include!("eigen_rxx/include/wrapper.hh");

        {% for i in fixed + vmats -%}
        type {{i}} = super::{{i}};
        type Map_{{i}}<'a> = super::Map_{{i}}<'a>;
        type Map_{{i}}_const<'a> = super::Map_{{i}}_const<'a>;
        {% if i == "MatrixXd" -%}
        type Map_{{i}}_stride<'a> = super::Map_{{i}}_stride<'a>;
        type Map_{{i}}_const_stride<'a> = super::Map_{{i}}_const_stride<'a>;
        {% endif %}

        #[rust_name = "{{i}}_add"]
        fn op_add(self_: &{{i}}, other: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_sub"]
        fn op_sub(self_: &{{i}}, other: &{{i}}) -> {{i}};
        {% endfor %}

        {% for i in mats + vmats -%}
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> {{i}};
        fn setIdentity(self: Pin<&mut {{i}}>) -> Pin<&mut {{i}}>;
        {% endfor %}

        fn MatrixXd_new(rows: usize, cols: usize) -> MatrixXd;
        fn MatrixXd_clone(v: &MatrixXd) -> MatrixXd;

        unsafe fn Map_MatrixXd_new<'a>(data: *mut f64, rows: usize, cols: usize) -> Map_MatrixXd<'a>;
        unsafe fn Map_MatrixXd_const_new<'a>(data: *const f64, rows: usize, cols: usize) -> Map_MatrixXd_const<'a>;

        unsafe fn Map_MatrixXd_stride_new<'a>(data: *mut f64, rows: usize, cols: usize, sx: usize, sy: usize) -> Map_MatrixXd_stride<'a>;
        unsafe fn Map_MatrixXd_const_stride_new<'a>(data: *const f64, rows: usize, cols: usize, sx: usize, sy: usize) -> Map_MatrixXd_const_stride<'a>;

        {% for i in fixed -%}
        #[rust_name = "Map_{{i}}_new"]
        unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_{{i}}<'a>;
        #[rust_name = "Map_{{i}}_const_new"]
        unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_{{i}}_const<'a>;
        {% endfor %}

        {% for i in ["Quaterniond"] -%}
        type {{i}} = super::{{i}};
        fn normalized(self: &{{i}}) -> {{i}};
        fn normalize(self: Pin<&mut {{i}}>);
        fn inverse(self: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> {{i}};
        fn toRotationMatrix(self: &{{i}}) -> Matrix3d;
        {% endfor %}

        {% for i in ["AngleAxisd"] -%}
        type {{i}} = super::{{i}};
        fn inverse(self: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> Quaterniond;
        fn toRotationMatrix(self: &{{i}}) -> Matrix3d;
        {% endfor %}


    }
}

impl MatrixXd {
    pub fn new(rows: usize, cols: usize) -> Self {
        ffi::MatrixXd_new(rows, cols)
    }
}

impl Clone for MatrixXd {
    fn clone(&self) -> Self {
        ffi::MatrixXd_clone(self)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = ffi::MatrixXd_clone(source);
    }
}

impl<'a> Map_MatrixXd<'a> {
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize) -> Self {
        unsafe {
            ffi::Map_MatrixXd_new::<'a>(data.as_mut_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_MatrixXd_const<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize) -> Self {
        unsafe {
            ffi::Map_MatrixXd_const_new::<'a>(data.as_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_MatrixXd_stride<'a> {
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize, sx: usize, sy: usize) -> Self {
        unsafe {
            ffi::Map_MatrixXd_stride_new::<'a>(data.as_mut_ptr(), rows, cols, sx, sy)
        }
    }
}

impl<'a> Map_MatrixXd_const_stride<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize, sx: usize, sy: usize) -> Self {
        unsafe {
            ffi::Map_MatrixXd_const_stride_new::<'a>(data.as_ptr(), rows, cols, sx, sy)
        }
    }
}

{% for i in fixed -%}
impl<'a> Map_{{i}}<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_{{i}}_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_{{i}}_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_{{i}}_const_new::<'a>(data.as_ptr())
        }
    }
}

{% endfor %}

{% for i in fixed + vmats -%}
impl Add for {{i}} {
    type Output = {{i}};

    fn add(self, other: {{i}}) -> Self::Output {
        ffi::{{i}}_add(&self, &other)
    }
}

impl Add for &{{i}} {
    type Output = {{i}};
    fn add(self, other: &{{i}}) -> Self::Output {
        ffi::{{i}}_add(self, other)
    }
}

impl Sub for {{i}} {
    type Output = {{i}};

    fn sub(self, other: {{i}}) -> {{i}} {
        ffi::{{i}}_sub(&self, &other)
    }
}

impl Sub for &{{i}} {
    type Output = {{i}};
    fn sub(self, other: &{{i}}) -> Self::Output {
        ffi::{{i}}_sub(self, other)
    }
}
{% endfor %}

{% for i in mats + vmats + ["Quaterniond"]-%}
impl Mul for {{i}} {
    type Output = {{i}};

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(&self, &other)
    }
}

impl Mul for &{{i}} {
    type Output = {{i}};

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(self, other)
    }
}
{% endfor %}

{% set i = "AngleAxisd" -%}
impl Mul for {{i}} {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(&self, &other)
    }
}

impl Mul for &{{i}} {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::{{i}}_mul(self, other)
    }
}

{% set mats = [("Matrix2d", 4), ("Matrix3d", 9), ("Matrix4d", 16)] -%}
{% set vecs = [("Vector2d", 2), ("Vector3d", 3), ("Vector4d", 4)] -%}
{% set vmats = [("MatrixXd", None)] -%}
{% set fixes = vecs + mats -%}

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::marker::PhantomData;

{% for i, s in fixes + vmats -%}
{% if not s -%}
#[repr(C)]
#[derive(Debug)]
pub struct {{i}} {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
}
{% else %}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub data: [f64; {{ s }}]
}
{% endif %}

unsafe impl cxx::ExternType for {{i}} {
    type Id = cxx::type_id!("eigen_rxx::{{i}}");
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
    type Id = cxx::type_id!("eigen_rxx::Map_{{i}}");
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
    type Id = cxx::type_id!("eigen_rxx::Map_{{i}}_const");
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
    type Id = cxx::type_id!("eigen_rxx::Map_{{i}}_stride");
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
    type Id = cxx::type_id!("eigen_rxx::Map_{{i}}_const_stride");
    type Kind = cxx::kind::Trivial;
}

{% endfor %}

#[cxx::bridge(namespace = "eigen_rxx")]
pub mod ffi {
    unsafe extern "C++" {
        include!("eigen_rxx/include/wrapper.hh");

        {% for i, _ in fixes + vmats -%}
        type {{i}} = super::{{i}};
        type Map_{{i}}<'a> = super::Map_{{i}}<'a>;
        type Map_{{i}}_const<'a> = super::Map_{{i}}_const<'a>;
        type Map_{{i}}_stride<'a> = super::Map_{{i}}_stride<'a>;
        type Map_{{i}}_const_stride<'a> = super::Map_{{i}}_const_stride<'a>;

        #[rust_name = "{{i}}_add"]
        fn op_add(self_: &{{i}}, other: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_sub"]
        fn op_sub(self_: &{{i}}, other: &{{i}}) -> {{i}};
        {% endfor %}

        {% for i, _ in mats + vmats -%}
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
    }
}

use crate::ffi::*;

impl MatrixXd {
    pub fn new(rows: usize, cols: usize) -> Self {
        MatrixXd_new(rows, cols)
    }
}

impl Clone for MatrixXd {
    fn clone(&self) -> Self {
        MatrixXd_clone(self)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = MatrixXd_clone(source);
    }
}

impl<'a> Map_MatrixXd<'a> {
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize) -> Self {
        unsafe {
            Map_MatrixXd_new::<'a>(data.as_mut_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_MatrixXd_const<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize) -> Self {
        unsafe {
            Map_MatrixXd_const_new::<'a>(data.as_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_MatrixXd_stride<'a> {
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize, sx: usize, sy: usize) -> Self {
        unsafe {
            Map_MatrixXd_stride_new::<'a>(data.as_mut_ptr(), rows, cols, sx, sy)
        }
    }
}

impl<'a> Map_MatrixXd_const_stride<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize, sx: usize, sy: usize) -> Self {
        unsafe {
            Map_MatrixXd_const_stride_new::<'a>(data.as_ptr(), rows, cols, sx, sy)
        }
    }
}

{% for i, s in fixes + vmats -%}
impl Add for {{i}} {
    type Output = {{i}};

    fn add(self, other: {{i}}) -> Self::Output {
        {{i}}_add(&self, &other)
    }
}

impl Add for &{{i}} {
    type Output = {{i}};
    fn add(self, other: &{{i}}) -> Self::Output {
        {{i}}_add(self, other)
    }
}

impl Sub for {{i}} {
    type Output = {{i}};

    fn sub(self, other: {{i}}) -> {{i}} {
        {{i}}_sub(&self, &other)
    }
}

impl Sub for &{{i}} {
    type Output = {{i}};
    fn sub(self, other: &{{i}}) -> Self::Output {
        {{i}}_sub(self, other)
    }
}
{% endfor %}

{% for i, _ in mats + vmats -%}
impl Mul for {{i}} {
    type Output = {{i}};

    fn mul(self, other: {{i}}) -> {{i}} {
        {{i}}_mul(&self, &other)
    }
}

impl Mul for &{{i}} {
    type Output = {{i}};

    fn mul(self, other: &{{i}}) -> {{i}} {
        {{i}}_mul(self, other)
    }
}
{% endfor %}

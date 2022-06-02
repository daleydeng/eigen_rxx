{% from "eigen_map.rs" import struct_eigen_map_fixed, struct_eigen_map_matx, struct_eigen_map_vecx, ffi_eigen_map_fixed, ffi_eigen_map_matx, ffi_eigen_map_vecx, impl_eigen_map_fixed, impl_eigen_map_matx, impl_eigen_map_vecx -%}
{% set objs = {
    "Matrix2d": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "size": 4,
    },
    "Matrix3d": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "size": 9,
    },
    "Matrix4d": {
        "mul": True,
        "var": 0,
        "sqr": True,
        "size": 16,
    },
    "Matrix2x3d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "size": 6,
    },
    "MatrixXd": {
        "mul": True,
        "var": 2,
        "sqr": False,
        "size": 0,
    },
    "Vector2d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "size": 2,
    },
    "Vector3d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "size": 3,
    },
    "Vector4d": {
        "mul": False,
        "var": 0,
        "sqr": False,
        "size": 4,
    },
    "VectorXd": {
        "mul": False,
        "var": 1,
        "sqr": False,
        "size": 0,
    },
} -%}

use std::ops::{Add, Sub, Mul};

{% for i, v in objs.items() -%}
{% if v["var"] == 2 -%}

#[repr(C)]
#[derive(Debug)]
pub struct {{i}} {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_matx(i) }}

{% elif v["var"] == 1 -%}
#[repr(C)]
#[derive(Debug)]
pub struct {{i}} {
    pub data: *mut f64,
    pub size: isize,
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_vecx(i) }}

{% else %}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub data: [f64; {{ v["size"] }}]
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_fixed(i) }}

{% endif %} // var
{% endfor %} // objs

{% set i = "Quaterniond" -%}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_fixed(i) }}

{% set i = "AngleAxisd" -%}
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct {{i}} {
    pub axis: Vector3d,
    pub angle: f64,
}
{{ impl_ExternType(i) }}
{{ struct_eigen_map_fixed(i) }}

#[cxx::bridge(namespace = "rxx")]
mod ffi {
    unsafe extern "C++" {
        include!("eigen_rxx/include/wrapper.hh");

        {% for i, v in objs.items() -%}
        type {{i}} = super::{{i}};

        #[rust_name = "{{i}}_add"]
        fn op_add(self_: &{{i}}, other: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_sub"]
        fn op_sub(self_: &{{i}}, other: &{{i}}) -> {{i}};

        {% if v["mul"] %}
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> {{i}};
        {% endif -%}
        {% if v["sqr"] %}
        fn setIdentity(self: Pin<&mut {{i}}>) -> Pin<&mut {{i}}>;
        {% endif -%}

        {% if v["var"] == 2 %}
        fn {{i}}_new(rows: usize, cols: usize) -> {{i}};
        fn {{i}}_clone(v: &{{i}}) -> {{i}};

        {{ ffi_eigen_map_matx(i) }}

        {% elif v["var"] == 1 %}
        fn {{i}}_new(size: usize) -> {{i}};
        fn {{i}}_clone(v: &{{i}}) -> {{i}};

        {{ ffi_eigen_map_vecx(i) }}

        {% elif v["var"] == 0 -%}

        {{ ffi_eigen_map_fixed(i) }}

        {% endif -%}

        {% endfor %} // objs

        {% for i in ["Quaterniond"] -%}
        type {{i}} = super::{{i}};
        fn normalized(self: &{{i}}) -> {{i}};
        fn normalize(self: Pin<&mut {{i}}>);
        fn inverse(self: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> {{i}};
        fn toRotationMatrix(self: &{{i}}) -> Matrix3d;

        {{ ffi_eigen_map_fixed(i) }}

        {% endfor %}

        {% for i in ["AngleAxisd"] -%}
        type {{i}} = super::{{i}};
        fn inverse(self: &{{i}}) -> {{i}};
        #[rust_name = "{{i}}_mul"]
        fn op_mul(self_: &{{i}}, other: &{{i}}) -> Quaterniond;
        fn toRotationMatrix(self: &{{i}}) -> Matrix3d;

        {{ ffi_eigen_map_fixed(i) }}

        {% endfor %}

    }
}

impl VectorXd {
    pub fn new(size: usize) -> Self {
        ffi::VectorXd_new(size)
    }
}

impl Clone for VectorXd {
    fn clone(&self) -> Self {
        ffi::VectorXd_clone(self)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = ffi::VectorXd_clone(source);
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

{% for i, v in objs.items() -%}

{% if v["var"] == 0 -%}
{{ impl_eigen_map_fixed(i) }}
{% elif v["var"] == 1 -%}
{{ impl_eigen_map_vecx(i) }}
{% elif v["var"] == 2 -%}
{{ impl_eigen_map_matx(i) }}
{% endif %}

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

{% if v["mul"] %}
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
{% endif %}

{% endfor %} // objs

{% set i = "Quaterniond"-%}
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
{{ impl_eigen_map_fixed(i) }}

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
{{ impl_eigen_map_fixed(i) }}

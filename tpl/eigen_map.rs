{% macro struct_eigen_map_fixed(cls) -%}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

{{ impl_ExternType("Map_"+cls, True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

{{ impl_ExternType("Map_"+cls+"_const", True) }}

{%- endmacro %}

{% macro struct_eigen_map_matx(cls) -%}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls, True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls+"_const", True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub s0: isize,
    pub s1: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls+"_stride", True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub s0: isize,
    pub s1: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls+"_const_stride", True) }}

{%- endmacro %}

{% macro struct_eigen_map_vecx(cls) -%}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}<'a> {
    pub data: *mut f64,
    pub size: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls, True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_const<'a> {
    pub data: *const f64,
    pub size: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls+"_const", True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_stride<'a> {
    pub data: *mut f64,
    pub size: isize,
    pub stride: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls+"_stride", True) }}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_{{cls}}_const_stride<'a> {
    pub data: *const f64,
    pub size: isize,
    pub stride: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
{{ impl_ExternType("Map_"+cls+"_const_stride", True) }}

{%- endmacro %}

{% macro ffi_eigen_map_fixed(cls) -%}

type Map_{{cls}}<'a> = super::Map_{{cls}}<'a>;
type Map_{{cls}}_const<'a> = super::Map_{{cls}}_const<'a>;
#[rust_name = "Map_{{cls}}_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_{{cls}}<'a>;
#[rust_name = "Map_{{cls}}_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_{{cls}}_const<'a>;

{%- endmacro %}

{% macro ffi_eigen_map_matx(cls) -%}

type Map_{{cls}}<'a> = super::Map_{{cls}}<'a>;
type Map_{{cls}}_const<'a> = super::Map_{{cls}}_const<'a>;
type Map_{{cls}}_stride<'a> = super::Map_{{cls}}_stride<'a>;
type Map_{{cls}}_const_stride<'a> = super::Map_{{cls}}_const_stride<'a>;
unsafe fn Map_{{cls}}_new<'a>(data: *mut f64, rows: usize, cols: usize) -> Map_{{cls}}<'a>;
unsafe fn Map_{{cls}}_const_new<'a>(data: *const f64, rows: usize, cols: usize) -> Map_{{cls}}_const<'a>;
unsafe fn Map_{{cls}}_stride_new<'a>(data: *mut f64, rows: usize, cols: usize, s0: usize, s1: usize) -> Map_{{cls}}_stride<'a>;
unsafe fn Map_{{cls}}_const_stride_new<'a>(data: *const f64, rows: usize, cols: usize, s0: usize, s1: usize) -> Map_{{cls}}_const_stride<'a>;

{%- endmacro %}

{% macro ffi_eigen_map_vecx(cls) -%}

type Map_{{cls}}<'a> = super::Map_{{cls}}<'a>;
type Map_{{cls}}_const<'a> = super::Map_{{cls}}_const<'a>;
type Map_{{cls}}_stride<'a> = super::Map_{{cls}}_stride<'a>;
type Map_{{cls}}_const_stride<'a> = super::Map_{{cls}}_const_stride<'a>;
unsafe fn Map_{{cls}}_new<'a>(data: *mut f64, size: usize) -> Map_{{cls}}<'a>;
unsafe fn Map_{{cls}}_const_new<'a>(data: *const f64, size: usize) -> Map_{{cls}}_const<'a>;

unsafe fn Map_{{cls}}_stride_new<'a>(data: *mut f64, size: usize, s: usize) -> Map_{{cls}}_stride<'a>;
unsafe fn Map_{{cls}}_const_stride_new<'a>(data: *const f64, size: usize, s: usize) -> Map_{{cls}}_const_stride<'a>;

{%- endmacro %}

{% macro impl_eigen_map_fixed(cls) -%}

impl<'a> Map_{{cls}}<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_{{cls}}_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_{{cls}}_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_{{cls}}_const_new::<'a>(data.as_ptr())
        }
    }
}

{%- endmacro %}

{% macro impl_eigen_map_matx(cls) -%}

impl<'a> Map_{{cls}}<'a> {
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_new::<'a>(data.as_mut_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_{{cls}}_const<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_const_new::<'a>(data.as_ptr(), rows, cols)
        }
    }
}

impl<'a> Map_{{cls}}_stride<'a> {
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize, s0: usize, s1: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_stride_new::<'a>(data.as_mut_ptr(), rows, cols, s0, s1)
        }
    }
}

impl<'a> Map_{{cls}}_const_stride<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize, s0: usize, s1: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_const_stride_new::<'a>(data.as_ptr(), rows, cols, s0, s1)
        }
    }
}

{%- endmacro %}

{% macro impl_eigen_map_vecx(cls) -%}

impl<'a> Map_{{cls}}<'a> {
    pub fn new(data: &'a mut [f64], size: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_new::<'a>(data.as_mut_ptr(), size)
        }
    }
}

impl<'a> Map_{{cls}}_const<'a> {
    pub fn new(data: &'a [f64], size: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_const_new::<'a>(data.as_ptr(), size)
        }
    }
}

impl<'a> Map_{{cls}}_stride<'a> {
    pub fn new(data: &'a mut [f64], size: usize, s: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_stride_new::<'a>(data.as_mut_ptr(), size, s)
        }
    }
}

impl<'a> Map_{{cls}}_const_stride<'a> {
    pub fn new(data: &'a [f64], size: usize, s: usize) -> Self {
        unsafe {
            ffi::Map_{{cls}}_const_stride_new::<'a>(data.as_ptr(), size, s)
        }
    }
}

{%- endmacro %}

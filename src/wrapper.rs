use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::marker::PhantomData;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2d {
    pub data: [f64; 2]
}


unsafe impl cxx::ExternType for Vector2d {
    type Id = cxx::type_id!("eigen_rxx::Vector2d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector2d<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector2d<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector2d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector2d_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector2d_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector2d_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector2d_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector2d_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector2d_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector2d_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector2d_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector2d_const_stride");
    type Kind = cxx::kind::Trivial;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3d {
    pub data: [f64; 3]
}


unsafe impl cxx::ExternType for Vector3d {
    type Id = cxx::type_id!("eigen_rxx::Vector3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector3d<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector3d<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector3d_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector3d_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector3d_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector3d_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector3d_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector3d_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector3d_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector3d_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector3d_const_stride");
    type Kind = cxx::kind::Trivial;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4d {
    pub data: [f64; 4]
}


unsafe impl cxx::ExternType for Vector4d {
    type Id = cxx::type_id!("eigen_rxx::Vector4d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector4d<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector4d<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector4d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector4d_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector4d_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector4d_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector4d_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector4d_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector4d_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector4d_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector4d_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Vector4d_const_stride");
    type Kind = cxx::kind::Trivial;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix2d {
    pub data: [f64; 4]
}


unsafe impl cxx::ExternType for Matrix2d {
    type Id = cxx::type_id!("eigen_rxx::Matrix2d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2d<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2d<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix2d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2d_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2d_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix2d_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2d_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2d_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix2d_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2d_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2d_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix2d_const_stride");
    type Kind = cxx::kind::Trivial;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix3d {
    pub data: [f64; 9]
}


unsafe impl cxx::ExternType for Matrix3d {
    type Id = cxx::type_id!("eigen_rxx::Matrix3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix3d<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix3d<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix3d_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix3d_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix3d_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix3d_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix3d_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix3d_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix3d_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix3d_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix3d_const_stride");
    type Kind = cxx::kind::Trivial;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Matrix4d {
    pub data: [f64; 16]
}


unsafe impl cxx::ExternType for Matrix4d {
    type Id = cxx::type_id!("eigen_rxx::Matrix4d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix4d<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix4d<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix4d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix4d_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix4d_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix4d_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix4d_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix4d_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix4d_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix4d_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix4d_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_Matrix4d_const_stride");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[derive(Debug)]
pub struct MatrixXd {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
}


unsafe impl cxx::ExternType for MatrixXd {
    type Id = cxx::type_id!("eigen_rxx::MatrixXd");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_MatrixXd<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_MatrixXd");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_MatrixXd_const<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_MatrixXd_const");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_MatrixXd_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_MatrixXd_stride");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub sx: isize,
    pub sy: isize,
    _pd: PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_MatrixXd_const_stride<'_> {
    type Id = cxx::type_id!("eigen_rxx::Map_MatrixXd_const_stride");
    type Kind = cxx::kind::Trivial;
}



#[cxx::bridge(namespace = "eigen_rxx")]
pub mod ffi {
    unsafe extern "C++" {
        include!("eigen_rxx/include/wrapper.hh");

        type Vector2d = super::Vector2d;
        type Map_Vector2d<'a> = super::Map_Vector2d<'a>;
        type Map_Vector2d_const<'a> = super::Map_Vector2d_const<'a>;
        type Map_Vector2d_stride<'a> = super::Map_Vector2d_stride<'a>;
        type Map_Vector2d_const_stride<'a> = super::Map_Vector2d_const_stride<'a>;

        #[rust_name = "Vector2d_add"]
        fn op_add(self_: &Vector2d, other: &Vector2d) -> Vector2d;
        #[rust_name = "Vector2d_sub"]
        fn op_sub(self_: &Vector2d, other: &Vector2d) -> Vector2d;
        type Vector3d = super::Vector3d;
        type Map_Vector3d<'a> = super::Map_Vector3d<'a>;
        type Map_Vector3d_const<'a> = super::Map_Vector3d_const<'a>;
        type Map_Vector3d_stride<'a> = super::Map_Vector3d_stride<'a>;
        type Map_Vector3d_const_stride<'a> = super::Map_Vector3d_const_stride<'a>;

        #[rust_name = "Vector3d_add"]
        fn op_add(self_: &Vector3d, other: &Vector3d) -> Vector3d;
        #[rust_name = "Vector3d_sub"]
        fn op_sub(self_: &Vector3d, other: &Vector3d) -> Vector3d;
        type Vector4d = super::Vector4d;
        type Map_Vector4d<'a> = super::Map_Vector4d<'a>;
        type Map_Vector4d_const<'a> = super::Map_Vector4d_const<'a>;
        type Map_Vector4d_stride<'a> = super::Map_Vector4d_stride<'a>;
        type Map_Vector4d_const_stride<'a> = super::Map_Vector4d_const_stride<'a>;

        #[rust_name = "Vector4d_add"]
        fn op_add(self_: &Vector4d, other: &Vector4d) -> Vector4d;
        #[rust_name = "Vector4d_sub"]
        fn op_sub(self_: &Vector4d, other: &Vector4d) -> Vector4d;
        type Matrix2d = super::Matrix2d;
        type Map_Matrix2d<'a> = super::Map_Matrix2d<'a>;
        type Map_Matrix2d_const<'a> = super::Map_Matrix2d_const<'a>;
        type Map_Matrix2d_stride<'a> = super::Map_Matrix2d_stride<'a>;
        type Map_Matrix2d_const_stride<'a> = super::Map_Matrix2d_const_stride<'a>;

        #[rust_name = "Matrix2d_add"]
        fn op_add(self_: &Matrix2d, other: &Matrix2d) -> Matrix2d;
        #[rust_name = "Matrix2d_sub"]
        fn op_sub(self_: &Matrix2d, other: &Matrix2d) -> Matrix2d;
        type Matrix3d = super::Matrix3d;
        type Map_Matrix3d<'a> = super::Map_Matrix3d<'a>;
        type Map_Matrix3d_const<'a> = super::Map_Matrix3d_const<'a>;
        type Map_Matrix3d_stride<'a> = super::Map_Matrix3d_stride<'a>;
        type Map_Matrix3d_const_stride<'a> = super::Map_Matrix3d_const_stride<'a>;

        #[rust_name = "Matrix3d_add"]
        fn op_add(self_: &Matrix3d, other: &Matrix3d) -> Matrix3d;
        #[rust_name = "Matrix3d_sub"]
        fn op_sub(self_: &Matrix3d, other: &Matrix3d) -> Matrix3d;
        type Matrix4d = super::Matrix4d;
        type Map_Matrix4d<'a> = super::Map_Matrix4d<'a>;
        type Map_Matrix4d_const<'a> = super::Map_Matrix4d_const<'a>;
        type Map_Matrix4d_stride<'a> = super::Map_Matrix4d_stride<'a>;
        type Map_Matrix4d_const_stride<'a> = super::Map_Matrix4d_const_stride<'a>;

        #[rust_name = "Matrix4d_add"]
        fn op_add(self_: &Matrix4d, other: &Matrix4d) -> Matrix4d;
        #[rust_name = "Matrix4d_sub"]
        fn op_sub(self_: &Matrix4d, other: &Matrix4d) -> Matrix4d;
        type MatrixXd = super::MatrixXd;
        type Map_MatrixXd<'a> = super::Map_MatrixXd<'a>;
        type Map_MatrixXd_const<'a> = super::Map_MatrixXd_const<'a>;
        type Map_MatrixXd_stride<'a> = super::Map_MatrixXd_stride<'a>;
        type Map_MatrixXd_const_stride<'a> = super::Map_MatrixXd_const_stride<'a>;

        #[rust_name = "MatrixXd_add"]
        fn op_add(self_: &MatrixXd, other: &MatrixXd) -> MatrixXd;
        #[rust_name = "MatrixXd_sub"]
        fn op_sub(self_: &MatrixXd, other: &MatrixXd) -> MatrixXd;
        

        #[rust_name = "Matrix2d_mul"]
        fn op_mul(self_: &Matrix2d, other: &Matrix2d) -> Matrix2d;
        fn setIdentity(self: Pin<&mut Matrix2d>) -> Pin<&mut Matrix2d>;
        #[rust_name = "Matrix3d_mul"]
        fn op_mul(self_: &Matrix3d, other: &Matrix3d) -> Matrix3d;
        fn setIdentity(self: Pin<&mut Matrix3d>) -> Pin<&mut Matrix3d>;
        #[rust_name = "Matrix4d_mul"]
        fn op_mul(self_: &Matrix4d, other: &Matrix4d) -> Matrix4d;
        fn setIdentity(self: Pin<&mut Matrix4d>) -> Pin<&mut Matrix4d>;
        #[rust_name = "MatrixXd_mul"]
        fn op_mul(self_: &MatrixXd, other: &MatrixXd) -> MatrixXd;
        fn setIdentity(self: Pin<&mut MatrixXd>) -> Pin<&mut MatrixXd>;
        

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

impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Vector2d) -> Self::Output {
        Vector2d_add(&self, &other)
    }
}

impl Add for &Vector2d {
    type Output = Vector2d;
    fn add(self, other: &Vector2d) -> Self::Output {
        Vector2d_add(self, other)
    }
}

impl Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Vector2d) -> Vector2d {
        Vector2d_sub(&self, &other)
    }
}

impl Sub for &Vector2d {
    type Output = Vector2d;
    fn sub(self, other: &Vector2d) -> Self::Output {
        Vector2d_sub(self, other)
    }
}
impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, other: Vector3d) -> Self::Output {
        Vector3d_add(&self, &other)
    }
}

impl Add for &Vector3d {
    type Output = Vector3d;
    fn add(self, other: &Vector3d) -> Self::Output {
        Vector3d_add(self, other)
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Vector3d {
        Vector3d_sub(&self, &other)
    }
}

impl Sub for &Vector3d {
    type Output = Vector3d;
    fn sub(self, other: &Vector3d) -> Self::Output {
        Vector3d_sub(self, other)
    }
}
impl Add for Vector4d {
    type Output = Vector4d;

    fn add(self, other: Vector4d) -> Self::Output {
        Vector4d_add(&self, &other)
    }
}

impl Add for &Vector4d {
    type Output = Vector4d;
    fn add(self, other: &Vector4d) -> Self::Output {
        Vector4d_add(self, other)
    }
}

impl Sub for Vector4d {
    type Output = Vector4d;

    fn sub(self, other: Vector4d) -> Vector4d {
        Vector4d_sub(&self, &other)
    }
}

impl Sub for &Vector4d {
    type Output = Vector4d;
    fn sub(self, other: &Vector4d) -> Self::Output {
        Vector4d_sub(self, other)
    }
}
impl Add for Matrix2d {
    type Output = Matrix2d;

    fn add(self, other: Matrix2d) -> Self::Output {
        Matrix2d_add(&self, &other)
    }
}

impl Add for &Matrix2d {
    type Output = Matrix2d;
    fn add(self, other: &Matrix2d) -> Self::Output {
        Matrix2d_add(self, other)
    }
}

impl Sub for Matrix2d {
    type Output = Matrix2d;

    fn sub(self, other: Matrix2d) -> Matrix2d {
        Matrix2d_sub(&self, &other)
    }
}

impl Sub for &Matrix2d {
    type Output = Matrix2d;
    fn sub(self, other: &Matrix2d) -> Self::Output {
        Matrix2d_sub(self, other)
    }
}
impl Add for Matrix3d {
    type Output = Matrix3d;

    fn add(self, other: Matrix3d) -> Self::Output {
        Matrix3d_add(&self, &other)
    }
}

impl Add for &Matrix3d {
    type Output = Matrix3d;
    fn add(self, other: &Matrix3d) -> Self::Output {
        Matrix3d_add(self, other)
    }
}

impl Sub for Matrix3d {
    type Output = Matrix3d;

    fn sub(self, other: Matrix3d) -> Matrix3d {
        Matrix3d_sub(&self, &other)
    }
}

impl Sub for &Matrix3d {
    type Output = Matrix3d;
    fn sub(self, other: &Matrix3d) -> Self::Output {
        Matrix3d_sub(self, other)
    }
}
impl Add for Matrix4d {
    type Output = Matrix4d;

    fn add(self, other: Matrix4d) -> Self::Output {
        Matrix4d_add(&self, &other)
    }
}

impl Add for &Matrix4d {
    type Output = Matrix4d;
    fn add(self, other: &Matrix4d) -> Self::Output {
        Matrix4d_add(self, other)
    }
}

impl Sub for Matrix4d {
    type Output = Matrix4d;

    fn sub(self, other: Matrix4d) -> Matrix4d {
        Matrix4d_sub(&self, &other)
    }
}

impl Sub for &Matrix4d {
    type Output = Matrix4d;
    fn sub(self, other: &Matrix4d) -> Self::Output {
        Matrix4d_sub(self, other)
    }
}
impl Add for MatrixXd {
    type Output = MatrixXd;

    fn add(self, other: MatrixXd) -> Self::Output {
        MatrixXd_add(&self, &other)
    }
}

impl Add for &MatrixXd {
    type Output = MatrixXd;
    fn add(self, other: &MatrixXd) -> Self::Output {
        MatrixXd_add(self, other)
    }
}

impl Sub for MatrixXd {
    type Output = MatrixXd;

    fn sub(self, other: MatrixXd) -> MatrixXd {
        MatrixXd_sub(&self, &other)
    }
}

impl Sub for &MatrixXd {
    type Output = MatrixXd;
    fn sub(self, other: &MatrixXd) -> Self::Output {
        MatrixXd_sub(self, other)
    }
}


impl Mul for Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: Matrix2d) -> Matrix2d {
        Matrix2d_mul(&self, &other)
    }
}

impl Mul for &Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: &Matrix2d) -> Matrix2d {
        Matrix2d_mul(self, other)
    }
}
impl Mul for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, other: Matrix3d) -> Matrix3d {
        Matrix3d_mul(&self, &other)
    }
}

impl Mul for &Matrix3d {
    type Output = Matrix3d;

    fn mul(self, other: &Matrix3d) -> Matrix3d {
        Matrix3d_mul(self, other)
    }
}
impl Mul for Matrix4d {
    type Output = Matrix4d;

    fn mul(self, other: Matrix4d) -> Matrix4d {
        Matrix4d_mul(&self, &other)
    }
}

impl Mul for &Matrix4d {
    type Output = Matrix4d;

    fn mul(self, other: &Matrix4d) -> Matrix4d {
        Matrix4d_mul(self, other)
    }
}
impl Mul for MatrixXd {
    type Output = MatrixXd;

    fn mul(self, other: MatrixXd) -> MatrixXd {
        MatrixXd_mul(&self, &other)
    }
}

impl Mul for &MatrixXd {
    type Output = MatrixXd;

    fn mul(self, other: &MatrixXd) -> MatrixXd {
        MatrixXd_mul(self, other)
    }
}

use std::ops::{Add, Sub, Mul};


#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix2d {
    pub data: [f64; 4]
}
unsafe impl cxx::ExternType for Matrix2d {
    type Id = cxx::type_id!("rxx::Matrix2d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2d<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix2d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix2d_const");
    type Kind = cxx::kind::Trivial;
}


 // var

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix3d {
    pub data: [f64; 9]
}
unsafe impl cxx::ExternType for Matrix3d {
    type Id = cxx::type_id!("rxx::Matrix3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix3d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix3d<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix3d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix3d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix3d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix3d_const");
    type Kind = cxx::kind::Trivial;
}


 // var

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix4d {
    pub data: [f64; 16]
}
unsafe impl cxx::ExternType for Matrix4d {
    type Id = cxx::type_id!("rxx::Matrix4d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix4d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix4d<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix4d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix4d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix4d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix4d_const");
    type Kind = cxx::kind::Trivial;
}


 // var

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Matrix2x3d {
    pub data: [f64; 6]
}
unsafe impl cxx::ExternType for Matrix2x3d {
    type Id = cxx::type_id!("rxx::Matrix2x3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2x3d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2x3d<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix2x3d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Matrix2x3d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Matrix2x3d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Matrix2x3d_const");
    type Kind = cxx::kind::Trivial;
}


 // var
#[repr(C)]
#[derive(Debug)]
pub struct MatrixXd {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
}
unsafe impl cxx::ExternType for MatrixXd {
    type Id = cxx::type_id!("rxx::MatrixXd");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_MatrixXd<'_> {
    type Id = cxx::type_id!("rxx::Map_MatrixXd");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd_const<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_MatrixXd_const<'_> {
    type Id = cxx::type_id!("rxx::Map_MatrixXd_const");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd_stride<'a> {
    pub data: *mut f64,
    pub rows: isize,
    pub cols: isize,
    pub s0: isize,
    pub s1: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_MatrixXd_stride<'_> {
    type Id = cxx::type_id!("rxx::Map_MatrixXd_stride");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_MatrixXd_const_stride<'a> {
    pub data: *const f64,
    pub rows: isize,
    pub cols: isize,
    pub s0: isize,
    pub s1: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_MatrixXd_const_stride<'_> {
    type Id = cxx::type_id!("rxx::Map_MatrixXd_const_stride");
    type Kind = cxx::kind::Trivial;
}


 // var

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector2d {
    pub data: [f64; 2]
}
unsafe impl cxx::ExternType for Vector2d {
    type Id = cxx::type_id!("rxx::Vector2d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector2d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector2d<'_> {
    type Id = cxx::type_id!("rxx::Map_Vector2d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector2d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector2d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Vector2d_const");
    type Kind = cxx::kind::Trivial;
}


 // var

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector3d {
    pub data: [f64; 3]
}
unsafe impl cxx::ExternType for Vector3d {
    type Id = cxx::type_id!("rxx::Vector3d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector3d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector3d<'_> {
    type Id = cxx::type_id!("rxx::Map_Vector3d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector3d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector3d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Vector3d_const");
    type Kind = cxx::kind::Trivial;
}


 // var

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Vector4d {
    pub data: [f64; 4]
}
unsafe impl cxx::ExternType for Vector4d {
    type Id = cxx::type_id!("rxx::Vector4d");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector4d<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector4d<'_> {
    type Id = cxx::type_id!("rxx::Map_Vector4d");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Vector4d_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Vector4d_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Vector4d_const");
    type Kind = cxx::kind::Trivial;
}


 // var
#[repr(C)]
#[derive(Debug)]
pub struct VectorXd {
    pub data: *mut f64,
    pub size: isize,
}
unsafe impl cxx::ExternType for VectorXd {
    type Id = cxx::type_id!("rxx::VectorXd");
    type Kind = cxx::kind::Trivial;
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_VectorXd<'a> {
    pub data: *mut f64,
    pub size: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_VectorXd<'_> {
    type Id = cxx::type_id!("rxx::Map_VectorXd");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_VectorXd_const<'a> {
    pub data: *const f64,
    pub size: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_VectorXd_const<'_> {
    type Id = cxx::type_id!("rxx::Map_VectorXd_const");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_VectorXd_stride<'a> {
    pub data: *mut f64,
    pub size: isize,
    pub stride: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_VectorXd_stride<'_> {
    type Id = cxx::type_id!("rxx::Map_VectorXd_stride");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_VectorXd_const_stride<'a> {
    pub data: *const f64,
    pub size: isize,
    pub stride: isize,
    _pd: std::marker::PhantomData<&'a ()>,
}
unsafe impl cxx::ExternType for Map_VectorXd_const_stride<'_> {
    type Id = cxx::type_id!("rxx::Map_VectorXd_const_stride");
    type Kind = cxx::kind::Trivial;
}


 // var
 // objs

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

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Quaterniond<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Quaterniond<'_> {
    type Id = cxx::type_id!("rxx::Map_Quaterniond");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_Quaterniond_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_Quaterniond_const<'_> {
    type Id = cxx::type_id!("rxx::Map_Quaterniond_const");
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

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_AngleAxisd<'a> {
    pub data: *mut f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_AngleAxisd<'_> {
    type Id = cxx::type_id!("rxx::Map_AngleAxisd");
    type Kind = cxx::kind::Trivial;
}


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct Map_AngleAxisd_const<'a> {
    pub data: *const f64,
    _pd: std::marker::PhantomData<&'a ()>,
}

unsafe impl cxx::ExternType for Map_AngleAxisd_const<'_> {
    type Id = cxx::type_id!("rxx::Map_AngleAxisd_const");
    type Kind = cxx::kind::Trivial;
}


#[cxx::bridge(namespace = "rxx")]
mod ffi {
    unsafe extern "C++" {
        include!("eigen_rxx/include/wrapper.hh");

        type Matrix2d = super::Matrix2d;

        #[rust_name = "Matrix2d_add"]
        fn op_add(self_: &Matrix2d, other: &Matrix2d) -> Matrix2d;
        #[rust_name = "Matrix2d_sub"]
        fn op_sub(self_: &Matrix2d, other: &Matrix2d) -> Matrix2d;

        
        #[rust_name = "Matrix2d_mul"]
        fn op_mul(self_: &Matrix2d, other: &Matrix2d) -> Matrix2d;
        
        fn setIdentity(self: Pin<&mut Matrix2d>) -> Pin<&mut Matrix2d>;
        type Map_Matrix2d<'a> = super::Map_Matrix2d<'a>;
type Map_Matrix2d_const<'a> = super::Map_Matrix2d_const<'a>;
#[rust_name = "Map_Matrix2d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Matrix2d<'a>;
#[rust_name = "Map_Matrix2d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Matrix2d_const<'a>;

        type Matrix3d = super::Matrix3d;

        #[rust_name = "Matrix3d_add"]
        fn op_add(self_: &Matrix3d, other: &Matrix3d) -> Matrix3d;
        #[rust_name = "Matrix3d_sub"]
        fn op_sub(self_: &Matrix3d, other: &Matrix3d) -> Matrix3d;

        
        #[rust_name = "Matrix3d_mul"]
        fn op_mul(self_: &Matrix3d, other: &Matrix3d) -> Matrix3d;
        
        fn setIdentity(self: Pin<&mut Matrix3d>) -> Pin<&mut Matrix3d>;
        type Map_Matrix3d<'a> = super::Map_Matrix3d<'a>;
type Map_Matrix3d_const<'a> = super::Map_Matrix3d_const<'a>;
#[rust_name = "Map_Matrix3d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Matrix3d<'a>;
#[rust_name = "Map_Matrix3d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Matrix3d_const<'a>;

        type Matrix4d = super::Matrix4d;

        #[rust_name = "Matrix4d_add"]
        fn op_add(self_: &Matrix4d, other: &Matrix4d) -> Matrix4d;
        #[rust_name = "Matrix4d_sub"]
        fn op_sub(self_: &Matrix4d, other: &Matrix4d) -> Matrix4d;

        
        #[rust_name = "Matrix4d_mul"]
        fn op_mul(self_: &Matrix4d, other: &Matrix4d) -> Matrix4d;
        
        fn setIdentity(self: Pin<&mut Matrix4d>) -> Pin<&mut Matrix4d>;
        type Map_Matrix4d<'a> = super::Map_Matrix4d<'a>;
type Map_Matrix4d_const<'a> = super::Map_Matrix4d_const<'a>;
#[rust_name = "Map_Matrix4d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Matrix4d<'a>;
#[rust_name = "Map_Matrix4d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Matrix4d_const<'a>;

        type Matrix2x3d = super::Matrix2x3d;

        #[rust_name = "Matrix2x3d_add"]
        fn op_add(self_: &Matrix2x3d, other: &Matrix2x3d) -> Matrix2x3d;
        #[rust_name = "Matrix2x3d_sub"]
        fn op_sub(self_: &Matrix2x3d, other: &Matrix2x3d) -> Matrix2x3d;

        type Map_Matrix2x3d<'a> = super::Map_Matrix2x3d<'a>;
type Map_Matrix2x3d_const<'a> = super::Map_Matrix2x3d_const<'a>;
#[rust_name = "Map_Matrix2x3d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Matrix2x3d<'a>;
#[rust_name = "Map_Matrix2x3d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Matrix2x3d_const<'a>;

        type MatrixXd = super::MatrixXd;

        #[rust_name = "MatrixXd_add"]
        fn op_add(self_: &MatrixXd, other: &MatrixXd) -> MatrixXd;
        #[rust_name = "MatrixXd_sub"]
        fn op_sub(self_: &MatrixXd, other: &MatrixXd) -> MatrixXd;

        
        #[rust_name = "MatrixXd_mul"]
        fn op_mul(self_: &MatrixXd, other: &MatrixXd) -> MatrixXd;
        
        fn MatrixXd_new(rows: usize, cols: usize) -> MatrixXd;
        fn MatrixXd_clone(v: &MatrixXd) -> MatrixXd;

        type Map_MatrixXd<'a> = super::Map_MatrixXd<'a>;
type Map_MatrixXd_const<'a> = super::Map_MatrixXd_const<'a>;
type Map_MatrixXd_stride<'a> = super::Map_MatrixXd_stride<'a>;
type Map_MatrixXd_const_stride<'a> = super::Map_MatrixXd_const_stride<'a>;
unsafe fn Map_MatrixXd_new<'a>(data: *mut f64, rows: usize, cols: usize) -> Map_MatrixXd<'a>;
unsafe fn Map_MatrixXd_const_new<'a>(data: *const f64, rows: usize, cols: usize) -> Map_MatrixXd_const<'a>;
unsafe fn Map_MatrixXd_stride_new<'a>(data: *mut f64, rows: usize, cols: usize, s0: usize, s1: usize) -> Map_MatrixXd_stride<'a>;
unsafe fn Map_MatrixXd_const_stride_new<'a>(data: *const f64, rows: usize, cols: usize, s0: usize, s1: usize) -> Map_MatrixXd_const_stride<'a>;

        type Vector2d = super::Vector2d;

        #[rust_name = "Vector2d_add"]
        fn op_add(self_: &Vector2d, other: &Vector2d) -> Vector2d;
        #[rust_name = "Vector2d_sub"]
        fn op_sub(self_: &Vector2d, other: &Vector2d) -> Vector2d;

        type Map_Vector2d<'a> = super::Map_Vector2d<'a>;
type Map_Vector2d_const<'a> = super::Map_Vector2d_const<'a>;
#[rust_name = "Map_Vector2d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Vector2d<'a>;
#[rust_name = "Map_Vector2d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Vector2d_const<'a>;

        type Vector3d = super::Vector3d;

        #[rust_name = "Vector3d_add"]
        fn op_add(self_: &Vector3d, other: &Vector3d) -> Vector3d;
        #[rust_name = "Vector3d_sub"]
        fn op_sub(self_: &Vector3d, other: &Vector3d) -> Vector3d;

        type Map_Vector3d<'a> = super::Map_Vector3d<'a>;
type Map_Vector3d_const<'a> = super::Map_Vector3d_const<'a>;
#[rust_name = "Map_Vector3d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Vector3d<'a>;
#[rust_name = "Map_Vector3d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Vector3d_const<'a>;

        type Vector4d = super::Vector4d;

        #[rust_name = "Vector4d_add"]
        fn op_add(self_: &Vector4d, other: &Vector4d) -> Vector4d;
        #[rust_name = "Vector4d_sub"]
        fn op_sub(self_: &Vector4d, other: &Vector4d) -> Vector4d;

        type Map_Vector4d<'a> = super::Map_Vector4d<'a>;
type Map_Vector4d_const<'a> = super::Map_Vector4d_const<'a>;
#[rust_name = "Map_Vector4d_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Vector4d<'a>;
#[rust_name = "Map_Vector4d_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Vector4d_const<'a>;

        type VectorXd = super::VectorXd;

        #[rust_name = "VectorXd_add"]
        fn op_add(self_: &VectorXd, other: &VectorXd) -> VectorXd;
        #[rust_name = "VectorXd_sub"]
        fn op_sub(self_: &VectorXd, other: &VectorXd) -> VectorXd;

        
        fn VectorXd_new(size: usize) -> VectorXd;
        fn VectorXd_clone(v: &VectorXd) -> VectorXd;

        type Map_VectorXd<'a> = super::Map_VectorXd<'a>;
type Map_VectorXd_const<'a> = super::Map_VectorXd_const<'a>;
type Map_VectorXd_stride<'a> = super::Map_VectorXd_stride<'a>;
type Map_VectorXd_const_stride<'a> = super::Map_VectorXd_const_stride<'a>;
unsafe fn Map_VectorXd_new<'a>(data: *mut f64, size: usize) -> Map_VectorXd<'a>;
unsafe fn Map_VectorXd_const_new<'a>(data: *const f64, size: usize) -> Map_VectorXd_const<'a>;

unsafe fn Map_VectorXd_stride_new<'a>(data: *mut f64, size: usize, s: usize) -> Map_VectorXd_stride<'a>;
unsafe fn Map_VectorXd_const_stride_new<'a>(data: *const f64, size: usize, s: usize) -> Map_VectorXd_const_stride<'a>;

         // objs

        type Quaterniond = super::Quaterniond;
        fn normalized(self: &Quaterniond) -> Quaterniond;
        fn normalize(self: Pin<&mut Quaterniond>);
        fn inverse(self: &Quaterniond) -> Quaterniond;
        #[rust_name = "Quaterniond_mul"]
        fn op_mul(self_: &Quaterniond, other: &Quaterniond) -> Quaterniond;
        fn toRotationMatrix(self: &Quaterniond) -> Matrix3d;

        type Map_Quaterniond<'a> = super::Map_Quaterniond<'a>;
type Map_Quaterniond_const<'a> = super::Map_Quaterniond_const<'a>;
#[rust_name = "Map_Quaterniond_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_Quaterniond<'a>;
#[rust_name = "Map_Quaterniond_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_Quaterniond_const<'a>;

        

        type AngleAxisd = super::AngleAxisd;
        fn inverse(self: &AngleAxisd) -> AngleAxisd;
        #[rust_name = "AngleAxisd_mul"]
        fn op_mul(self_: &AngleAxisd, other: &AngleAxisd) -> Quaterniond;
        fn toRotationMatrix(self: &AngleAxisd) -> Matrix3d;

        type Map_AngleAxisd<'a> = super::Map_AngleAxisd<'a>;
type Map_AngleAxisd_const<'a> = super::Map_AngleAxisd_const<'a>;
#[rust_name = "Map_AngleAxisd_new"]
unsafe fn Map_fixed_new<'a>(data: *mut f64) -> Map_AngleAxisd<'a>;
#[rust_name = "Map_AngleAxisd_const_new"]
unsafe fn Map_fixed_const_new<'a>(data: *const f64) -> Map_AngleAxisd_const<'a>;

        

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

impl<'a> Map_Matrix2d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix2d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Matrix2d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix2d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Matrix2d {
    type Output = Matrix2d;

    fn add(self, other: Matrix2d) -> Self::Output {
        ffi::Matrix2d_add(&self, &other)
    }
}

impl Add for &Matrix2d {
    type Output = Matrix2d;
    fn add(self, other: &Matrix2d) -> Self::Output {
        ffi::Matrix2d_add(self, other)
    }
}

impl Sub for Matrix2d {
    type Output = Matrix2d;

    fn sub(self, other: Matrix2d) -> Matrix2d {
        ffi::Matrix2d_sub(&self, &other)
    }
}

impl Sub for &Matrix2d {
    type Output = Matrix2d;
    fn sub(self, other: &Matrix2d) -> Self::Output {
        ffi::Matrix2d_sub(self, other)
    }
}


impl Mul for Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Matrix2d_mul(&self, &other)
    }
}

impl Mul for &Matrix2d {
    type Output = Matrix2d;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Matrix2d_mul(self, other)
    }
}


impl<'a> Map_Matrix3d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix3d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Matrix3d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix3d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Matrix3d {
    type Output = Matrix3d;

    fn add(self, other: Matrix3d) -> Self::Output {
        ffi::Matrix3d_add(&self, &other)
    }
}

impl Add for &Matrix3d {
    type Output = Matrix3d;
    fn add(self, other: &Matrix3d) -> Self::Output {
        ffi::Matrix3d_add(self, other)
    }
}

impl Sub for Matrix3d {
    type Output = Matrix3d;

    fn sub(self, other: Matrix3d) -> Matrix3d {
        ffi::Matrix3d_sub(&self, &other)
    }
}

impl Sub for &Matrix3d {
    type Output = Matrix3d;
    fn sub(self, other: &Matrix3d) -> Self::Output {
        ffi::Matrix3d_sub(self, other)
    }
}


impl Mul for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Matrix3d_mul(&self, &other)
    }
}

impl Mul for &Matrix3d {
    type Output = Matrix3d;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Matrix3d_mul(self, other)
    }
}


impl<'a> Map_Matrix4d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix4d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Matrix4d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix4d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Matrix4d {
    type Output = Matrix4d;

    fn add(self, other: Matrix4d) -> Self::Output {
        ffi::Matrix4d_add(&self, &other)
    }
}

impl Add for &Matrix4d {
    type Output = Matrix4d;
    fn add(self, other: &Matrix4d) -> Self::Output {
        ffi::Matrix4d_add(self, other)
    }
}

impl Sub for Matrix4d {
    type Output = Matrix4d;

    fn sub(self, other: Matrix4d) -> Matrix4d {
        ffi::Matrix4d_sub(&self, &other)
    }
}

impl Sub for &Matrix4d {
    type Output = Matrix4d;
    fn sub(self, other: &Matrix4d) -> Self::Output {
        ffi::Matrix4d_sub(self, other)
    }
}


impl Mul for Matrix4d {
    type Output = Matrix4d;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Matrix4d_mul(&self, &other)
    }
}

impl Mul for &Matrix4d {
    type Output = Matrix4d;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Matrix4d_mul(self, other)
    }
}


impl<'a> Map_Matrix2x3d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix2x3d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Matrix2x3d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Matrix2x3d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Matrix2x3d {
    type Output = Matrix2x3d;

    fn add(self, other: Matrix2x3d) -> Self::Output {
        ffi::Matrix2x3d_add(&self, &other)
    }
}

impl Add for &Matrix2x3d {
    type Output = Matrix2x3d;
    fn add(self, other: &Matrix2x3d) -> Self::Output {
        ffi::Matrix2x3d_add(self, other)
    }
}

impl Sub for Matrix2x3d {
    type Output = Matrix2x3d;

    fn sub(self, other: Matrix2x3d) -> Matrix2x3d {
        ffi::Matrix2x3d_sub(&self, &other)
    }
}

impl Sub for &Matrix2x3d {
    type Output = Matrix2x3d;
    fn sub(self, other: &Matrix2x3d) -> Self::Output {
        ffi::Matrix2x3d_sub(self, other)
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
    pub fn new(data: &'a mut [f64], rows: usize, cols: usize, s0: usize, s1: usize) -> Self {
        unsafe {
            ffi::Map_MatrixXd_stride_new::<'a>(data.as_mut_ptr(), rows, cols, s0, s1)
        }
    }
}

impl<'a> Map_MatrixXd_const_stride<'a> {
    pub fn new(data: &'a [f64], rows: usize, cols: usize, s0: usize, s1: usize) -> Self {
        unsafe {
            ffi::Map_MatrixXd_const_stride_new::<'a>(data.as_ptr(), rows, cols, s0, s1)
        }
    }
}


impl Add for MatrixXd {
    type Output = MatrixXd;

    fn add(self, other: MatrixXd) -> Self::Output {
        ffi::MatrixXd_add(&self, &other)
    }
}

impl Add for &MatrixXd {
    type Output = MatrixXd;
    fn add(self, other: &MatrixXd) -> Self::Output {
        ffi::MatrixXd_add(self, other)
    }
}

impl Sub for MatrixXd {
    type Output = MatrixXd;

    fn sub(self, other: MatrixXd) -> MatrixXd {
        ffi::MatrixXd_sub(&self, &other)
    }
}

impl Sub for &MatrixXd {
    type Output = MatrixXd;
    fn sub(self, other: &MatrixXd) -> Self::Output {
        ffi::MatrixXd_sub(self, other)
    }
}


impl Mul for MatrixXd {
    type Output = MatrixXd;

    fn mul(self, other: Self) -> Self::Output {
        ffi::MatrixXd_mul(&self, &other)
    }
}

impl Mul for &MatrixXd {
    type Output = MatrixXd;

    fn mul(self, other: Self) -> Self::Output {
        ffi::MatrixXd_mul(self, other)
    }
}


impl<'a> Map_Vector2d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Vector2d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Vector2d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Vector2d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Vector2d) -> Self::Output {
        ffi::Vector2d_add(&self, &other)
    }
}

impl Add for &Vector2d {
    type Output = Vector2d;
    fn add(self, other: &Vector2d) -> Self::Output {
        ffi::Vector2d_add(self, other)
    }
}

impl Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Vector2d) -> Vector2d {
        ffi::Vector2d_sub(&self, &other)
    }
}

impl Sub for &Vector2d {
    type Output = Vector2d;
    fn sub(self, other: &Vector2d) -> Self::Output {
        ffi::Vector2d_sub(self, other)
    }
}



impl<'a> Map_Vector3d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Vector3d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Vector3d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Vector3d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, other: Vector3d) -> Self::Output {
        ffi::Vector3d_add(&self, &other)
    }
}

impl Add for &Vector3d {
    type Output = Vector3d;
    fn add(self, other: &Vector3d) -> Self::Output {
        ffi::Vector3d_add(self, other)
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Vector3d {
        ffi::Vector3d_sub(&self, &other)
    }
}

impl Sub for &Vector3d {
    type Output = Vector3d;
    fn sub(self, other: &Vector3d) -> Self::Output {
        ffi::Vector3d_sub(self, other)
    }
}



impl<'a> Map_Vector4d<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Vector4d_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Vector4d_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Vector4d_const_new::<'a>(data.as_ptr())
        }
    }
}


impl Add for Vector4d {
    type Output = Vector4d;

    fn add(self, other: Vector4d) -> Self::Output {
        ffi::Vector4d_add(&self, &other)
    }
}

impl Add for &Vector4d {
    type Output = Vector4d;
    fn add(self, other: &Vector4d) -> Self::Output {
        ffi::Vector4d_add(self, other)
    }
}

impl Sub for Vector4d {
    type Output = Vector4d;

    fn sub(self, other: Vector4d) -> Vector4d {
        ffi::Vector4d_sub(&self, &other)
    }
}

impl Sub for &Vector4d {
    type Output = Vector4d;
    fn sub(self, other: &Vector4d) -> Self::Output {
        ffi::Vector4d_sub(self, other)
    }
}



impl<'a> Map_VectorXd<'a> {
    pub fn new(data: &'a mut [f64], size: usize) -> Self {
        unsafe {
            ffi::Map_VectorXd_new::<'a>(data.as_mut_ptr(), size)
        }
    }
}

impl<'a> Map_VectorXd_const<'a> {
    pub fn new(data: &'a [f64], size: usize) -> Self {
        unsafe {
            ffi::Map_VectorXd_const_new::<'a>(data.as_ptr(), size)
        }
    }
}

impl<'a> Map_VectorXd_stride<'a> {
    pub fn new(data: &'a mut [f64], size: usize, s: usize) -> Self {
        unsafe {
            ffi::Map_VectorXd_stride_new::<'a>(data.as_mut_ptr(), size, s)
        }
    }
}

impl<'a> Map_VectorXd_const_stride<'a> {
    pub fn new(data: &'a [f64], size: usize, s: usize) -> Self {
        unsafe {
            ffi::Map_VectorXd_const_stride_new::<'a>(data.as_ptr(), size, s)
        }
    }
}


impl Add for VectorXd {
    type Output = VectorXd;

    fn add(self, other: VectorXd) -> Self::Output {
        ffi::VectorXd_add(&self, &other)
    }
}

impl Add for &VectorXd {
    type Output = VectorXd;
    fn add(self, other: &VectorXd) -> Self::Output {
        ffi::VectorXd_add(self, other)
    }
}

impl Sub for VectorXd {
    type Output = VectorXd;

    fn sub(self, other: VectorXd) -> VectorXd {
        ffi::VectorXd_sub(&self, &other)
    }
}

impl Sub for &VectorXd {
    type Output = VectorXd;
    fn sub(self, other: &VectorXd) -> Self::Output {
        ffi::VectorXd_sub(self, other)
    }
}



 // objs

impl Mul for Quaterniond {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Quaterniond_mul(&self, &other)
    }
}

impl Mul for &Quaterniond {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::Quaterniond_mul(self, other)
    }
}
impl<'a> Map_Quaterniond<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_Quaterniond_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_Quaterniond_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_Quaterniond_const_new::<'a>(data.as_ptr())
        }
    }
}

impl Mul for AngleAxisd {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::AngleAxisd_mul(&self, &other)
    }
}

impl Mul for &AngleAxisd {
    type Output = Quaterniond;

    fn mul(self, other: Self) -> Self::Output {
        ffi::AngleAxisd_mul(self, other)
    }
}
impl<'a> Map_AngleAxisd<'a> {
    pub fn new(data: &'a mut [f64]) -> Self {
        unsafe {
            ffi::Map_AngleAxisd_new::<'a>(data.as_mut_ptr())
        }
    }
}

impl<'a> Map_AngleAxisd_const<'a> {
    pub fn new(data: &'a [f64]) -> Self {
        unsafe {
            ffi::Map_AngleAxisd_const_new::<'a>(data.as_ptr())
        }
    }
}

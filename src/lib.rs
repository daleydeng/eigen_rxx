#![allow(incomplete_features, non_camel_case_types, non_snake_case)]
#![feature(generic_const_exprs, core_ffi_c, pin_macro)]

mod gen;
pub use gen::ffi;
extern crate nalgebra as na;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_basic() {
        let x = na::Matrix2::new(
            1., 3.,
            2., 4.
        );
        let raw_x: ffi::Matrix<_, 2, 2> = x.into();
        assert_eq!(&raw_x.data, &[1., 2., 3., 4.]);
        let x_rec: na::Matrix2<f64> = raw_x.into();
        assert_eq!(x, x_rec);

        let x = na::Matrix2::new(
            1, 3,
            2, 4
        );
        let raw_x: ffi::Matrix<_, 2, 2> = x.into();
        assert_eq!(&raw_x.data, &[1, 2, 3, 4]);
        let x_rec1: na::Matrix2<i32> = raw_x.into();
        assert_eq!(x, x_rec1);
    }

    #[test]
    fn test_map() {
        let x = na::Matrix2::new(
            1., 3.,
            2., 4.
        );
        let x: na::MatrixSlice2<_> = (&x).into();
        let x_rec = na::MatrixSlice2::from(ffi::Map_Matrix2d::from(x));
        assert_eq!(x.as_slice(), x_rec.as_slice());
    }

    #[test]
    fn test_mut_map() {
        let mut x0 = na::Matrix2::new(
            1., 3.,
            2., 4.
        );

        let x_rec: na::Matrix2<_> = {
            let x: na::MatrixSliceMut2<_> = (&mut x0).into();
            let y: ffi::MapMut_Matrix2d = x.into();
            na::MatrixSliceMut2::from(y).into()
        };

        x0[(0, 0)] = 4.;

        let x_rec1: na::Matrix2<_> = {
            let x: na::MatrixSliceMut2<_> = (&mut x0).into();
            let y: ffi::MapMut_Matrix2d = x.into();
            na::MatrixSliceMut2::from(y).into()
        };

        assert_eq!(x_rec1 - x_rec, na::Matrix2::new(
            3., 0.,
            0., 0.
        ))
    }

    #[test]
    fn test_quat() {
        let q = na::Quaternion::new(1., 2., 3., 4.);
        let x: ffi::Quaterniond = q.into();
        let q_rec: na::Quaternion<_> = x.into();
        assert_eq!(q, q_rec);

        let q1 = na::Quaternion::new(5., 6., 7., 8.);
        let x1: ffi::Quaterniond = q1.into();

        let q_prod = q * q1;
        let x_prod_rec: na::Quaternion<_> = (x * x1).into();
        assert_eq!(q_prod, x_prod_rec)
    }

    #[test]
    fn test_map_quat() {
	let q = na::Quaternion::new(1., 2., 3., 4.);
        let x: ffi::Map_Quaterniond = (&q).into();
        let q_rec: na::Quaternion<_> = x.into();
        assert_eq!(q, q_rec);
    }

    #[test]
    fn test_mut_map_quat() {
	let mut q = na::Quaternion::new(1., 2., 3., 4.);
        let x: ffi::MapMut_Quaterniond = (&mut q).into();
        let q_rec: na::Quaternion<_> = x.into();
        assert_eq!(q, q_rec);
    }

}

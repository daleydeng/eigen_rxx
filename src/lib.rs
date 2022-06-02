mod wrapper;
pub use wrapper::*;

#[cfg(test)]
mod tests {
    use crate::*;
    use core::pin::Pin;
    use std::ptr;

    #[test]
    fn it_works() {
        let v1 = Matrix3d {
            data: [1., 2., 3., 4., 5., 6., 7., 8., 9.],
        };
        println!("p_v1 {:p}", &v1);
        println!("v1: {:?}", &v1);

        let mut v2 = v1 + v1;
        let v3 = v1 * v2;

        println!("p_v2 {:p}", &v2);
        Pin::new(&mut v2).setIdentity();
        println!("v2: {:?}", &v2);
        println!("v3: {:?}", &v3);

        let result = 2 + 2;
        assert_eq!(result, 4);

        let v1 = MatrixXd::new(4, 4);
        unsafe {
            for i in 0..16 {
                ptr::write(v1.data.offset(i), (i * 2) as f64);
            }
        }

        let v2 = &v1 + &v1;
        let v3 = &v1 * &v1;
        println!("v2 {} {} {}", v2.rows, v2.cols, unsafe {
            *v2.data.offset(0)
        });
        println!("v3 {} {} {}", v3.rows, v3.cols, unsafe {
            *v3.data.offset(2)
        });

        let mut v4: [f64; 4] = [1., 2., 3., 4.];
        let v5 = Map_MatrixXd::new(&mut v4, 2, 2);
        println!("v5 {:?}", v5);

        let v6: [f64; 4] = [1., 2., 3., 4.];
        let v7 = Map_MatrixXd_const::new(&v6, 2, 2);
        println!("v7 {:?}", v7);

        let mut v8: [f64; 4] = [1., 2., 3., 4.];
        let v9 = Map_MatrixXd_stride::new(&mut v8, 2, 2, 3, 1);
        println!("v9 {:?}", v9);

        let mut q = Quaterniond {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 4.,
        };
        Pin::new(&mut q).normalize();
        println!("q {:?}", q);

        let q2 = q.normalized();
        let q3 = q2 * q2;
        println!("q3 {:?}", q3);

        let q4 = q3.inverse();
        println!("q4 {:?}", q4);

        let m4 = q4.toRotationMatrix();
        println!("m4 {:?}", m4);
    }
}

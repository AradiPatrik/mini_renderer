use std::ops::{Add, Sub, Mul};
use num::Num;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Scalar<T: Num>(T);

#[derive(Debug, Eq, PartialEq)]
struct Vector2<N: Num> {
    x: Scalar<N>,
    y: Scalar<N>,
}

#[derive(Debug, Eq, PartialEq)]
struct Vector3<N: Num> {
    x: Scalar<N>,
    y: Scalar<N>,
    z: Scalar<N>,
}

impl<N: Num + Copy> Vector3<N> {
    pub fn new(x: N, y: N, z: N) -> Vector3<N> {
        Vector3 { x: Scalar(x), y: Scalar(y), z: Scalar(z) }
    }
}

impl<N: Num + Copy> Vector2<N> {
    pub fn new(x: N, y: N) -> Vector2<N> {
        Vector2 { x: Scalar(x), y: Scalar(y) }
    }
}

macro_rules! impl_ops {
($x:ty) => (
    impl_ops!($x; i64);
    impl_ops!($x; i32);
    impl_ops!($x; i16);
    impl_ops!($x; i8);
    impl_ops!($x; u64);
    impl_ops!($x; u32);
    impl_ops!($x; u16);
    impl_ops!($x; u8);
    impl_ops!($x; f64);
    impl_ops!($x; f32);
);
($x:ty, $($xs:ty),+) => (
    impl_ops!($x);
    impl_ops!($($xs),+);
);
($T:ty; $U:ty) => (
    impl Mul<Scalar<$T>> for Scalar<$U> {
        type Output = Scalar<$U>;
        fn mul(self, rhs: Scalar<$T>) -> Scalar<$U> {
            Scalar(self.0 * rhs.0 as $U)
        }
    }

    impl Add<Scalar<$T>> for Scalar<$U> {
        type Output = Scalar<$U>;
        fn add(self, rhs: Scalar<$T>) -> Scalar<$U> {
            Scalar(self.0 + rhs.0 as $U)
        }
    }

    impl Sub<Scalar<$T>> for Scalar<$U> {
        type Output = Scalar<$U>;
        fn sub(self, rhs: Scalar<$T>) -> Scalar<$U> {
            Scalar(self.0 - rhs.0 as $U)
        }
    }

    impl<'a> Mul<&'a Vector3<$T>> for Scalar<$U> {
        type Output = Vector3<$U>;
        fn mul(self, rhs: &'a Vector3<$T>) -> Vector3<$U> {
            Vector3 {
                x: self * rhs.x,
                y: self * rhs.y,
                z: self * rhs.z,
            }
        }
    }

    impl<'a> Mul<Scalar<$T>> for &'a Vector3<$U> {
        type Output = Vector3<$U>;
        fn mul(self, rhs: Scalar<$T>) -> Vector3<$U> {
            Vector3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs
            }
        }
    }

    impl<'a> Add<&'a Vector3<$T>> for &'a Vector3<$U> {
        type Output = Vector3<$U>;
        fn add(self, rhs: &'a Vector3<$T>) -> Vector3<$U> {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z
            }
        }
    }

    impl<'a> Sub<&'a Vector3<$T>> for &'a Vector3<$U> {
        type Output = Vector3<$U>;
        fn sub(self, rhs: &'a Vector3<$T>) -> Vector3<$U> {
            Vector3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z
            }
        }
    }

    impl<'a> Mul<&'a Vector2<$T>> for Scalar<$U> {
        type Output = Vector2<$U>;
        fn mul(self, rhs: &'a Vector2<$T>) -> Vector2<$U> {
            Vector2 {
                x: self * rhs.x,
                y: self * rhs.y,
            }
        }
    }

    impl<'a> Mul<Scalar<$T>> for &'a Vector2<$U> {
        type Output = Vector2<$U>;
        fn mul(self, rhs: Scalar<$T>) -> Vector2<$U> {
            Vector2 {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl<'a> Add<&'a Vector2<$T>> for &'a Vector2<$U> {
        type Output = Vector2<$U>;
        fn add(self, rhs: &'a Vector2<$T>) -> Vector2<$U> {
            Vector2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<'a> Sub<&'a Vector2<$T>> for &'a Vector2<$U> {
        type Output = Vector2<$U>;
        fn sub(self, rhs: &'a Vector2<$T>) -> Vector2<$U> {
            Vector2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
);
}

impl_ops!(i64, i32, i16, i8, u64, u32, u16, u8, f64, f32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_be_able_to_mul_scalars() {
        assert_eq!(Scalar(6), Scalar(3) * Scalar(2));
    }

    #[test]
    fn should_be_able_to_create_new_vec2() {
        let vec = Vector2::new(3, 4);
        assert_eq!(vec.x.0, 3);
        assert_eq!(vec.y.0, 4);
    }

    #[test]
    fn should_be_able_to_create_floating_point_vec2() {
        let vec = Vector2::new(3.1, 4.3);
        assert_eq!(vec.x.0, 3.1);
        assert_eq!(vec.y.0, 4.3);
    }

    #[test]
    fn should_be_able_to_add_vec2() {
        let x = Vector2::new(5, 5);
        let y = Vector2::new(1, 1);
        let z = &x + &y;
        assert_eq!(z, Vector2::new(6, 6));
    }

    #[test]
    fn should_be_able_to_subtract_vec2() {
        let x = Vector2::new(5, 5);
        let y = Vector2::new(1, 1);
        let z = &x - &y;
        assert_eq!(z, Vector2::new(4, 4));
    }

    // #[test]
    // fn should_be_able_to_dot_two_vec2s() {
    //     let x = Vector2::new(1, 2);
    //     let y = Vector2::new(3, 4);
    //     assert_eq!(11, x.dot(&y));
    //     assert_eq!(x.dot(&y), y.dot(&x));
    // }

    #[test]
    fn should_be_able_to_multiply_vec2_with_scalar() {
        let vector = Vector2::new(3, 4);
        assert_eq!(&vector * Scalar(2), Vector2::new(6, 8));
        assert_eq!(Scalar(2) * &vector, Vector2::new(6, 8));
    }

    #[test]
    fn should_be_able_to_create_new_vec3() {
        let vec = Vector3::new(1, 3, 5);
        assert_eq!(vec.x.0, 1);
        assert_eq!(vec.y.0, 3);
        assert_eq!(vec.z.0, 5);
    }

    #[test]
    fn should_be_able_to_add_vec3s() {
        let vec1 = Vector3::new(1, 3, 5);
        let vec2 = Vector3::new(1, 2, 3);
        assert_eq!(&vec1 + &vec2, Vector3::new(2, 5, 8));
    }

    #[test]
    fn should_be_able_to_subtract_vec3s() {
        let vec1 = Vector3::new(1.0, 3.0, 5.0);
        let vec2 = Vector3::new(1.0, 4.0, 2.5);
        assert_eq!(&vec1 - &vec2, Vector3::new(0.0, -1.0, 2.5));
    }

    // #[test]
    // fn should_be_able_to_dot_two_vec3s() {
    //     let vec1 = Vector3::new(1.0, 2.0, 3.0);
    //     let vec2 = Vector3::new(2.0, 3.0, 5.0);
    //     assert_eq!(23.0, vec1.dot(&vec2));
    //     assert_eq!(vec1.dot(&vec2), vec2.dot(&vec1));
    // }

    #[test]
    fn should_be_able_to_scale_vec3s() {
        let vector1 = Vector3::new(1.0, 3.0, 4.0);
        let vector2 = Vector3::new(1, 3, 4);
        assert_eq!(&vector1 * Scalar(3.0), Vector3::new(3.0, 9.0, 12.0));
        assert_eq!(&vector2 * Scalar(3), Vector3::new(3, 9, 12));
        assert_eq!(&vector1 * Scalar(3.0), Scalar(3.0) * &vector1);
    }

    // #[test]
    // fn should_be_able_to_cross_vec3s() {
    //     let x_unit = Vector3::new(1, 0, 0);
    //     let y_unit = Vector3::new(0, 1, 0);
    //     let z_unit = Vector3::new(0, 0, 1);
    //     assert_eq!(x_unit.cross(&y_unit), z_unit);
    // }

    // #[test]
    // fn test_conversions_vec3() {
    //     let x: Vector3<u8> = Vector3::new(3, 3, 3);
    //     let _a: Vector3<u16> = Vector3::from(&x);
    //     let _b: Vector3<u32> = Vector3::from(&x);
    //     let _c: Vector3<u64> = Vector3::from(&x);
    //     let _d: Vector3<i16> = Vector3::from(&x);
    //     let _e: Vector3<i32> = Vector3::from(&x);
    //     let _f: Vector3<i64> = Vector3::from(&x);
    // }
}

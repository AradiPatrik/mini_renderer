use num::Num;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::convert::From;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T: Num + Copy>(T);

#[derive(Debug, Clone, PartialEq)]
pub struct Vector2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<'a, T: Num + Copy> Mul<Scalar<T>> for &'a Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Scalar<T>) -> Vector2<T> {
        Vector2::new(self.x * rhs.0, self.y * rhs.0)
    }
}

impl<'a, T: Num + Copy> Mul<&'a Vector2<T>> for Scalar<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: &'a Vector2<T>) -> Vector2<T> {
        rhs * self
    }
}

impl<'a, T: Num + Copy> Add<&'a Vector2<T>> for &'a Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, rhs: &'a Vector2<T>) -> Vector2<T> {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub type Vector2i = Vector2<i32>;
pub type Vector2u = Vector2<u32>;
pub type Vector2f = Vector2<f64>;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

impl<'a, T: Num + Copy> Add for &'a Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: &'a Vector3<T>) -> Vector3<T> {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<'a, T: Num + Copy> Sub for &'a Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: &'a Vector3<T>) -> Vector3<T> {
        Vector3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<'a, T: Num + Copy> Mul<Scalar<T>> for &'a Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Scalar<T>) -> Vector3<T> {
        Vector3::new(
            self.x * rhs.0,
            self.y * rhs.0,
            self.z * rhs.0,
        )
    }
}

impl<'a, T: Num + Copy> Mul<&'a Vector3<T>> for Scalar<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: &'a Vector3<T>) -> Vector3<T> {
        rhs * self
    }
}

pub type Vector3i = Vector3<i32>;
pub type Vector3u = Vector3<u32>;
pub type Vector3f = Vector3<f64>;

impl Vector3i {
    pub fn cross(&self, rhs: &Vector3i) -> Vector3i {
        Vector3i {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Vector3f {
    pub fn cross(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Vector3u {
    pub fn cross(&self, rhs: &Vector3u) -> Vector3i {
        Vector3i {
            x: self.y as i32 * rhs.z as i32 - self.z as i32 * rhs.y as i32,
            y: self.z as i32 * rhs.x as i32 - self.x as i32 * rhs.z as i32,
            z: self.x as i32 * rhs.y as i32 - self.y as i32 * rhs.x as i32,
        }
    }
}

impl From<Vector3u> for Vector3i {
    fn from(rhs: Vector3u) -> Vector3i {
        Vector3::new(
            rhs.x as i32,
            rhs.y as i32,
            rhs.z as i32,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_vec2s() -> (Vector2i, Vector2u, Vector2f) {
        (
            Vector2i::new(1, -1),
            Vector2u::new(1, 2),
            Vector2f::new(1.0, 2.0),
        )
    }

    fn init_vec3s() -> (Vector3i, Vector3u, Vector3f) {
        (
            Vector3i::new(1, -1, 1),
            Vector3u::new(1, 2, 3),
            Vector3f::new(1.0, 2.0, 3.0),
        )
    }

    #[test]
    fn should_be_able_to_create_vector_2_of_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec2s();
        assert_eq!(veci, Vector2{ x: 1, y: -1 });
        assert_eq!(vecu, Vector2{ x: 1, y: 2 });
        assert_eq!(vecf, Vector2{ x: 1.0, y: 2.0 });
    }

    #[test]
    fn should_be_able_to_multiply_vec2s_with_scalar_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec2s();
        assert_eq!(&veci * Scalar(3), Vector2::new(3, -3));
        assert_eq!(&vecu * Scalar(3), Vector2::new(3, 6));
        assert_eq!(&vecf * Scalar(3.0), Vector2::new(3.0, 6.0));
    }

    #[test]
    fn order_of_scalar_multiplication_should_not_matter_vec2() {
        let (veci, vecu, vecf) = init_vec2s();
        assert_eq!(&veci * Scalar(3), Scalar(3) * &veci);
        assert_eq!(&vecu * Scalar(3), Scalar(3) * &vecu);
        assert_eq!(&vecf * Scalar(3.0), Scalar(3.0) * &vecf);
    }

    #[test]
    fn should_be_able_to_add_to_vec2s_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec2s();
        assert_eq!(&veci + &Vector2::new(1, 3), Vector2::new(2, 2));
        assert_eq!(&vecu + &Vector2::new(1, 3), Vector2::new(2, 5));
        assert_eq!(&vecf + &Vector2::new(1.0, 3.0), Vector2::new(2.0, 5.0));
    }

    #[test]
    fn should_be_able_to_create_vector_3_of_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec3s();
        assert_eq!(veci, Vector3{ x:1, y:-1, z:1 });
        assert_eq!(vecu, Vector3{ x:1, y:2, z:3 });
        assert_eq!(vecf, Vector3{ x:1.0, y:2.0, z:3.0 });
    }

    #[test]
    fn should_be_able_to_add_to_vector_3s_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec3s();
        assert_eq!(&veci + &veci, Vector3::new(2, -2, 2));
        assert_eq!(&vecu + &vecu, Vector3::new(2, 4, 6));
        assert_eq!(&vecf + &vecf, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn should_be_able_to_substrackt_to_vector_3s_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec3s();
        assert_eq!(&veci - &Vector3::new(1, 2, 3), Vector3::new(0, -3, -2));
        assert_eq!(&vecu - &Vector3::new(1, 2, 3), Vector3::new(0, 0, 0));
        assert_eq!(&vecf - &Vector3::new(1.0, 2.0, 3.0), Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn should_be_able_to_multiply_vec3s_with_scalar_i32_u32_f64() {
        let (veci, vecu, vecf) = init_vec3s();
        assert_eq!(&veci * Scalar(3), Vector3::new(3, -3, 3));
        assert_eq!(&vecu * Scalar(3), Vector3::new(3, 6, 9));
        assert_eq!(&vecf * Scalar(3.0), Vector3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn order_of_scalar_multiplication_should_not_matter_vec3() {
        let (veci, vecu, vecf) = init_vec3s();
        assert_eq!(Scalar(3) * &veci, &veci * Scalar(3));
        assert_eq!(Scalar(3) * &vecu, &vecu * Scalar(3));
        assert_eq!(Scalar(3.0) * &vecf, &vecf * Scalar(3.0));
    }

    #[test]
    fn should_be_able_to_cross_vector_3s_i32() {
        let i = Vector3i::new(1, 0, 0);
        let j = Vector3i::new(0, 1, 0);
        let k = Vector3i::new(0, 0, 1);
        assert_eq!(i.cross(&j), k);
        assert_eq!(i.cross(&j), (Scalar(-1) * &j).cross(&i));
        assert_eq!(i.cross(&(&j + &k)), &i.cross(&j) + &i.cross(&k));
        assert_eq!(i.cross(&i), Vector3::new(0, 0, 0));
    }

    #[test]
    fn should_be_able_to_cross_vector_3s_u32() {
        let i = Vector3u::new(1, 0, 0);
        let j = Vector3u::new(0, 1, 0);
        let k = Vector3u::new(0, 0, 1);
        assert_eq!(i.cross(&j), Vector3i::from(k.clone()));
        assert_eq!(
            i.cross(&j),
            (Scalar(-1) * &Vector3i::from(j.clone())).cross(&Vector3i::from(i.clone()))
        );
        assert_eq!(i.cross(&(&j + &k)), &i.cross(&j) + &i.cross(&k));
        assert_eq!(i.cross(&i), Vector3::new(0, 0, 0));
    }

    #[test]
    fn should_be_able_to_cross_vector_3s_f64() {
        let i = Vector3f::new(1.0, 0.0, 0.0);
        let j = Vector3f::new(0.0, 1.0, 0.0);
        let k = Vector3f::new(0.0, 0.0, 1.0);
        assert_eq!(i.cross(&j), k);
        assert_eq!(i.cross(&j), (Scalar(-1.0) * &j).cross(&i));
        assert_eq!(i.cross(&(&j + &k)), &i.cross(&j) + &i.cross(&k));
        assert_eq!(i.cross(&i), Vector3::new(0.0, 0.0, 0.0));
    }
}

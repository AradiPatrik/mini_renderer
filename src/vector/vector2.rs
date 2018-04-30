use std::ops::{Add, Sub, Mul};
use num::traits::Num;
use num::FromPrimitive;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Scalar<T: Num + Copy>(T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 {x, y}
    }

    pub fn dot(self, rhs: Vector2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Num + Copy> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> <Self as Add<Vector2<T>>>::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num + Copy> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> <Self as Sub<Vector2<T>>>::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Num + Copy> Mul<Scalar<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Scalar<T>) -> Vector2<T> {
        Vector2::new(self.x * rhs.0, self.y * rhs.0)
    }
}

impl<T: Num + Copy> Mul<Vector2<T>> for Scalar<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T> {
        rhs * self
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy + FromPrimitive> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(self, rhs: Vector3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross (self, rhs: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            (self.y * rhs.z - rhs.y * self.z),
            (self.x * rhs.z - rhs.x * self.z) * T::from_i32(-1).unwrap(),
            (self.x * rhs.y - rhs.x * self.y)
        )
    }
}

impl<T: Num + Copy + FromPrimitive> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> <Self as Add<Vector3<T>>>::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Num + Copy + FromPrimitive> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> <Self as Sub<Vector3<T>>>::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Num + Copy + FromPrimitive> Mul<Scalar<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Scalar<T>) -> Vector3<T> {
        Vector3::new(self.x * rhs.0, self.y * rhs.0, self.z * rhs.0)
    }
}

impl<T: Num + Copy + FromPrimitive> Mul<Vector3<T>> for Scalar<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
        rhs * self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_be_able_to_create_new_vec2() {
        let vec = Vector2::new(3, 4);
        assert_eq!(vec.x, 3);
        assert_eq!(vec.y, 4);
    }

    #[test]
    fn should_be_able_to_create_floating_point_vec2() {
        let vec = Vector2::new(3.1, 4.3);
        assert_eq!(vec.x, 3.1);
        assert_eq!(vec.y, 4.3);
    }

    #[test]
    fn should_be_able_to_add_vec2() {
        let x = Vector2::new(5, 5);
        let y = Vector2::new(1, 1);
        let z = x + y;
        assert_eq!(z, Vector2::new(6, 6));
    }

    #[test]
    fn should_be_able_to_substract_vec2() {
        let x = Vector2::new(5, 5);
        let y = Vector2::new(1, 1);
        let z = x - y;
        assert_eq!(z, Vector2::new(4, 4));
    }

    #[test]
    fn should_be_able_to_dot_two_vec2s() {
        let x = Vector2::new(1, 2);
        let y = Vector2::new(3, 4);
        assert_eq!(11, x.dot(y));
        assert_eq!(x.dot(y), y.dot(x));
    }

    #[test]
    fn should_be_able_to_multiply_vec2_with_scalar() {
        let vector = Vector2::new(3, 4);
        assert_eq!(vector * Scalar(2), Vector2::new(6, 8));
        assert_eq!(Scalar(2) * vector, Vector2::new(6, 8));
    }

    #[test]
    fn should_be_able_to_create_new_vec3() {
        let vec = Vector3::new(1, 3, 5);
        assert_eq!(vec.x, 1);
        assert_eq!(vec.y, 3);
        assert_eq!(vec.z, 5);
    }

    #[test]
    fn should_be_able_to_add_vec3s() {
        let vec1 = Vector3::new(1, 3, 5);
        let vec2 = Vector3::new(1, 2, 3);
        assert_eq!(vec1 + vec2, Vector3::new(2, 5, 8));
    }

    #[test]
    fn should_be_able_to_subtract_vec3s() {
        let vec1 = Vector3::new(1.0, 3.0, 5.0);
        let vec2 = Vector3::new(1.0, 4.0, 2.5);
        assert_eq!(vec1 - vec2, Vector3::new(0.0, -1.0, 2.5));
    }

    #[test]
    fn shoudl_be_able_to_dot_two_vec3s() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(2.0, 3.0, 5.0);
        assert_eq!(23.0, vec1.dot(vec2));
        assert_eq!(vec1.dot(vec2), vec2.dot(vec1));
    }

    #[test]
    fn should_be_able_to_scale_vec3s() {
        let vector1 = Vector3::new(1.0, 3.0, 4.0);
        let vector2 = Vector3::new(1, 3, 4);
        assert_eq!(vector1 * Scalar(3.0), Vector3::new(3.0, 9.0, 12.0));
        assert_eq!(vector2 * Scalar(3), Vector3::new(3, 9, 12));
        assert_eq!(vector1 * Scalar(3.0), Scalar(3.0) * vector1);
    }

    #[test]
    fn should_be_able_to_cross_vec3s() {
        let x_unit = Vector3::new(1, 0, 0);
        let y_unit = Vector3::new(0, 1, 0);
        let z_unit = Vector3::new(0, 0, 1);
        assert_eq!(x_unit.cross(y_unit), z_unit);
    }
}
use num::Num;
use std::ops::Add;
use std::ops::Sub;

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

pub type Vector3i = Vector3<i32>;
pub type Vector3u = Vector3<u32>;
pub type Vector3f = Vector3<f64>;

#[cfg(test)]
mod test {
    use super::*;

    fn init_vec3s() -> (Vector3i, Vector3u, Vector3f) {
        (
            Vector3i::new(1, -1, 1),
            Vector3u::new(1, 2, 3),
            Vector3f::new(1.0, 2.0, 3.0),
        )
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
}


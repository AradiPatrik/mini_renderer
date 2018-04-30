use std::ops::{Add, Sub};
use num::traits::Num;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 {x, y}
    }
}

impl<T: Num> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> <Self as Add<Vector2<T>>>::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Num> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> <Self as Sub<Vector2<T>>>::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_be_able_to_add_points() {
        let x = Vector2::new(5, 5);
        let y = Vector2::new(1, 1);
        let z = x + y;
        assert_eq!(z, Vector2::new(6, 6));
    }

    #[test]
    fn should_be_able_to_substract_points() {
        let x = Vector2::new(5, 5);
        let y = Vector2::new(1, 1);
        let z = x - y;
        assert_eq!(z, Vector2::new(4, 4));
    }
}
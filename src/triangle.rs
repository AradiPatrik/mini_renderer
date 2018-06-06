use bounding_box::BoundingBox2;
use cgmath::{Point2, Point3, Vector3};
use cgmath::BaseNum;

pub struct Triangle<S> {
    pub a: Point3<S>,
    pub b: Point3<S>,
    pub c: Point3<S>,
}

impl<S: BaseNum + PartialOrd> Triangle<S> {
    pub fn new(a: Point3<S>, b: Point3<S>, c: Point3<S>) -> Self {
        Triangle { a, b, c }
    }

    pub fn get_bary_coords(&self, p: Point3<S>) -> Point2<f64> {
        // TODO: do something about ugly unwraps
        let ab_vec = &self.b.cast::<f64>().unwrap() - &self.a.cast::<f64>().unwrap();
        let ac_vec = &self.c.cast::<f64>().unwrap() - &self.a.cast::<f64>().unwrap();
        let pa_vec = &self.a.cast::<f64>().unwrap() - p.cast::<f64>().unwrap();
        let x_coords = Vector3::new(ab_vec.x, ac_vec.x, pa_vec.x);
        let y_coords = Vector3::new(ab_vec.y, ac_vec.y, pa_vec.y);
        let cross_product = x_coords.cross(y_coords).cast::<f64>().unwrap();
        Point2::new(
            cross_product.x / cross_product.z,
            cross_product.y / cross_product.z,
        )
    }

    pub fn is_inside_point(&self, p: Point3<S>) -> bool {
        let bary_coords = self.get_bary_coords(p);
        bary_coords.x >= 0.0 && bary_coords.y >= 0.0 && bary_coords.x + bary_coords.y <= 1.0
    }

    pub fn get_bounding_box(&self) -> BoundingBox2<S> {
        BoundingBox2::from_triangle(&self)
    }
}

impl Triangle<u32> {
    pub fn get_z_of_inside_point(&self, p: Point2<u32>) -> u32 {
        let bary_coords = self.get_bary_coords(Point3::new(p.x, p.y, 0));
        (bary_coords.x * self.a.z as f64 + bary_coords.y * self.b.z as f64 + (1.0 - (bary_coords.x + bary_coords.y)) * self.c.z as f64) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::{Point3, Triangle};

    #[test]
    fn should_be_able_to_create_triangle() {
        let triangle = Triangle::new(Point3::new(0, 0, 0), Point3::new(0, 2, 0), Point3::new(2, 0, 0));
        assert_eq!(triangle.a, Point3::new(0, 0, 0));
        assert_eq!(triangle.b, Point3::new(0, 2, 0));
        assert_eq!(triangle.c, Point3::new(2, 0, 0));
    }

    #[test]
    fn test_inside_point() {
        let inside_point = Point3::new(10.0, 3.2, 0.0);
        let outside_point_left = Point3::new(-3.0, 3.2, 0.0);
        let outside_point_right = Point3::new(16.0, 4.0, 0.0);
        let outside_point_down = Point3::new(5.0, -3.0, 0.0);
        let outside_point_up = Point3::new(4.0, 16.0, 0.0);
        let triangle = Triangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(10.0, 10.0, 0.0),
            Point3::new(14.0, 0.0, 0.0),
        );
        assert!(triangle.is_inside_point(inside_point));
        assert!(!triangle.is_inside_point(outside_point_left));
        assert!(!triangle.is_inside_point(outside_point_right));
        assert!(!triangle.is_inside_point(outside_point_down));
        assert!(!triangle.is_inside_point(outside_point_up));
    }

    #[test]
    fn test_get_bounding_box() {
        let triangle = Triangle::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(10.0, 10.0, 0.0),
            Point3::new(14.0, -1.0, 0.0),
        );
        let bounding_box = triangle.get_bounding_box();
        assert_eq!(bounding_box.min_x(), 0.0);
        assert_eq!(bounding_box.min_y(), -1.0);
        assert_eq!(bounding_box.max_x(), 14.0);
        assert_eq!(bounding_box.max_y(), 10.0);
    }
}
    
use cgmath::BaseNum;
use cgmath::{Point2, Vector3};
use bounding_box::BoundingBox2;

pub struct Triangle2<S> {
    pub a: Point2<S>,
    pub b: Point2<S>,
    pub c: Point2<S>,
}

impl<S: BaseNum + PartialOrd> Triangle2<S> {
    pub fn new(a: Point2<S>, b: Point2<S>, c: Point2<S>) -> Self {
        Triangle2 { a, b, c }
    }

    pub fn get_bary_coords(&self, p: Point2<S>) -> Point2<f64> {
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

    pub fn is_inside_point(&self, p: Point2<S>) -> bool {
        let bary_coords = self.get_bary_coords(p);
        bary_coords.x >= 0.0 && bary_coords.y >= 0.0 && bary_coords.x + bary_coords.y <= 1.0
    }

    pub fn get_bounding_box(&self) -> BoundingBox2<S> {
        BoundingBox2::from_triangle2(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Triangle2, Point2};
    
    #[test]
    fn should_be_able_to_create_triangle() {
        let triangle = Triangle2::new(Point2::new(0, 0), Point2::new(0, 2), Point2::new(2, 0));
        assert_eq!(triangle.a, Point2::new(0, 0));
        assert_eq!(triangle.b, Point2::new(0, 2));
        assert_eq!(triangle.c, Point2::new(2, 0));
    }

    #[test]
    fn test_inside_point() {
        let inside_point = Point2::new(10.0, 3.2);
        let outside_point_left = Point2::new(-3.0, 3.2);
        let outside_point_right = Point2::new(16.0, 4.0);
        let outside_point_down = Point2::new(5.0, -3.0);
        let outside_point_up = Point2::new(4.0, 16.0);
        let triangle = Triangle2::new(
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 10.0),
            Point2::new(14.0, 0.0),
        );
        assert!(triangle.is_inside_point(inside_point));
        assert!(!triangle.is_inside_point(outside_point_left));
        assert!(!triangle.is_inside_point(outside_point_right));
        assert!(!triangle.is_inside_point(outside_point_down));
        assert!(!triangle.is_inside_point(outside_point_up));
    }

    #[test]
    fn test_get_bounding_box() {
        let triangle = Triangle2::new(
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 10.0),
            Point2::new(14.0, -1.0),
        );
        let bounding_box = triangle.get_bounding_box();
        assert_eq!(bounding_box.min_x(), 0.0);
        assert_eq!(bounding_box.min_y(), -1.0);
        assert_eq!(bounding_box.max_x(), 14.0);
        assert_eq!(bounding_box.max_y(), 10.0);
    }

}
    
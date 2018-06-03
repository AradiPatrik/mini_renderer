use cgmath::Point2;
use cgmath::BaseNum;
use triangle2::Triangle2;

pub struct BoundingBox2<S> {
    pub lower_left: Point2<S>,
    pub upper_right: Point2<S>,
}

impl<S: BaseNum + PartialOrd> BoundingBox2<S> {
    pub fn from_triangle2(triangle: &Triangle2<S>) -> BoundingBox2<S> {
        use std::cmp::Ordering::Less;
        let x_coordinate_comparator =
            |p: &&&Point2<S>, q: &&&Point2<S>| p.x.partial_cmp(&q.x).unwrap_or(Less);
        let y_coordiante_comparator =
            |p: &&&Point2<S>, q: &&&Point2<S>| p.y.partial_cmp(&q.y).unwrap_or(Less);
        let points = [&triangle.a, &triangle.b, &triangle.c];
        // It is okay to unwrap the results here because we know for a fact, that points is not an empty slice
        let min_x_point = points.iter().min_by(x_coordinate_comparator).unwrap();
        let min_y_point = points.iter().min_by(y_coordiante_comparator).unwrap();
        let max_x_point = points.iter().max_by(x_coordinate_comparator).unwrap();
        let max_y_point = points.iter().max_by(y_coordiante_comparator).unwrap();
        BoundingBox2 {
            lower_left: Point2::new(min_x_point.x, min_y_point.y),
            upper_right: Point2::new(max_x_point.x, max_y_point.y),
        }
    }

    pub fn min_x(&self) -> S {
        self.lower_left.x
    }

    pub fn min_y(&self) -> S {
        self.lower_left.y
    }

    pub fn max_x(&self) -> S {
        self.upper_right.x
    }

    pub fn max_y(&self) -> S {
        self.upper_right.y
    }
}
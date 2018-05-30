extern crate image;
extern crate wavefront_obj;
extern crate num;
extern crate cgmath;
use image::{ImageBuffer, RgbImage, Rgb};
use std::mem;
use wavefront_obj::obj::Vertex;
use cgmath::{Point2, Vector3, BaseNum};

#[derive(Debug, PartialEq)]
pub enum RendererError {
    PixelOutOfImageBounds(u32, u32, Point2<u32>)
}

#[derive(Debug, Clone)]
pub struct Renderer {
    buffer: RgbImage,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Renderer { buffer: ImageBuffer::new(width, height)}
    }

    pub fn from_buffer(buffer: RgbImage) -> Self {
        Renderer { buffer }
    }

    pub fn clear_to_color(&mut self, color: Rgb<u8>) {
        for pixel in self.buffer.pixels_mut() {
            *pixel = color;
        }
    }

    pub fn draw_point(&mut self, point: &Point2<u32>, color: Rgb<u8>) -> Result<(), RendererError> {
        self.check_for_out_of_bounds(point)?;
        self.buffer[(point.x, point.y)] = color;
        Ok(())
    }

    pub fn draw_line(&mut self, start: Point2<u32>, end: Point2<u32>, col: Rgb<u8>) -> Result<(), RendererError> {
        self.check_for_out_of_bounds(&start)?;
        self.check_for_out_of_bounds(&end)?;
        LineDrawer::new(start, end, col, &mut self.buffer).draw_line();
        Ok(())
    }

    pub fn triangle_2d(&mut self, vertex_a: &Vertex, vertex_b: &Vertex, vertex_c: &Vertex, col: Rgb<u8>) -> Result<(), RendererError> {
        let point_a = self.vertex_into_image_space_2d(vertex_a);
        let point_b = self.vertex_into_image_space_2d(vertex_b);
        let point_c = self.vertex_into_image_space_2d(vertex_c);
        self.draw_line(point_a.clone(), point_b.clone(), col)?;
        self.draw_line(point_b, point_c.clone(), col)?;
        self.draw_line(point_c, point_a, col)
    }

    fn vertex_into_image_space_2d(&self, vertex: &Vertex) -> Point2<u32> {
        let mut result = Point2::new(0, 0);
        result.x = ((vertex.x + 1.0) * (self.buffer.width() - 1) as f64 / 2.0) as u32;
        result.y = ((vertex.y + 1.0) * (self.buffer.height() - 1) as f64 / 2.0) as u32;
        result
    }

    pub fn get_buffer_reference(&self) -> &RgbImage {
        &self.buffer
    }

    pub fn unpack(self) -> RgbImage {
        self.buffer
    }

    fn check_for_out_of_bounds(&self, point: &Point2<u32>) -> Result<(), RendererError> {
        let (width, height) = self.buffer.dimensions();
        if point.x >= width || point.y >= height {
            Err(RendererError::PixelOutOfImageBounds(width, height, point.clone()))
        } else {
            Ok(())
        }
    }
}

#[inline]
pub fn lerp(start: u32, end: u32, lerp_amount: f64) -> u32 {
    (start as f64 + (end as i32 - start as i32) as f64 * lerp_amount).round() as u32
}

pub struct Triangle2<S> {
    pub a: Point2<S>,
    pub b: Point2<S>,
    pub c: Point2<S>
}

impl<S: BaseNum + PartialOrd> Triangle2<S> {
    pub fn new(a: Point2<S>, b: Point2<S>, c: Point2<S>) -> Self {
        Triangle2 {a, b, c}
    }

    pub fn get_bary_coords(&self, p: Point2<S>) -> Point2<f64> {
        let ab_vec = &self.b.cast::<f64>().unwrap() - &self.a.cast::<f64>().unwrap();
        let ac_vec = &self.c.cast::<f64>().unwrap() - &self.a.cast::<f64>().unwrap();
        let pa_vec = &self.a.cast::<f64>().unwrap() - p.cast::<f64>().unwrap();
        let x_coords = Vector3::new(ab_vec.x, ac_vec.x, pa_vec.x);
        let y_coords = Vector3::new(ab_vec.y, ac_vec.y, pa_vec.y);
        let cross_product = x_coords.cross(y_coords).cast::<f64>().unwrap();
        Point2::new(cross_product.x / cross_product.z, cross_product.y / cross_product.z)
        
    }

    pub fn is_inside_point(&self, p: Point2<S>) -> bool {
        let bary_coords = self.get_bary_coords(p);
        bary_coords.x >= 0.0 && bary_coords.y >= 0.0 && bary_coords.x + bary_coords.y <= 1.0
    }

    pub fn get_bounding_box<'a>(&self) -> (Point2<S>, Point2<S>) {
        use std::cmp::Ordering::Less;
        let x_coordinate_comparator = |p: &&&Point2<S>, q: &&&Point2<S>| {p.x.partial_cmp(&q.x).unwrap_or(Less)};
        let y_coordiante_comparator = |p: &&&Point2<S>, q: &&&Point2<S>| {p.y.partial_cmp(&q.y).unwrap_or(Less)};
        let points = [&self.a, &self.b, &self.c];
        // It is okay to unwrap the results here because we know for a fact, that points is not an empty slice
        let min_x_point = points.iter().min_by(x_coordinate_comparator).unwrap();
        let min_y_point = points.iter().min_by(y_coordiante_comparator).unwrap();
        let max_x_point = points.iter().max_by(x_coordinate_comparator).unwrap();
        let max_y_point = points.iter().max_by(y_coordiante_comparator).unwrap();
        (
            Point2::new(min_x_point.x, min_y_point.y),
            Point2::new(max_x_point.x, max_y_point.y)
        )
    }
}

struct LineDrawer<'a> {
    start: Point2<u32>,
    end: Point2<u32>,
    col: Rgb<u8>,
    is_steep: bool,
    buffer: &'a mut RgbImage,
}

impl<'a> LineDrawer<'a> {
    pub fn new(start: Point2<u32>, end: Point2<u32>, col: Rgb<u8>, buffer: &'a mut RgbImage) -> Self {
        let mut drawer = LineDrawer::create_initial_instance(start, end, col, buffer);
        drawer.make_line_shallow();
        drawer.order_points();
        drawer
    }

    fn create_initial_instance(start: Point2<u32>, end: Point2<u32>, col: Rgb<u8>, buffer: &'a mut RgbImage) -> Self {
        LineDrawer {
            start,
            end,
            col,
            is_steep: false,
            buffer,
        }
    }

    fn make_line_shallow(&mut self) {
        let x_distance = (self.start.x as i32 - self.end.x as i32).abs() as u32;
        let y_distance = (self.start.y as i32 - self.end.y as i32).abs() as u32;
        if y_distance > x_distance {
            mem::swap(&mut self.start.x, &mut self.end.y);
            mem::swap(&mut self.start.y, &mut self.end.x);
            self.is_steep = true;
        }
    }

    fn order_points(&mut self) {
        if self.start.x > self.end.x {
            mem::swap(&mut self.start, &mut self.end);
        }
    }

    pub fn draw_line(&mut self) {
        for x in self.start.x..self.end.x + 1 {
            self.fill_next_line_point(x);
        }
    }

    fn fill_next_line_point(&mut self, x: u32) {
        let base_offset = lerp(self.start.y, self.end.y, (x - self.start.x) as f64 / (self.end.x - self.start.x) as f64);
        if self.is_steep {
            self.buffer[(base_offset, x)] = self.col;
        } else {
            self.buffer[(x, base_offset)] = self.col;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wavefront_obj::obj::Vertex;

    #[test]
    fn should_be_able_to_create_renderer_from_dimensions() {
        let _renderer = Renderer::new(10, 10);
    }

    #[test]
    fn should_be_able_to_create_renderer_from_buffer() {
        let image: RgbImage = ImageBuffer::new(300, 300);
        let _renderer = Renderer::from_buffer(image);
    }

    #[test]
    fn draw_a_zero_length_line_should_draw_a_dot() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.draw_line(Point2::new(0, 0), Point2::new(0, 0), Rgb([1, 1, 1])).is_ok());
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    #[test]
    fn draw_even_line() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.draw_line(Point2::new(0, 0), Point2::new(1, 1), Rgb([1, 1, 1])).is_ok());
        renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    }

    #[test]
    fn parameter_order_should_not_matter() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.draw_line(Point2::new(1, 1), Point2::new(0, 0), Rgb([1, 1, 1])).is_ok());
        renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    }

    #[test]
    fn over_indexing_should_result_in_error() {
        let mut renderer = Renderer::new(2, 2);
        let result = renderer.draw_line(Point2::new(2, 2), Point2::new(2, 2), Rgb([1, 1, 1]));
        assert_eq!(result, Err(RendererError::PixelOutOfImageBounds(2, 2, Point2::new(2, 2))));
    }

    #[test]
    fn should_be_able_to_draw_shallow_line() {
        let mut renderer = Renderer::new(2, 2);
        let draw_result = renderer.draw_line(Point2::new(0, 0), Point2::new(1, 0), Rgb([1,1,1]));
        assert!(draw_result.is_ok());
        renderer_should_have_drawn_flat_line(&renderer);
    }

    #[test]
    fn should_be_able_to_draw_steep_line() {
        let mut renderer = Renderer::new(2, 2);
        let draw_result = renderer.draw_line(Point2::new(0, 0), Point2::new(0, 1), Rgb([1,1,1]));
        assert!(draw_result.is_ok());
        renderer_should_have_drawn_straight_vertical_line(&renderer);
    }

    #[test]
    fn should_be_able_to_clear_with_renderer() {
        let mut renderer = Renderer::new(2, 2);
        renderer.clear_to_color(Rgb([5,5,5]));
        assert_eq!(renderer.buffer[(0, 0)], Rgb([5, 5, 5]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([5, 5, 5]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([5, 5, 5]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([5, 5, 5]));
    }

    #[test]
    fn should_be_able_to_get_reference_to_data() {
        let renderer = Renderer::new(2, 2);
        let buffer_ref = renderer.get_buffer_reference();
        for pixel_ref in buffer_ref.pixels() {
            assert_eq!(Rgb([0, 0, 0]), *pixel_ref);
        }
        assert_eq!(2, buffer_ref.height());
        assert_eq!(2, buffer_ref.width());
    }

    #[test]
    fn should_be_able_to_unpack_renderer() {
        let renderer = Renderer::new(2, 2);
        let buffer = renderer.unpack();
        for pixel_ref in buffer.pixels() {
            assert_eq!(Rgb([0, 0, 0]), *pixel_ref);
        }
        assert_eq!(2, buffer.height());
        assert_eq!(2, buffer.width());
    }

    #[test]
    fn should_be_able_to_draw_triangle() {
        let mut renderer = Renderer::new(3, 3);
        let vertex_a = Vertex{ x: 0.0, y: 1.0, z: 0.0 };
        let vertex_b = Vertex{ x: 1.0, y: -1.0, z: 0.0};
        let vertex_c = Vertex{ x: -1.0, y: -1.0, z: 0.0};
        let result = renderer.triangle_2d(&vertex_a, &vertex_b, &vertex_c, Rgb([1, 1, 1]));
        assert_eq!(Ok(()), result);
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 2)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(2, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([1, 1, 1]));
    }

    #[test]
    fn should_be_able_to_draw_point() {
        let mut renderer = Renderer::new(2, 2);
        let result = renderer.draw_point(&Point2::new(1, 1), Rgb([1, 1, 1]));
        assert_eq!(Ok(()), result);
        assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
    }

    #[test]
    fn over_indexing_point_should_return_error() {
        let mut renderer = Renderer::new(2, 2);
        let result = renderer.draw_point(&Point2::new(2, 2), Rgb([1, 1, 1]));
        assert_eq!(result, Err(RendererError::PixelOutOfImageBounds(2, 2, Point2::new(2, 2))));
    }

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
        let triangle = Triangle2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0), Point2::new(14.0, 0.0));
        assert!(triangle.is_inside_point(inside_point));
        assert!(!triangle.is_inside_point(outside_point_left));
        assert!(!triangle.is_inside_point(outside_point_right));
        assert!(!triangle.is_inside_point(outside_point_down));
        assert!(!triangle.is_inside_point(outside_point_up));
    }

    #[test]
    fn test_get_bounding_box() {
        let triangle = Triangle2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 10.0), Point2::new(14.0, -1.0));
        assert_eq!(triangle.get_bounding_box(), (Point2::new(0.0, -1.0), Point2::new(14.0, 10.0)));
    }

    #[test]
    fn should_be_able_to_draw_triangle_from_vertices() {
        
    }

    fn renderer_should_have_drawn_line_from_bottom_left_to_top_right(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    fn renderer_should_have_drawn_flat_line(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
    }

    fn renderer_should_have_drawn_straight_vertical_line(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    }
}

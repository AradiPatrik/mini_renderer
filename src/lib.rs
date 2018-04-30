extern crate image;
extern crate wavefront_obj;
extern crate num;
use image::{ImageBuffer, RgbImage, Rgb};
use std::mem;
use wavefront_obj::obj::Vertex;
pub mod vector;
use vector::vector2::Vector2;

#[derive(Debug, PartialEq)]
pub enum RendererError {
    PixelOutOfImageBounds(u32, u32, Vector2<u32>)
}

pub struct Renderer {
    buffer: RgbImage
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

    pub fn line(&mut self, start: Vector2<u32>, end: Vector2<u32>, col: Rgb<u8>) -> Result<(), RendererError> {
        if let Err(error) = self.check_for_out_of_bounds(&start, &end) {
            return Err(error);
        }
        LineDrawer::new(start, end, col, &mut self.buffer).draw_line();
        Ok(())
    }

    pub fn triangle_2d(&mut self, vertex_a: &Vertex, vertex_b: &Vertex, vertex_c: &Vertex, col: Rgb<u8>) -> Result<(), RendererError> {
        let point_a = self.vertex_into_image_space_2d(vertex_a);
        let point_b = self.vertex_into_image_space_2d(vertex_b);
        let point_c = self.vertex_into_image_space_2d(vertex_c);
        self.line(point_a.clone(), point_b.clone(), col)?;
        self.line(point_b, point_c.clone(), col)?;
        self.line(point_c, point_a, col)
    }

    fn vertex_into_image_space_2d(&self, vertex: &Vertex) -> Vector2<u32> {
        let mut result = Vector2::new(0, 0);
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

    fn check_for_out_of_bounds(&self, start: &Vector2<u32>, end: &Vector2<u32>) -> Result<(), RendererError> {
        let (width, height) = self.buffer.dimensions();
        if start.x >= width || start.y >= height {
            Err(RendererError::PixelOutOfImageBounds(width, height, start.clone()))
        } else if end.x >= width || end.y >= height {
            Err(RendererError::PixelOutOfImageBounds(width, height, end.clone()))
        } else {
            Ok(())
        }
    }
}

#[inline]
pub fn lerp(start: u32, end: u32, lerp_amount: f64) -> u32 {
    (start as f64 + (end as i32 - start as i32) as f64 * lerp_amount).round() as u32
}

struct LineDrawer<'a> {
    start: Vector2<u32>,
    end: Vector2<u32>,
    col: Rgb<u8>,
    is_steep: bool,
    buffer: &'a mut RgbImage,
}

impl<'a> LineDrawer<'a> {
    pub fn new(start: Vector2<u32>, end: Vector2<u32>, col: Rgb<u8>, buffer: &'a mut RgbImage) -> Self {
        let mut drawer = LineDrawer::create_initial_instance(start, end, col, buffer);
        drawer.invert_line();
        drawer.switch_points();
        drawer
    }

    fn create_initial_instance(start: Vector2<u32>, end: Vector2<u32>, col: Rgb<u8>, buffer: &'a mut RgbImage) -> Self {
        LineDrawer {
            start,
            end,
            col,
            is_steep: false,
            buffer,
        }
    }

    fn invert_line(&mut self) {
        let x_distance = (self.start.x as i32 - self.end.x as i32).abs() as u32;
        let y_distance = (self.start.y as i32 - self.end.y as i32).abs() as u32;
        if y_distance > x_distance {
            mem::swap(&mut self.start.x, &mut self.end.y);
            mem::swap(&mut self.start.y, &mut self.end.x);
            self.is_steep = true;
        }
    }

    fn switch_points(&mut self) {
        if self.start.x > self.end.x {
            mem::swap(&mut self.start, &mut self.end);
        }
    }

    pub fn draw_line(&mut self) {
        for x in self.start.x..self.end.x + 1 {
            self.fill_point(x);
        }
    }

    fn fill_point(&mut self, x: u32) {
        if self.is_steep {
            self.buffer[(lerp(self.start.y, self.end.y, (x - self.start.x) as f64 / (self.end.x - self.start.x) as f64 ), x)] = self.col;
        } else {
            self.buffer[(x, lerp(self.start.y, self.end.y, (x - self.start.x) as f64 / (self.end.x - self.start.x) as f64 ))] = self.col;
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
    fn should_be_able_to_create_a_point() {
        let a = Vector2::new(2, 3);
        assert_eq!(a.x, 2);
        assert_eq!(a.y, 3);
    }

    #[test]
    fn draw_a_zero_length_line_should_draw_a_dot() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.line(Vector2::new(0, 0), Vector2::new(0, 0), Rgb([1, 1, 1])).is_ok());
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    #[test]
    fn draw_even_line() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.line(Vector2::new(0, 0), Vector2::new(1, 1), Rgb([1, 1, 1])).is_ok());
        renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    }

    #[test]
    fn parameter_order_should_not_matter() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.line(Vector2::new(1, 1), Vector2::new(0, 0), Rgb([1, 1, 1])).is_ok());
        renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    }

    #[test]
    fn over_indexing_should_result_in_error() {
        let mut renderer = Renderer::new(2, 2);
        let result = renderer.line(Vector2::new(2, 2), Vector2::new(2, 2), Rgb([1, 1, 1]));
        assert_eq!(result, Err(RendererError::PixelOutOfImageBounds(2, 2, Vector2::new(2, 2))));
    }

    #[test]
    fn should_be_able_to_draw_shallow_line() {
        let mut renderer = Renderer::new(2, 2);
        let draw_result = renderer.line(Vector2::new(0, 0), Vector2::new(1, 0), Rgb([1,1,1]));
        assert!(draw_result.is_ok());
        renderer_should_have_drawn_flat_line(&renderer);
    }

    #[test]
    fn should_be_able_to_draw_steep_line() {
        let mut renderer = Renderer::new(2, 2);
        let draw_result = renderer.line(Vector2::new(0, 0), Vector2::new(0, 1), Rgb([1,1,1]));
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
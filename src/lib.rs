extern crate image;
extern crate wavefront_obj;
use image::{ImageBuffer, RgbImage, Rgb};
use std::mem;
use wavefront_obj::obj::Vertex;

#[derive(Debug, PartialEq)]
pub enum RendererError {
    PixelOutOfImageBounds(u32, u32, Point)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
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

    pub fn line(&mut self, mut start: Point, mut end: Point, col: Rgb<u8>) -> Result<(), RendererError> {
        if let Err(error) = self.check_for_out_of_bounds(&start, &end) {
            return Err(error);
        }
        let x_distance = (start.x as i32 - end.x as i32).abs() as u32;
        let y_distance = (start.y as i32 - end.y as i32).abs() as u32;
        let mut is_steep = false;
        if y_distance > x_distance {
            mem::swap(&mut start.x, &mut end.y);
            mem::swap(&mut start.y, &mut end.x);
            is_steep = true;
        }
        if start.x > end.x {
            mem::swap(&mut start, &mut end);
        }
        for x in start.x..end.x + 1 {
            if is_steep {
                self.buffer[(lerp(start.y, end.y, (x - start.x) as f64 / (end.x - start.x) as f64 ), x)] = col;
            } else {
                self.buffer[(x, lerp(start.y, end.y, (x - start.x) as f64 / (end.x - start.x) as f64 ))] = col;
            }
        }
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

    fn vertex_into_image_space_2d(&self, vertex: &Vertex) -> Point {
        let mut result = Point::new(0, 0);
        result.x = ((vertex.x + 1.0) * self.buffer.width() as f64 / 2.0) as u32;
        result.y = ((vertex.y + 1.0) * self.buffer.height() as f64 / 2.0) as u32;
        result
    }

    pub fn get_buffer_reference(&self) -> &RgbImage {
        &self.buffer
    }

    pub fn unpack(self) -> RgbImage {
        self.buffer
    }

    fn check_for_out_of_bounds(&self, start: &Point, end: &Point) -> Result<(), RendererError> {
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
fn lerp(start: u32, end: u32, lerp_amount: f64) -> u32 {
    (start as f64 + (end as i32 - start as i32) as f64 * lerp_amount).round() as u32
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
        let a = Point::new(2, 3);
        assert_eq!(a.x, 2);
        assert_eq!(a.y, 3);
    }

    #[test]
    fn draw_a_zero_length_line_should_draw_a_dot() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.line(Point::new(0, 0), Point::new(0, 0), Rgb([1, 1, 1])).is_ok());
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    #[test]
    fn draw_even_line() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.line(Point::new(0, 0), Point::new(1, 1), Rgb([1, 1, 1])).is_ok());
        renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    }

    #[test]
    fn parameter_order_should_not_matter() {
        let mut renderer = Renderer::new(2, 2);
        assert!(renderer.line(Point::new(1, 1), Point::new(0, 0), Rgb([1, 1, 1])).is_ok());
        renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    }

    #[test]
    fn over_indexing_should_result_in_error() {
        let mut renderer = Renderer::new(2, 2);
        let result = renderer.line(Point::new(2, 2), Point::new(2, 2), Rgb([1, 1, 1]));
        assert_eq!(result, Err(RendererError::PixelOutOfImageBounds(2, 2, Point::new(2, 2))));
    }

    #[test]
    fn should_be_able_to_draw_shallow_line() {
        let mut renderer = Renderer::new(2, 2);
        let draw_result = renderer.line(Point::new(0, 0), Point::new(1, 0), Rgb([1,1,1]));
        assert!(draw_result.is_ok());
        renderer_should_have_drawn_flat_line(&renderer);
    }

    #[test]
    fn should_be_able_to_draw_steep_line() {
        let mut renderer = Renderer::new(2, 2);
        let draw_result = renderer.line(Point::new(0, 0), Point::new(0, 1), Rgb([1,1,1]));
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
        assert_eq!(renderer.buffer[(2, 1)], Rgb([0, 0, 0]));
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
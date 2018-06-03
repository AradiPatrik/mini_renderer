use image::RgbImage;
use image::Rgb;
use image::ImageBuffer;
use renderer_error::RendererError;
use wavefront_obj::obj::Vertex;
use draw_mode::DrawMode;
use triangle_drawer::TriangleDrawer;

#[derive(Debug, Clone)]
pub struct Renderer {
    buffer: RgbImage,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Renderer {
            buffer: ImageBuffer::new(width, height),
        }
    }

    pub fn from_buffer(buffer: RgbImage) -> Self {
        Renderer { buffer }
    }

    pub fn clear_to_color(&mut self, color: Rgb<u8>) {
        for pixel in self.buffer.pixels_mut() {
            *pixel = color;
        }
    }

    pub fn draw_triangle_2d(
        &mut self,
        vertex_a: &Vertex,
        vertex_b: &Vertex,
        vertex_c: &Vertex,
        col: Rgb<u8>,
    ) -> Result<(), RendererError> {
        let mut drawer =
            TriangleDrawer::from_vertices(vertex_a, vertex_b, vertex_c, &mut self.buffer)?;
        drawer.draw(DrawMode::Wireframe, col);
        Ok(())
    }

    pub fn draw_filled_triangle_2d(
        &mut self,
        vertex_a: &Vertex,
        vertex_b: &Vertex,
        vertex_c: &Vertex,
        col: Rgb<u8>,
    ) -> Result<(), RendererError> {
        let mut drawer =
            TriangleDrawer::from_vertices(vertex_a, vertex_b, vertex_c, &mut self.buffer)?;
        drawer.draw(DrawMode::Normal, col);
        Ok(())
    }

    pub fn get_buffer_reference(&self) -> &RgbImage {
        &self.buffer
    }

    pub fn unpack(self) -> RgbImage {
        self.buffer
    }
}

#[cfg(test)]
mod test {
    use super::{Rgb, Vertex, ImageBuffer, Renderer, RgbImage};

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
    fn should_be_able_to_clear_with_renderer() {
        let mut renderer = Renderer::new(2, 2);
        renderer.clear_to_color(Rgb([5, 5, 5]));
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
        let vertex_a = Vertex {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let vertex_b = Vertex {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        };
        let vertex_c = Vertex {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        };
        let result = renderer.draw_triangle_2d(&vertex_a, &vertex_b, &vertex_c, Rgb([1, 1, 1]));
        assert_eq!(Ok(()), result);
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 2)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(2, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([1, 1, 1]));
    }

    #[test]
    fn should_be_able_to_draw_filled_triangle_from_vertices() {
        let mut renderer = Renderer::new(4, 4);
        let bottom_left = Vertex {
            x: -1.0,
            y: -1.0,
            z: 0.0,
        };
        let tor_right = Vertex {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };
        let bottom_right = Vertex {
            x: 1.0,
            y: -1.0,
            z: 0.0,
        };
        let result = renderer.draw_filled_triangle_2d(
            &bottom_left,
            &tor_right,
            &bottom_right,
            Rgb([1, 1, 1]),
        );
        assert_eq!(Ok(()), result);
        bottom_floor_should_be_filled(&renderer);
        right_wall_should_be_filled(&renderer);
        slope_should_be_filled(&renderer);
        middle_point_should_be_filled(&renderer);
    }

    

    fn bottom_floor_should_be_filled(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(2, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(3, 1)], Rgb([1, 1, 1]));
    }

    fn right_wall_should_be_filled(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(3, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(3, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(3, 2)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(3, 3)], Rgb([1, 1, 1]));
    }

    fn slope_should_be_filled(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(2, 2)], Rgb([1, 1, 1]));
    }

    fn middle_point_should_be_filled(renderer: &Renderer) {
        assert_eq!(renderer.buffer[(2, 1)], Rgb([1, 1, 1]));
    }
}
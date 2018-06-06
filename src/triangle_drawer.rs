use cgmath::{Point2, Point3};
use draw_mode::DrawMode;
use image::{Rgb, RgbImage};
use line_drawer::LineDrawer;
use renderer_error::RendererError;
use triangle::Triangle;
use vertex_coordinate_mapper::VertexCoordinateMapper;
use wavefront_obj::obj::Vertex;
use z_buffer::ZBuffer;
use z_buffer::PixelVisibility;

pub struct TriangleDrawer<'a> {
    triangle: Triangle<u32>,
    buffer: &'a mut RgbImage,
    z_buffer: &'a mut ZBuffer,
}

impl<'a> TriangleDrawer<'a> {
    pub fn from_vertices(
        a: &Vertex,
        b: &Vertex,
        c: &Vertex,
        buffer: &'a mut RgbImage,
        z_buffer: &'a mut ZBuffer,
    ) -> Result<Self, RendererError> {
        let mapper = VertexCoordinateMapper::new(buffer.width(), buffer.height());
        Ok(TriangleDrawer {
            triangle: Triangle::new(
                mapper.map_vertex_coords_to_pixel_coords(a)?,
                mapper.map_vertex_coords_to_pixel_coords(b)?,
                mapper.map_vertex_coords_to_pixel_coords(c)?,
            ),
            buffer,
            z_buffer,
        })
    }

    pub fn draw(&mut self, draw_mode: DrawMode, col: Rgb<u8>) {
        match draw_mode {
            DrawMode::Normal => {
                self.draw_outline(col);
                self.fill_triangle(col);
            }
            DrawMode::Wireframe => {
                self.draw_outline(col);
            }
        }
    }

    fn draw_outline(&mut self, col: Rgb<u8>) {
        LineDrawer::new(
            self.triangle.a.clone(),
            self.triangle.b.clone(),
            col,
            &mut self.buffer,
            &mut self.z_buffer,
        ).draw_line();
        LineDrawer::new(
            self.triangle.b.clone(),
            self.triangle.c.clone(),
            col,
            &mut self.buffer,
            &mut self.z_buffer,
        ).draw_line();
        LineDrawer::new(
            self.triangle.c.clone(),
            self.triangle.a.clone(),
            col,
            &mut self.buffer,
            &mut self.z_buffer,
        ).draw_line();
    }

    fn fill_triangle(&mut self, col: Rgb<u8>) {
        let bounding_box = self.triangle.get_bounding_box();
        for x in bounding_box.min_x()..=bounding_box.max_x() {
            for y in bounding_box.min_y()..=bounding_box.max_y() {
                if self.triangle.is_inside_point(Point3::new(x, y, 0).clone()) {
                    match self.z_buffer.update_buffer(x, y, self.triangle.get_z_of_inside_point(Point2::new(x, y))).unwrap() {
                        PixelVisibility::Visible => {self.buffer[(x, y)] = col;},
                        PixelVisibility::Hidden => {},
                    }
                }
            }
        }
    }
}
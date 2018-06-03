use image::{Rgb, RgbImage};
use draw_mode::DrawMode;
use renderer_error::RendererError;
use wavefront_obj::obj::Vertex;
use triangle2::Triangle2;
use cgmath::Point2;
use vertex_coordinate_mapper::VertexCoordinateMapper;
use line_drawer::LineDrawer;

pub struct TriangleDrawer<'a> {
    triangle: Triangle2<u32>,
    buffer: &'a mut RgbImage,
}

impl<'a> TriangleDrawer<'a> {
    pub fn from_vertices(
        a: &Vertex,
        b: &Vertex,
        c: &Vertex,
        buffer: &'a mut RgbImage,
    ) -> Result<Self, RendererError> {
        let mapper = VertexCoordinateMapper::new(buffer.width(), buffer.height());
        Ok(TriangleDrawer {
            triangle: Triangle2::new(
                mapper.map_vertex_coords_to_pixel_coord(a)?,
                mapper.map_vertex_coords_to_pixel_coord(b)?,
                mapper.map_vertex_coords_to_pixel_coord(c)?,
            ),
            buffer,
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
        ).draw_line();
        LineDrawer::new(
            self.triangle.b.clone(),
            self.triangle.c.clone(),
            col,
            &mut self.buffer,
        ).draw_line();
        LineDrawer::new(
            self.triangle.c.clone(),
            self.triangle.a.clone(),
            col,
            &mut self.buffer,
        ).draw_line();
    }

    fn fill_triangle(&mut self, col: Rgb<u8>) {
        let bounding_box = self.triangle.get_bounding_box();
        for x in bounding_box.min_x()..=bounding_box.max_x() {
            for y in bounding_box.min_y()..=bounding_box.max_y() {
                if self.triangle.is_inside_point(Point2::new(x, y).clone()) {
                    self.buffer[(x, y)] = col;
                }
            }
        }
    }
}
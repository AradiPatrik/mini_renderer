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
    PixelOutOfImageBounds(u32, u32, Point2<u32>),
    NotInNormalizedDeviceCoords(Vertex)
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

    pub fn draw_triangle_2d(&mut self, vertex_a: &Vertex, vertex_b: &Vertex, vertex_c: &Vertex, col: Rgb<u8>) -> Result<(), RendererError> {
        let mut drawer = TriangleDrawer::from_vertices(vertex_a, vertex_b, vertex_c, &mut self.buffer)?;
        drawer.draw(DrawMode::Wireframe, col);
        Ok(())
    }

    pub fn draw_filled_triangle_2d(&mut self, vertex_a: &Vertex, vertex_b: &Vertex, vertex_c: &Vertex, col: Rgb<u8>) -> Result<(), RendererError> {
        let mut drawer = TriangleDrawer::from_vertices(vertex_a, vertex_b, vertex_c, &mut self.buffer)?;
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

#[inline]
pub fn lerp(start: u32, end: u32, lerp_amount: f64) -> u32 {
    (start as f64 + (end as i32 - start as i32) as f64 * lerp_amount).round() as u32
}

pub struct BoundingBox2<S> {
    pub lower_left: Point2<S>,
    pub upper_right: Point2<S>
}

impl<S: BaseNum + PartialOrd> BoundingBox2<S> {
    pub fn from_triangle2(triangle: &Triangle2<S>) -> BoundingBox2<S> {
        use std::cmp::Ordering::Less;
        let x_coordinate_comparator = |p: &&&Point2<S>, q: &&&Point2<S>| {p.x.partial_cmp(&q.x).unwrap_or(Less)};
        let y_coordiante_comparator = |p: &&&Point2<S>, q: &&&Point2<S>| {p.y.partial_cmp(&q.y).unwrap_or(Less)};
        let points = [&triangle.a, &triangle.b, &triangle.c];
        // It is okay to unwrap the results here because we know for a fact, that points is not an empty slice
        let min_x_point = points.iter().min_by(x_coordinate_comparator).unwrap();
        let min_y_point = points.iter().min_by(y_coordiante_comparator).unwrap();
        let max_x_point = points.iter().max_by(x_coordinate_comparator).unwrap();
        let max_y_point = points.iter().max_by(y_coordiante_comparator).unwrap();
        BoundingBox2 {
            lower_left: Point2::new(min_x_point.x, min_y_point.y),
            upper_right: Point2::new(max_x_point.x, max_y_point.y)
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
        // TODO: do something about ugly unwraps
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

    pub fn get_bounding_box(&self) -> BoundingBox2<S> {
        BoundingBox2::from_triangle2(&self)
    }
}

enum DrawMode {
    Normal,
    Wireframe
}

struct TriangleDrawer<'a> {
    triangle: Triangle2<u32>,
    buffer: &'a mut RgbImage
}

impl<'a> TriangleDrawer<'a> {
    pub fn from_vertices(a: &Vertex, b: &Vertex, c: &Vertex, buffer: &'a mut RgbImage) -> Result<Self, RendererError> {
        let mapper = VertexCoordinateMapper::new(buffer.width(), buffer.height());
        Ok (
            TriangleDrawer {
                triangle: Triangle2::new (
                    mapper.map_vertex_coords_to_pixel_coord(a)?,
                    mapper.map_vertex_coords_to_pixel_coord(b)?,
                    mapper.map_vertex_coords_to_pixel_coord(c)?
                ),
                buffer
            }
        )
    }

    pub fn draw(&mut self, draw_mode: DrawMode, col: Rgb<u8>) {
        match draw_mode {
            DrawMode::Normal => {
                self.draw_outline(col);
                self.fill_triangle(col);
            },
            DrawMode::Wireframe => {
                self.draw_outline(col);
            }
        }
    }

    fn draw_outline(&mut self, col: Rgb<u8>) {
        LineDrawer::new(self.triangle.a.clone(), self.triangle.b.clone(), col, &mut self.buffer).draw_line();
        LineDrawer::new(self.triangle.b.clone(), self.triangle.c.clone(), col, &mut self.buffer).draw_line();
        LineDrawer::new(self.triangle.c.clone(), self.triangle.a.clone(), col, &mut self.buffer).draw_line();
    }

    fn fill_triangle(&mut self, col: Rgb<u8>) {
        let bounding_box = self.triangle.get_bounding_box();
        for x in bounding_box.min_x() ..= bounding_box.max_x() {
            for y in bounding_box.min_y() ..= bounding_box.max_y() {
                if self.triangle.is_inside_point(Point2::new(x, y).clone()) {
                    self.buffer[(x, y)] = col;
                }
            }
        }
    }
}

struct VertexCoordinateMapper {
    buffer_width: u32,
    buffer_height: u32
}

impl VertexCoordinateMapper {
    fn new(buffer_width: u32, buffer_height: u32) -> Self {
        VertexCoordinateMapper { buffer_width, buffer_height }
    }

    fn map_vertex_coords_to_pixel_coord(&self, v: &Vertex) -> Result<Point2<u32>, RendererError> {
        check_if_in_normalized_device_coordinates(v)?;
        Ok (
            Point2::new (
                ((v.x + 1.0) * (self.buffer_width - 1) as f64 / 2.0) as u32,
                ((v.y + 1.0) * (self.buffer_height - 1) as f64 / 2.0) as u32
            )
        )
    }
}

fn check_if_in_normalized_device_coordinates(v: &Vertex) -> Result<(), RendererError> {
    if v.x > 1.0 || v.y > 1.0 || v.x < -1.0 || v.y < -1.0 {
        Err(RendererError::NotInNormalizedDeviceCoords(v.clone()))
    } else {
        Ok(())
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

pub trait VecFrom<T> {
    fn from_vertex(vertex: T) -> Self;
}

impl<'a> VecFrom<&'a Vertex> for cgmath::Vector3<f64> {
    fn from_vertex(vert: &'a Vertex) -> Self {
        Vector3 {
            x: vert.x,
            y: vert.y,
            z: vert.z
        }
    }
}

impl VecFrom<Vertex> for cgmath::Vector3<f64> {
    fn from_vertex(vert: Vertex) -> Self {
        Vector3 {
            x: vert.x,
            y: vert.y,
            z: vert.z
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

    // TODO change these tests to test LineDrawer instead of Renderer

    // #[test]
    // fn draw_a_zero_length_line_should_draw_a_dot() {
    //     let mut renderer = Renderer::new(2, 2);
    //     assert!(renderer.draw_line(Point2::new(0, 0), Point2::new(0, 0), Rgb([1, 1, 1])).is_ok());
    //     assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
    //     assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
    //     assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    // }

    // #[test]
    // fn draw_even_line() {
    //     let mut renderer = Renderer::new(2, 2);
    //     assert!(renderer.draw_line(Point2::new(0, 0), Point2::new(1, 1), Rgb([1, 1, 1])).is_ok());
    //     renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    // }

    // #[test]
    // fn parameter_order_should_not_matter() {
    //     let mut renderer = Renderer::new(2, 2);
    //     assert!(renderer.draw_line(Point2::new(1, 1), Point2::new(0, 0), Rgb([1, 1, 1])).is_ok());
    //     renderer_should_have_drawn_line_from_bottom_left_to_top_right(&renderer);
    // }

    // #[test]
    // fn over_indexing_should_result_in_error() {
    //     let mut renderer = Renderer::new(2, 2);
    //     let result = renderer.draw_line(Point2::new(2, 2), Point2::new(2, 2), Rgb([1, 1, 1]));
    //     assert_eq!(result, Err(RendererError::PixelOutOfImageBounds(2, 2, Point2::new(2, 2))));
    // }

    // #[test]
    // fn should_be_able_to_draw_shallow_line() {
    //     let mut renderer = Renderer::new(2, 2);
    //     let draw_result = renderer.draw_line(Point2::new(0, 0), Point2::new(1, 0), Rgb([1,1,1]));
    //     assert!(draw_result.is_ok());
    //     renderer_should_have_drawn_flat_line(&renderer);
    // }

    // #[test]
    // fn should_be_able_to_draw_steep_line() {
    //     let mut renderer = Renderer::new(2, 2);
    //     let draw_result = renderer.draw_line(Point2::new(0, 0), Point2::new(0, 1), Rgb([1,1,1]));
    //     assert!(draw_result.is_ok());
    //     renderer_should_have_drawn_straight_vertical_line(&renderer);
    // }

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
        let result = renderer.draw_triangle_2d(&vertex_a, &vertex_b, &vertex_c, Rgb([1, 1, 1]));
        assert_eq!(Ok(()), result);
        assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 2)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(2, 0)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
        assert_eq!(renderer.buffer[(1, 0)], Rgb([1, 1, 1]));
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
        let bounding_box = triangle.get_bounding_box();
        assert_eq!(bounding_box.min_x(), 0.0);
        assert_eq!(bounding_box.min_y(), -1.0);
        assert_eq!(bounding_box.max_x(), 14.0);
        assert_eq!(bounding_box.max_y(), 10.0);
    }

    #[test]
    fn should_be_able_to_draw_filled_triangle_from_vertices() {
        let mut renderer = Renderer::new(4, 4);
        let bottom_left = Vertex {x: -1.0, y: -1.0, z: 0.0};
        let tor_right = Vertex {x: 1.0, y: 1.0, z: 0.0};
        let bottom_right = Vertex {x: 1.0, y: -1.0, z: 0.0};
        let result = renderer.draw_filled_triangle_2d(&bottom_left, &tor_right, &bottom_right, Rgb([1, 1, 1]));
        assert_eq!(Ok(()), result);
        bottom_floor_should_be_filled(&renderer);
        right_wall_should_be_filled(&renderer);
        slope_should_be_filled(&renderer);
        middle_point_should_be_filled(&renderer);
    }


    #[test]
    fn should_be_able_to_create_vector3_from_vertex() {
        let vert = Vertex{x: 1.0, y: 2.0, z: 3.0};
        {
            let vec = Vector3::from_vertex(&vert);
            assert_eq!(vert.x, vec.x);
            assert_eq!(vert.y, vec.y);
            assert_eq!(vert.z, vec.z);
        }
        let vec = Vector3::from_vertex(vert);
        assert_eq!(vert.x, vec.x);
        assert_eq!(vert.y, vec.y);
        assert_eq!(vert.z, vec.z);
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

    // fn renderer_should_have_drawn_line_from_bottom_left_to_top_right(renderer: &Renderer) {
    //     assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(1, 1)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
    //     assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    // }

    // fn renderer_should_have_drawn_flat_line(renderer: &Renderer) {
    //     assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
    //     assert_eq!(renderer.buffer[(1, 0)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(0, 1)], Rgb([0, 0, 0]));
    // }

    // fn renderer_should_have_drawn_straight_vertical_line(renderer: &Renderer) {
    //     assert_eq!(renderer.buffer[(0, 0)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(1, 1)], Rgb([0, 0, 0]));
    //     assert_eq!(renderer.buffer[(0, 1)], Rgb([1, 1, 1]));
    //     assert_eq!(renderer.buffer[(1, 0)], Rgb([0, 0, 0]));
    // }
}

use cgmath::{Point3};
use image::{Rgb, RgbImage};
use std::mem;
use z_buffer::ZBuffer;
use z_buffer::PixelVisibility;

pub struct LineDrawer<'a> {
    start: Point3<u32>,
    end: Point3<u32>,
    col: Rgb<u8>,
    is_steep: bool,
    buffer: &'a mut RgbImage,
    z_buffer: &'a mut ZBuffer,
}

impl<'a> LineDrawer<'a> {
    pub fn new(
        start: Point3<u32>,
        end: Point3<u32>,
        col: Rgb<u8>,
        buffer: &'a mut RgbImage,
        z_buffer: &'a mut ZBuffer,
    ) -> Self {
        let mut drawer = LineDrawer::create_initial_instance(start, end, col, buffer, z_buffer);
        drawer.make_line_shallow();
        drawer.order_points();
        drawer
    }

    fn create_initial_instance(
        start: Point3<u32>,
        end: Point3<u32>,
        col: Rgb<u8>,
        buffer: &'a mut RgbImage,
        z_buffer: &'a mut ZBuffer,
    ) -> Self {
        LineDrawer {
            start,
            end,
            col,
            is_steep: false,
            buffer,
            z_buffer,
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
            let current_point = self.get_current_point(x);
            match self.z_buffer.update_buffer(current_point.x, current_point.y, current_point.z).unwrap() {
                PixelVisibility::Visible => {self.fill_next_line_point(x)},
                PixelVisibility::Hidden => {},
            }
        }
    }

    fn get_current_point(&self, x: u32) -> Point3<u32> {
        let offset_y = lerp(self.start.y, self.end.y, self.get_lerp_amount(x));
        let (x_coord, y_coord) = if self.is_steep {
            (offset_y, x)
        } else {
            (x, offset_y)
        };
        let z_coord = lerp(self.start.z, self.end.z, self.get_lerp_amount(x));
        Point3::new(x_coord, y_coord, z_coord)
    }

    fn fill_next_line_point(&mut self, x: u32) {
        let base_offset = lerp(
            self.start.y,
            self.end.y,
            self.get_lerp_amount(x),
        );
        if self.is_steep {
            self.buffer[(base_offset, x)] = self.col;
        } else {
            self.buffer[(x, base_offset)] = self.col;
        }
    }

    #[inline]
    fn get_lerp_amount(&self, x: u32) -> f64 {
        (x - self.start.x) as f64 / (self.end.x - self.start.x) as f64
    }
}

#[inline]
pub fn lerp(start: u32, end: u32, lerp_amount: f64) -> u32 {
    (start as f64 + (end as i32 - start as i32) as f64 * lerp_amount).round() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn draw_a_zero_length_line_should_draw_a_dot() {
        let mut buffer = RgbImage::new(2, 2);
        let mut z_buffer = ZBuffer::new(2, 2);
        LineDrawer::new(
            Point3::new(0, 0, 0),
            Point3::new(0, 0, 0),
            Rgb([1, 1, 1]),
            &mut buffer,
            &mut z_buffer,
        ).draw_line();
        assert_eq!(buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    #[test]
    fn draw_even_line() {
        let mut buffer = RgbImage::new(2, 2);
        let mut z_buffer = ZBuffer::new(2, 2);
        LineDrawer::new(
            Point3::new(0, 0, 0),
            Point3::new(1, 1, 0),
            Rgb([1, 1, 1]),
            &mut buffer,
            &mut z_buffer,
        ).draw_line();
        drawer_should_have_drawn_line_from_bottom_left_to_top_right(&buffer);
    }

    #[test]
    fn parameter_order_should_not_matter() {
        let mut buffer = RgbImage::new(2, 2);
        let mut z_buffer = ZBuffer::new(2, 2);
        LineDrawer::new(
            Point3::new(1, 1, 3),
            Point3::new(0, 0, 3),
            Rgb([1, 1, 1]),
            &mut buffer,
            &mut z_buffer,
        ).draw_line();
        drawer_should_have_drawn_line_from_bottom_left_to_top_right(&buffer);
    }

    #[test]
    fn should_be_able_to_draw_shallow_line() {
        let mut buffer = RgbImage::new(2, 2);
        let mut z_buffer = ZBuffer::new(2, 2);
        LineDrawer::new(
            Point3::new(0, 0, 0),
            Point3::new(1, 0, 0),
            Rgb([1, 1, 1]),
            &mut buffer,
            &mut z_buffer,
        ).draw_line();
        drawer_should_have_drawn_flat_line(&buffer);
    }

    #[test]
    fn should_be_able_to_draw_steep_line() {
        let mut buffer = RgbImage::new(2, 2);
        let mut z_buffer = ZBuffer::new(2, 2);
        LineDrawer::new(
            Point3::new(0, 0, 0),
            Point3::new(0, 1, 0),
            Rgb([1, 1, 1]),
            &mut buffer,
            &mut z_buffer,
        ).draw_line();
        drawer_should_have_drawn_straight_vertical_line(&buffer);
    }

    fn drawer_should_have_drawn_line_from_bottom_left_to_top_right(buffer: &RgbImage) {
        assert_eq!(buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(1, 1)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    fn drawer_should_have_drawn_flat_line(buffer: &RgbImage) {
        assert_eq!(buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(1, 0)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(0, 1)], Rgb([0, 0, 0]));
    }

    fn drawer_should_have_drawn_straight_vertical_line(buffer: &RgbImage) {
        assert_eq!(buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(0, 1)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(1, 0)], Rgb([0, 0, 0]));
    }
}

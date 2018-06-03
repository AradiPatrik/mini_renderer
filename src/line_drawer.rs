use cgmath::Point2;
use image::{Rgb, RgbImage};
use std::mem;

pub struct LineDrawer<'a> {
    start: Point2<u32>,
    end: Point2<u32>,
    col: Rgb<u8>,
    is_steep: bool,
    buffer: &'a mut RgbImage,
}

impl<'a> LineDrawer<'a> {
    pub fn new(
        start: Point2<u32>,
        end: Point2<u32>,
        col: Rgb<u8>,
        buffer: &'a mut RgbImage,
    ) -> Self {
        let mut drawer = LineDrawer::create_initial_instance(start, end, col, buffer);
        drawer.make_line_shallow();
        drawer.order_points();
        drawer
    }

    fn create_initial_instance(
        start: Point2<u32>,
        end: Point2<u32>,
        col: Rgb<u8>,
        buffer: &'a mut RgbImage,
    ) -> Self {
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
        let base_offset = lerp(
            self.start.y,
            self.end.y,
            (x - self.start.x) as f64 / (self.end.x - self.start.x) as f64,
        );
        if self.is_steep {
            self.buffer[(base_offset, x)] = self.col;
        } else {
            self.buffer[(x, base_offset)] = self.col;
        }
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
        LineDrawer::new(
            Point2::new(0, 0),
            Point2::new(0, 0),
            Rgb([1, 1, 1]),
            &mut buffer
        ).draw_line();
        assert_eq!(buffer[(0, 0)], Rgb([1, 1, 1]));
        assert_eq!(buffer[(1, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(0, 1)], Rgb([0, 0, 0]));
        assert_eq!(buffer[(1, 0)], Rgb([0, 0, 0]));
    }

    #[test]
    fn draw_even_line() {
        let mut buffer = RgbImage::new(2, 2);
        LineDrawer::new(
            Point2::new(0, 0),
            Point2::new(1, 1),
            Rgb([1, 1, 1]),
            &mut buffer
        ).draw_line();
        drawer_should_have_drawn_line_from_bottom_left_to_top_right(&buffer);
    }

    #[test]
    fn parameter_order_should_not_matter() {
        let mut buffer = RgbImage::new(2, 2);
        LineDrawer::new(
            Point2::new(1, 1),
            Point2::new(0, 0),
            Rgb([1, 1, 1]),
            &mut buffer
        ).draw_line();
        drawer_should_have_drawn_line_from_bottom_left_to_top_right(&buffer);
    }

    #[test]
    fn should_be_able_to_draw_shallow_line() {
        let mut buffer = RgbImage::new(2, 2);
        LineDrawer::new(
            Point2::new(0, 0),
            Point2::new(1, 0),
            Rgb([1, 1, 1]),
            &mut buffer
        ).draw_line();
        drawer_should_have_drawn_flat_line(&buffer);
    }

    #[test]
    fn should_be_able_to_draw_steep_line() {
        let mut buffer = RgbImage::new(2, 2);
        LineDrawer::new(
            Point2::new(0, 0),
            Point2::new(0, 1),
            Rgb([1, 1, 1]),
            &mut buffer
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

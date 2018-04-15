use image::traits::pixel_buffer::{PixelBuffer, Result};
use std::mem::swap;

pub struct Renderer<T: PixelBuffer> {
    pub buffer: T
}

impl<T: PixelBuffer> Renderer<T> {
    pub fn new(width: u16, height: u16, init_color: &T::PixelType) -> Self {
        Renderer { buffer: PixelBuffer::new(width, height, init_color) }
    }

    pub fn from_pixel_buffer(image: T) -> Self {
        Renderer { buffer: image }
    }

    pub fn line(&mut self, mut x1: u16, mut y1: u16, mut x2: u16, mut y2: u16, line_color: &T::PixelType) -> Result {
        if self.buffer.get(x1, y1).is_none() || self.buffer.get(x2, y2).is_none() {
            return Err(());
        }
        let mut is_steep = false;
        if (x2 as i32 - x1 as i32).abs() < (y2 as i32 - y1 as i32).abs() {
            swap(&mut x1, &mut y1);
            swap(&mut x2, &mut y2);
            is_steep = true;
        }
        for x in x1..=x2 {
            let offset_percent = offset_percent_u16(x1, x2, x);
            let y = lerp_u16(y1, y2, offset_percent);
            if is_steep {
                self.buffer.set(y, x, line_color)?;
            } else {
                self.buffer.set(x, y, line_color)?;
            }
        }
        Ok(())
    }
}

fn offset_percent_u16(begin: u16, end: u16, mid: u16) -> f64 {
    (mid - begin) as f64 / (end - begin) as f64
}

fn lerp_u16(begin: u16, end: u16, offset_percent: f64) -> u16 {
    begin + ((end - begin) as f64 * offset_percent).round() as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::bgr_pixel::BGRPixel;
    use image::traits::pixel::Pixel;
    use image::bgr_pixel_buffer::BGRPixelBuffer;

    #[test]
    fn should_be_able_to_create_a_new_renderer() {
        let renderer: Renderer<BGRPixelBuffer> = Renderer::new(1, 1, &BGRPixel::white());
        assert_eq!(
            renderer.buffer.as_bytes(),
            BGRPixelBuffer::new(1, 1, &BGRPixel::white()).as_bytes()
        );
    }

    #[test]
    fn from_pixel_buffer_works() {
        let pixel_buffer = BGRPixelBuffer::new(1, 1, &Pixel::white());
        let renderer = Renderer::from_pixel_buffer(pixel_buffer);
        assert_eq!(
            renderer.buffer.as_bytes(),
            BGRPixelBuffer::new(1, 1, &Pixel::white()).as_bytes()
        );
    }

    #[test]
    fn test_line() {
        let mut renderer = renderer_with_4_by_4_white_pixel_buffer();
        assert!(renderer.line(0, 0, 2, 2, &Pixel::black()).is_ok());
        assert_eq!(renderer.buffer.get(0, 0).unwrap(), Pixel::black());
        assert_eq!(renderer.buffer.get(1, 1).unwrap(), Pixel::black());
        assert_eq!(renderer.buffer.get(2, 2).unwrap(), Pixel::black());
        assert_ne!(renderer.buffer.get(3, 3).unwrap(), Pixel::black());

        renderer = renderer_with_4_by_4_white_pixel_buffer();
        assert!(!renderer.line(0, 0, 4, 4, &Pixel::black()).is_ok());
        assert_ne!(renderer.buffer.get(0, 0).unwrap(), Pixel::black());

        renderer = renderer_with_4_by_4_white_pixel_buffer();
        assert!(renderer.line(0, 0, 3, 1, &Pixel::black()).is_ok());
    }

    fn renderer_with_4_by_4_white_pixel_buffer() -> Renderer<BGRPixelBuffer> {
            renderer_with_white_buffer_with_dimensions(4, 4)
    }

    fn renderer_with_white_buffer_with_dimensions(width: u16, height: u16) -> Renderer<BGRPixelBuffer> {
        Renderer::new(width, height, &Pixel::white())
    }
}
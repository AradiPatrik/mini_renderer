use image::traits::pixel_buffer::{PixelBuffer, Result};
use std::mem::swap;
use std::io;

pub struct Renderer<T: PixelBuffer> {
    pub image: T
}

impl<T: PixelBuffer> Renderer<T> {
    pub fn from_image(image: T) -> Self {
        Renderer { image }
    }

    pub fn line(&mut self, mut x1: u16, mut y1: u16, mut x2: u16, mut y2: u16, line_color: &T::PixelType) -> Result {
        let mut is_steep = false;
        if (x2 as i32 - x1 as i32).abs() < (y2 as i32 - y1 as i32).abs() {
            swap(&mut x1, &mut y1);
            swap(&mut x2, &mut y2);
            is_steep = true;
        }
        for x in x1..x2 {
            let offset_percent = offset_percent_u16(x1, x2, x);
            let y = lerp_u16(y1, y2, offset_percent);
            if is_steep {
                self.image.set(y, x, line_color)?;
            } else {
                self.image.set(x, y, line_color)?;
            }
        }
        Ok(())
    }

    pub fn new(width: u16, height: u16, init_color: &T::PixelType) -> Self {
        Renderer { image: PixelBuffer::new(width, height, init_color) }
    }

    pub fn render(&self, file_name: &str) -> io::Result<usize> {
        self.image.write_to_file(file_name)
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
    #[test]
    fn ok() {

    }
}
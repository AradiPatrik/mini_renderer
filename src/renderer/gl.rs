use image::tga_image::TGAImage;
use image::tga_image;
use super::super::image::pixel::*;
use std::io;
use std::mem;

pub struct Renderer {
    pub image: TGAImage
}

impl Renderer {
    pub fn from_image(image: TGAImage) -> Self {
        Renderer { image }
    }

    pub fn line(&mut self, mut x1: u16, mut y1: u16, mut x2: u16, mut y2: u16, line_color: &Pixel) -> tga_image::Result {
        let mut is_steep = false;
        if (x2 as i32 - x1 as i32).abs() < (y2 as i32 - y1 as i32).abs() {
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
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

    pub fn new(width: u16, height: u16, init_color: &Pixel) -> Self {
        Renderer { image: TGAImage::new(width, height, init_color) }
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
    use super::*;


}
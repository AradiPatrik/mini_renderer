use super::pixel::*;
use super::tga_header::*;
use std::iter;
use std::io;
use std::fs;
use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct TGAImage {
    pub width: u16,
    pub height: u16,
    data: Vec<u8>,
}

impl TGAImage {
    pub fn new(width: u16, height: u16, init_color: Pixel) -> Self {
        let data: Vec<u8> = iter::repeat(init_color)
            .flat_map(|p| p.into_iter())
            .take((width as u32 * height as u32 * 3 as u32) as usize)
            .collect();
        TGAImage { width, height, data }
    }

    pub fn set(&mut self, x: u16, y: u16, pixel: &Pixel) {
        let start = self.coords_to_index(x, y);
        let end = start + 3usize;
        self.data.splice(start..end, pixel.into_iter());
    }

    pub fn get(&self, x: u16, y: u16) -> Option<Pixel>{
        if x >= self.width || y >= self.height {
            None
        } else {
            let start = self.coords_to_index(x, y);
            let pixel: Pixel = self.data.iter()
                .skip(start)
                .take(3)
                .collect();
            Some(pixel)
        }
    }

    pub fn write_to_file(&self, file_name: &str) -> io::Result<usize> {
        let file_handle = fs::File::create(file_name)?;
        let mut output_stream = io::BufWriter::new(file_handle);
        output_stream.write(&TGAHeader::get_rgb_header(self.width, self.height).get_bytes())
            .and_then(|_|output_stream.write(self.data.as_slice()))
    }

    fn coords_to_index(&self, x: u16, y: u16) -> usize {
        x as usize * 3usize + y as usize * self.width as usize * 3usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creating_a_2_by_2_picture_should_result_in_rgb_repeating_4_times() {
        let image = TGAImage::new(2u16, 2u16, Pixel::from_rgb(1, 2, 3));
        assert_eq!(
            image.data, vec![
                3, 2, 1,  3, 2, 1,
                3, 2, 1,  3, 2, 1,
            ]
        )
    }

    #[test]
    fn getting_a_pixel_should_work() {
        let image = TGAImage::new(500, 500, Pixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(1, 0).unwrap(), Pixel::from_rgb(1, 2, 3));
    }

    #[test]
    fn setting_a_pixel_should_work() {
        let mut image = TGAImage::new(500, 500, Pixel::from_rgb(0, 0, 0));
        image.set(0, 0, &Pixel::from_rgb(1, 1, 1));
        image.get(0, 0);
    }

    #[test]
    fn test_white_image() {
        let image = TGAImage::new(500, 500, Pixel::from_rgb(255, 255, 255));
        image.write_to_file("white_test.tga").unwrap();
    }

    #[test]
    fn test_upper_right_corner() {
        let image = TGAImage::new(500, 500, Pixel::from_rgb(255, 255, 255));
        assert_eq!(image.get(499, 499), Some(Pixel::from_rgb(255, 255, 255)));
    }

    #[test]
    fn test_white_image_with_black_line() {
        let width = 500u16;
        let height = 500u16;
        let mut image = TGAImage::new(width, height, Pixel::from_rgb(255, 255, 255));
        for i in 0..width {
            image.set(i, i, &Pixel::from_rgb(0, 0, 0));
        }
        image.write_to_file("line_test.tga").unwrap();
    }

    #[test]
    fn test_blue_image() {
        let image = TGAImage::new(300, 300, Pixel::from_rgb(0, 0, 255));
        image.write_to_file("blue_test.tga").unwrap();
    }

    #[test]
    fn dots_test() {
        let mut image = TGAImage::new(300, 300, Pixel::from_rgb(255, 255, 255));
        image.set(10, 80, &Pixel::from_rgb(255, 0, 0));
        image.set(80, 10, &Pixel::from_rgb(0, 255, 0));
        image.set(240, 100, &Pixel::from_rgb(0, 0, 255));
        image.write_to_file("dots_test.tga").unwrap();
    }
}
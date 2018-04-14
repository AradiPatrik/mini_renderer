use std::io;
use std::io::Write;
use std::iter::repeat;
use std::fs::File;
use image::tga_pixel::BGRPixel;
use image::tga_header::TGAHeader;
use image::traits::pixel_buffer::{PixelBuffer, Result};
use image::traits::image_header::ImageHeader;

#[derive(Default, Debug)]
pub struct TGAImage {
    pub width: u16,
    pub height: u16,
    data: Vec<u8>,
}

impl PixelBuffer for TGAImage {
    type PixelType = BGRPixel;
    fn new(width: u16, height: u16, init_color: &BGRPixel) -> Self {
        let data: Vec<u8> = repeat(init_color)
            .flat_map(|p| p.into_iter())
            .take((width as u32 * height as u32 * 3 as u32) as usize)
            .collect();
        TGAImage { width, height, data }
    }

    fn set(&mut self, x: u16, y: u16, pixel: &BGRPixel) -> Result {
        if x >= self.width || y >= self.height {
            Err(())
        } else {
            let start = self.coords_to_index(x, y);
            let end = start + 3usize;
            self.data.splice(start..end, pixel.into_iter());
            Ok(())
        }
    }

    fn get(&self, x: u16, y: u16) -> Option<BGRPixel> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let start = self.coords_to_index(x, y);
            let pixel: BGRPixel = self.data.iter()
                .skip(start)
                .take(3)
                .collect();
            Some(pixel)
        }
    }

    fn write_to_file(&self, file_name: &str) -> io::Result<usize> {
        let file_handle = File::create(file_name)?;
        let mut output_stream = io::BufWriter::new(file_handle);
        output_stream.write(&TGAHeader::get_rgb_header(self.width, self.height).get_bytes())
            .and_then(|_| output_stream.write(self.data.as_slice()))
    }
}

impl TGAImage {
    fn coords_to_index(&self, x: u16, y: u16) -> usize {
        x as usize * 3usize + y as usize * self.width as usize * 3usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::traits::pixel::Pixel;

    #[test]
    fn creating_a_2_by_2_picture_should_result_in_rgb_repeating_4_times() {
        let image = TGAImage::new(2u16, 2u16, &BGRPixel::from_rgb(1, 2, 3));
        assert_eq!(
            image.data, vec![
                3, 2, 1, 3, 2, 1,
                3, 2, 1, 3, 2, 1,
            ]
        )
    }

    #[test]
    fn getting_a_valid_pixel_should_return_some_valid_pixel() {
        let image = TGAImage::new(3, 3, &BGRPixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(1, 2), Some(BGRPixel::from_rgb(1, 2, 3)));
    }

    #[test]
    fn getting_an_invalid_pixel_should_return_none() {
        let image = TGAImage::new(3, 3, &BGRPixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(50, 10), None);
    }

    #[test]
    fn over_indexing_on_the_x_axis_should_return_none() {
        let image = TGAImage::new(3, 3, &BGRPixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(3, 2), None);
    }

    #[test]
    fn over_indexing_on_the_y_axis_should_return_none() {
        let image = TGAImage::new(3, 3, &BGRPixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(2, 3), None);
    }

    #[test]
    fn setting_a_valid_pixel_should_return_ok_and_should_set_the_right_pixel() {
        let mut image = TGAImage::new(3, 3, &BGRPixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(0, 0, &BGRPixel::from_rgb(1, 1, 1)), Ok(()));
        assert_eq!(image.get(0, 0), Some(BGRPixel::from_rgb(1, 1, 1)));
    }

    #[test]
    fn setting_an_over_indexed_by_one_pixel_on_the_x_axis_should_return_err() {
        let mut image = TGAImage::new(3, 3, &BGRPixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(3, 2, &BGRPixel::from_rgb(1, 1, 1)), Err(()));
    }

    #[test]
    fn setting_an_over_indexed_by_one_pixel_on_the_y_axis_should_return_err() {
        let mut image = TGAImage::new(3, 3, &BGRPixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(2, 3, &BGRPixel::from_rgb(1, 1, 1)), Err(()));
    }

    #[test]
    fn setting_an_over_indexed_pixel_should_return_err() {
        let mut image = TGAImage::new(10, 10, &BGRPixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(20, 20, &BGRPixel::from_rgb(1, 1, 1)), Err(()));
    }
}
use super::pixel::*;
use super::tga_header::*;
use std::iter;
use std::io;
use std::fs;
use std::io::prelude::*;
use std;

#[derive(Default, Debug)]
pub struct TGAImage {
    pub width: u16,
    pub height: u16,
    data: Vec<u8>,
}

pub type Result = std::result::Result<(), ()>;

impl TGAImage {
    pub fn new(width: u16, height: u16, init_color: &Pixel) -> Self {
        let data: Vec<u8> = iter::repeat(init_color)
            .flat_map(|p| p.into_iter())
            .take((width as u32 * height as u32 * 3 as u32) as usize)
            .collect();
        TGAImage { width, height, data }
    }

    pub fn set(&mut self, x: u16, y: u16, pixel: &Pixel) -> Result {
        if x >= self.width || y >= self.height {
            Err(())
        } else {
            let start = self.coords_to_index(x, y);
            let end = start + 3usize;
            self.data.splice(start..end, pixel.into_iter());
            Ok(())
        }
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
        let image = TGAImage::new(2u16, 2u16, &Pixel::from_rgb(1, 2, 3));
        assert_eq!(
            image.data, vec![
                3, 2, 1,  3, 2, 1,
                3, 2, 1,  3, 2, 1,
            ]
        )
    }

    #[test]
    fn getting_a_valid_pixel_should_return_some_valid_pixel() {
        let image = TGAImage::new(3, 3, &Pixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(1, 2), Some(Pixel::from_rgb(1, 2, 3)));
    }

    #[test]
    fn getting_an_invalid_pixel_should_return_none() {
        let image = TGAImage::new(3, 3, &Pixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(50, 10), None);
    }

    #[test]
    fn over_indexing_on_the_x_axis_should_return_none() {
        let image = TGAImage::new(3, 3, &Pixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(3, 2), None);
    }

    #[test]
    fn over_indexing_on_the_y_axis_should_return_none() {
        let image = TGAImage::new(3, 3, &Pixel::from_rgb(1, 2, 3));
        assert_eq!(image.get(2, 3), None);
    }

    #[test]
    fn setting_a_valid_pixel_should_return_ok_and_should_set_the_right_pixel() {
        let mut image = TGAImage::new(3, 3, &Pixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(0, 0, &Pixel::from_rgb(1, 1, 1)), Ok(()));
        assert_eq!(image.get(0, 0), Some(Pixel::from_rgb(1, 1, 1)));
    }

    #[test]
    fn setting_an_over_indexed_by_one_pixel_on_the_x_axis_should_return_err() {
        let mut image = TGAImage::new(3, 3, &Pixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(3, 2, &Pixel::from_rgb(1, 1, 1)), Err(()));
    }

    #[test]
    fn setting_an_over_indexed_by_one_pixel_on_the_y_axis_should_return_err() {
        let mut image = TGAImage::new(3, 3, &Pixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(2, 3, &Pixel::from_rgb(1, 1, 1)), Err(()));
    }

    #[test]
    fn setting_an_over_indexed_pixel_should_return_err() {
        let mut image = TGAImage::new(10, 10, &Pixel::from_rgb(0, 0, 0));
        assert_eq!(image.set(20, 20, &Pixel::from_rgb(1, 1, 1)), Err(()));
    }
}
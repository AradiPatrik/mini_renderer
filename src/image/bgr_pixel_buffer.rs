use std::iter::repeat;
use image::bgr_pixel::BGRPixel;
use image::traits::pixel_buffer::{PixelBuffer, Result};
use std::io::Write;
use std::io;

#[derive(Default, Debug)]
pub struct BGRPixelBuffer {
    pub width: u16,
    pub height: u16,
    data: Vec<u8>,
}

impl PixelBuffer for BGRPixelBuffer {
    type PixelType = BGRPixel;
    fn new(width: u16, height: u16, init_color: &BGRPixel) -> Self {
        let data: Vec<u8> = repeat(init_color)
            .flat_map(|p| p.into_iter())
            .take((width as u32 * height as u32 * 3 as u32) as usize)
            .collect();
        BGRPixelBuffer { width, height, data }
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

    fn unpack(self) -> Vec<u8> {
        self.data
    }

    fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    fn write<T: Write>(&self, sync: &mut T) -> io::Result<usize> {
        sync.write(self.data.as_slice())
    }

}

impl AsRef<[u8]> for BGRPixelBuffer {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl BGRPixelBuffer {
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
        let image = BGRPixelBuffer::new(2u16, 2u16, &Pixel::from_rgb(1, 2, 3));
        assert_eq!(
            image.data, vec![
                3, 2, 1, 3, 2, 1,
                3, 2, 1, 3, 2, 1,
            ]
        )
    }

    #[test]
    fn getting_a_valid_pixel_should_return_some_valid_pixel() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.get(1, 2), Some(Pixel::black()));
    }

    #[test]
    fn getting_an_invalid_pixel_should_return_none() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.get(50, 10), None);
    }

    #[test]
    fn over_indexing_on_the_x_axis_should_return_none() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.get(3, 2), None);
    }

    #[test]
    fn over_indexing_on_the_y_axis_should_return_none() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.get(2, 3), None);
    }

    #[test]
    fn setting_a_valid_pixel_should_return_ok_and_should_set_the_right_pixel() {
        let mut image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.set(0, 0, &Pixel::black()), Ok(()));
        assert_eq!(image.get(0, 0), Some(Pixel::black()));
    }

    #[test]
    fn setting_an_over_indexed_by_one_pixel_on_the_x_axis_should_return_err() {
        let mut image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.set(3, 2, &Pixel::black()), Err(()));
    }

    #[test]
    fn should_be_able_to_unpack_data() {
        let image = BGRPixelBuffer::new(2, 2, &Pixel::black());
        let old_data = image.data.clone();
        let mut new_data = image.unpack();
        assert_eq!(old_data, new_data);
        new_data[0] = 8;
        assert_ne!(old_data, new_data);
    }

    #[test]
    fn should_be_able_to_get_image_as_reference() {
        let image = BGRPixelBuffer::new(2, 1, &Pixel::black());
        let bytes: &[u8] = image.data.as_ref();
        assert_eq!(bytes, image.data.as_slice());
    }

    #[test]
    fn should_be_able_to_get_bytes() {
        let image = BGRPixelBuffer::new(1, 2, &Pixel::black());
        let bytes = image.as_bytes();
        assert_eq!(bytes, image.data.as_slice());
    }

    #[test]
    fn should_be_able_to_write_to_file() {
        let image = BGRPixelBuffer::new(1, 2, &Pixel::black());
        let mut writer = vec![];
        assert!(image.write(&mut writer).is_ok());
        assert_eq!(writer.as_slice(), image.data.as_slice());
    }
}
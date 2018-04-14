use std::iter::repeat;
use image::bgr_pixel::BGRPixel;
use image::traits::pixel_buffer::{PixelBuffer, Result};

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

    fn get_data_ref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    fn get_data_ref(&self) -> &Vec<u8> {
        &self.data
    }

    fn clone_buffer(&self) -> Vec<u8> {
        self.data.clone()
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
    fn setting_an_over_indexed_by_one_pixel_on_the_y_axis_should_return_err() {
        let mut image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.set(2, 3, &Pixel::black()), Err(()));
    }

    #[test]
    fn setting_an_over_indexed_pixel_should_return_err() {
        let mut image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.set(20, 20, &Pixel::black()), Err(()));
    }

    #[test]
    fn get_data_ref_mut_should_return_mut_ref_to_data() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        let data_clone = image.data.clone();
        assert_eq!(data_clone, image.data);
    }

    #[test]
    fn should_be_able_to_modifie_data_through_get_data_ref_mut() {
        let mut image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        image.get_data_ref_mut()[0] = 1;
        assert_eq!(image.data[0], 1);
    }

    #[test]
    fn get_data_ref_should_return_ref_to_data() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(&image.data, image.get_data_ref());
    }

    #[test]
    fn can_get_copy_of_data() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        assert_eq!(image.data, image.clone_buffer());
    }

    #[test]
    fn changing_clone_should_not_affect_original() {
        let image = BGRPixelBuffer::new(3, 3, &Pixel::black());
        let mut data_clone = image.clone_buffer();
        data_clone[0] = 100;
        assert_eq!(image.data[0], 0);
    }
}
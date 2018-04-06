use super::pixel::*;
use std::iter;

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
        if x > self.width || y > self.height {
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

    fn coords_to_index(&self, x: u16, y: u16) -> usize {
        (x * 3u16 + y * self.width * 3u16) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creating_a_2_by_2_black_picture_should_result_in_rgb_repeating_4_times() {
        let image = TGAImage::new(2u16, 2u16, Pixel::from_rgb(1, 2, 3));
        assert_eq!(
            image.data, vec![
                1, 2, 3,  1, 2, 3,
                1, 2, 3,  1, 2, 3,
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
}
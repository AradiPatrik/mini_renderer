use std::iter::FromIterator;
use image::traits::pixel::Pixel;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BGRPixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Pixel for BGRPixel {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        BGRPixel { b, g, r }
    }
}

impl IntoIterator for BGRPixel {
    type Item = u8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        PixelIntoIterator { pixel: self, index: 0 }
    }
}

impl<'a> IntoIterator for &'a BGRPixel {
    type Item = u8;
    type IntoIter = PixelIterator<'a>;

    fn into_iter(self) -> PixelIterator<'a> {
        PixelIterator { pixel: self, index: 0 }
    }
}

impl FromIterator<u8> for BGRPixel {
    fn from_iter<T: IntoIterator<Item=u8>>(iter: T) -> Self {
        let mut into_iterator = iter.into_iter();
        BGRPixel {
            b: into_iterator.next().unwrap(),
            g: into_iterator.next().unwrap(),
            r: into_iterator.next().unwrap(),
        }
    }
}

pub struct PixelIntoIterator {
    pixel: BGRPixel,
    index: usize,
}

impl Iterator for PixelIntoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let result = match self.index {
            0 => self.pixel.b,
            1 => self.pixel.g,
            2 => self.pixel.r,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

pub struct PixelIterator<'a> {
    pixel: &'a BGRPixel,
    index: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let result = match self.index {
            0 => self.pixel.b,
            1 => self.pixel.g,
            2 => self.pixel.r,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

pub const BITS_IN_RGB_PIXEL: u8 = 24u8;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_rgb_works_with_same_values() {
        let pix = BGRPixel::from_rgb(0, 0, 0);
        assert_eq!(pix.r, 0);
        assert_eq!(pix.g, 0);
        assert_eq!(pix.b, 0);
    }

    #[test]
    fn from_rgb_works_with_different_values() {
        let pix = BGRPixel::from_rgb(10, 20, 30);
        assert_eq!(pix.r, 10);
        assert_eq!(pix.g, 20);
        assert_eq!(pix.b, 30);
    }

    #[test]
    fn default_interface_method_white_works() {
        assert_eq!(BGRPixel::white(), BGRPixel::from_rgb(255, 255, 255));
    }

    #[test]
    fn default_interface_method_black_works() {
        assert_eq!(BGRPixel::black(), BGRPixel::from_rgb(0, 0, 0));
    }

    #[test]
    fn default_interface_method_blue_works() {
        assert_eq!(BGRPixel::blue(), BGRPixel::from_rgb(0, 0, 255));
    }

    #[test]
    fn default_interface_method_red_works() {
        assert_eq!(BGRPixel::red(), BGRPixel::from_rgb(255, 0, 0));
    }

    #[test]
    fn default_interface_method_green_works() {
        assert_eq!(BGRPixel::green(), BGRPixel::from_rgb(0, 255, 0));
    }

    #[test]
    fn into_iter_works_for_pixel() {
        let pixel = BGRPixel::from_rgb(1, 2, 3);
        let mut rgb = Vec::with_capacity(3);
        for i in pixel {
            rgb.push(i);
        }
        assert_eq!(rgb, [3, 2, 1]);
    }

    #[test]
    fn into_iter_works_for_borrowed_pixel() {
        let pixel = BGRPixel::from_rgb(1, 2, 3);
        let mut rgb = Vec::with_capacity(3);
        for i in &pixel {
            rgb.push(i);
        }
        assert_eq!(rgb, [3, 2, 1]);
    }

    #[test]
    fn can_create_pixel_from_slice() {
        let pixel = BGRPixel::from_iter(vec![1u8, 2u8, 3u8].into_iter());
        assert_eq!(pixel, BGRPixel::from_rgb(3, 2, 1));
    }
}
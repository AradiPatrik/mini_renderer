use std;
use std::iter::FromIterator;

pub trait Pixel where Self: std::marker::Sized{
    fn from_rgb(r: u8, g: u8, b: u8) -> Self;

    fn white() -> Self {
        Self::from_rgb(255, 255, 255)
    }

    fn blue() -> Self {
        Self::from_rgb(0, 0, 255)
    }

    fn black() -> Self {
        Self::from_rgb(0, 0, 0)
    }

    fn red() -> Self {
        Self::from_rgb(255, 0, 0)
    }

    fn green() -> Self {
        Self::from_rgb(0, 255, 0)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TGAPixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Pixel for TGAPixel {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        TGAPixel { b, g, r }
    }
}

impl IntoIterator for TGAPixel {
    type Item = u8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        PixelIntoIterator { pixel: self, index: 0 }
    }
}

pub struct PixelIntoIterator {
    pixel: TGAPixel,
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

impl<'a> IntoIterator for &'a TGAPixel {
    type Item = u8;
    type IntoIter = PixelIterator<'a>;

    fn into_iter(self) -> PixelIterator<'a> {
        PixelIterator { pixel: self, index: 0 }
    }
}

pub struct PixelIterator<'a> {
    pixel: &'a TGAPixel,
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

impl<'a> FromIterator<&'a u8> for TGAPixel {
    fn from_iter<T: IntoIterator<Item= &'a u8>>(iter: T) -> Self {
        let mut into_iterator = iter.into_iter();
        TGAPixel {
            b: *into_iterator.next().unwrap(),
            g: *into_iterator.next().unwrap(),
            r: *into_iterator.next().unwrap(),
        }
    }
}

pub const BITS_IN_RGB_PIXEL: u8 = 24u8;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_rgb_works_with_same_values() {
        let pix = TGAPixel::from_rgb(0, 0, 0);
        assert_eq!(pix.r, 0);
        assert_eq!(pix.g, 0);
        assert_eq!(pix.b, 0);
    }

    #[test]
    fn from_rgb_works_with_different_values() {
        let pix = TGAPixel::from_rgb(10, 20, 30);
        assert_eq!(pix.r, 10);
        assert_eq!(pix.g, 20);
        assert_eq!(pix.b, 30);
    }

    #[test]
    fn default_interface_method_white_works() {
        assert_eq!(TGAPixel::white(), TGAPixel::from_rgb(255, 255, 255));
    }

    #[test]
    fn default_interface_method_black_works() {
        assert_eq!(TGAPixel::black(), TGAPixel::from_rgb(0, 0, 0));
    }

    #[test]
    fn default_interface_method_blue_works() {
        assert_eq!(TGAPixel::blue(), TGAPixel::from_rgb(0, 0, 255));
    }

    #[test]
    fn default_interface_method_red_works() {
        assert_eq!(TGAPixel::red(), TGAPixel::from_rgb(255, 0, 0));
    }

    #[test]
    fn default_interface_method_green_works() {
        assert_eq!(TGAPixel::green(), TGAPixel::from_rgb(0, 255, 0));
    }

    #[test]
    fn into_iter_works_for_pixel() {
        let pixel = TGAPixel::from_rgb(1, 2, 3);
        let mut rgb = Vec::with_capacity(3);
        for i in pixel {
            rgb.push(i);
        }
        assert_eq!(rgb, [3, 2, 1]);
    }

    #[test]
    fn into_iter_works_for_borrowed_pixel() {
        let pixel = TGAPixel::from_rgb(1, 2, 3);
        let mut rgb = Vec::with_capacity(3);
        for i in &pixel {
            rgb.push(i);
        }
        assert_eq!(rgb, [3, 2, 1]);
    }

    #[test]
    fn can_create_pixel_from_slice() {
        let pixel = TGAPixel::from_iter(&[1u8, 2u8, 3u8]);
        assert_eq!(pixel, TGAPixel::from_rgb(3, 2, 1));
    }
}
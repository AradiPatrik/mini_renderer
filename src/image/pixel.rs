use std::iter::FromIterator;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new() -> Self {
        Pixel::from_rgb(255, 255, 255)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }
}

impl IntoIterator for Pixel {
    type Item = u8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        PixelIntoIterator { pixel: self, index: 0 }
    }
}

pub struct PixelIntoIterator {
    pixel: Pixel,
    index: usize,
}

impl Iterator for PixelIntoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<'a> IntoIterator for &'a Pixel {
    type Item = u8;
    type IntoIter = PixelIterator<'a>;

    fn into_iter(self) -> PixelIterator<'a> {
        PixelIterator { pixel: self, index: 0 }
    }
}

pub struct PixelIterator<'a> {
    pixel: &'a Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl<'a> FromIterator<&'a u8> for Pixel {
    fn from_iter<T: IntoIterator<Item= &'a u8>>(iter: T) -> Self {
        let mut into_iterator = iter.into_iter();
        Pixel {
            r: *into_iterator.next().unwrap(),
            g: *into_iterator.next().unwrap(),
            b: *into_iterator.next().unwrap(),
        }
    }
}

pub const BITS_IN_RGB_PIXEL: u8 = 24u8;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_iterate_over_pixel() {
        let pixel = Pixel::from_rgb(1, 2, 3);
        let mut rgb = Vec::with_capacity(3);
        for i in pixel {
            rgb.push(i);
        }
        assert_eq!(rgb, [1, 2, 3]);
    }

    #[test]
    fn can_iterate_over_borrowed_pixel() {
        let pixel = Pixel::from_rgb(1, 2, 3);
        let mut rgb = Vec::with_capacity(3);
        for i in &pixel {
            rgb.push(i);
        }
        assert_eq!(rgb, [1, 2, 3]);
    }

    #[test]
    fn can_create_pixel_from_slice() {
        let pixel = Pixel::from_iter(&[1u8, 2u8, 3u8]);
        assert_eq!(pixel, Pixel::from_rgb(1, 2, 3));
    }
}
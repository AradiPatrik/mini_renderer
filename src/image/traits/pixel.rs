use std::iter::FromIterator;

pub trait Pixel: Sized + for<'a> FromIterator<&'a u8> {
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
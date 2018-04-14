use std::result;
use image::traits::pixel::Pixel;
use std::io::Write;
use std::io;

pub type Result = result::Result<(), ()>;

pub trait PixelBuffer: AsRef<[u8]> {
    type PixelType: Pixel;

    fn new(width: u16, height: u16, init_color: &Self::PixelType) -> Self;
    fn set (&mut self, x: u16, y: u16, pixel: &Self::PixelType) -> Result;
    fn get(&self, x: u16, y: u16) -> Option<Self::PixelType>;
    fn unpack(self) -> Vec<u8>;
    fn as_bytes(&self) -> &[u8];
    fn write<T: Write>(&self, sync: &mut T) -> io::Result<usize>;
}
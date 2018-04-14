use std::io;
use std::result;

pub type Result = result::Result<(), ()>;

pub trait PixelBuffer {
    type PixelType;

    fn new(width: u16, height: u16, init_color: &Self::PixelType) -> Self;
    fn set (&mut self, x: u16, y: u16, pixel: &Self::PixelType) -> Result;
    fn get(&self, x: u16, y: u16) -> Option<Self::PixelType>;
    fn write_to_file(&self, file_name: &str) -> io::Result<usize>;
}
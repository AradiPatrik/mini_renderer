use std::result;

pub type Result = result::Result<(), ()>;

pub trait PixelBuffer {
    type PixelType;

    fn new(width: u16, height: u16, init_color: &Self::PixelType) -> Self;
    fn set (&mut self, x: u16, y: u16, pixel: &Self::PixelType) -> Result;
    fn get(&self, x: u16, y: u16) -> Option<Self::PixelType>;
    fn get_data_ref_mut(&mut self) -> &mut Vec<u8>;
    fn get_data_ref(& self) -> & Vec<u8>;
    fn clone_buffer(& self) -> Vec<u8>;
}
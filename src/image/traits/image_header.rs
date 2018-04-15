use std::io::Write;
use std::io::Result;

pub trait ImageHeader {
    fn as_bytes(&self) -> Vec<u8>;
    fn write<W: Write>(&self, sync: &mut W) -> Result<usize>;
}
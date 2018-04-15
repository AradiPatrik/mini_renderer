use image::tga_header::TGAHeader;
use image::bgr_pixel_buffer::BGRPixelBuffer;
use image::bgr_pixel::BGRPixel;
use image::traits::pixel_buffer::PixelBuffer;
use std::io::Write;
use std::io;
use image::traits::image_header::ImageHeader;

pub struct TGAImage {
    pub header: TGAHeader,
    pub data: BGRPixelBuffer,
}

impl TGAImage {
    pub fn new(width: u16, height: u16, init_pixel: &BGRPixel) -> Self {
        TGAImage {
            header: TGAHeader::get_rgb_header(width, height),
            data: BGRPixelBuffer::new(width, height, init_pixel)
        }
    }

    pub fn from_header_and_buffer(header: TGAHeader, data: BGRPixelBuffer) -> Self {
        if header.width != data.width || header.height != data.height {
            panic!("Header and buffer width or height does not match!");
        }
        TGAImage {
            header,
            data
        }
    }

    pub fn width(&self) -> u16 {
        self.header.width
    }

    pub fn height(&self) -> u16 {
        self.header.height
    }

    pub fn write<W: Write>(&self, sync: &mut W) -> io::Result<usize> {
        let original_data = self.header
            .as_bytes()
            .iter()
            .cloned()
            .chain(self.data.as_bytes().iter().cloned())
            .collect::<Vec<_>>();
        sync.write(&original_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use image::traits::pixel::Pixel;

    #[test]
    fn new_image_header_should_have_correct_width_and_height() {
        let image = TGAImage::new(1, 2, &Pixel::black());
        assert_eq!(image.width(), 1);
        assert_eq!(image.height(), 2)
    }

    #[test]
    fn should_be_able_to_initialize_from_pixel_buffer() {
        let image = TGAImage::from_header_and_buffer(TGAHeader::get_rgb_header(3, 3), BGRPixelBuffer::new(3, 3, &BGRPixel::white()));
        assert_eq!(image.data.as_bytes(), TGAImage::new(3, 3, &BGRPixel::white()).data.as_bytes());
        assert_eq!(image.header.as_bytes(), TGAHeader::get_rgb_header(3, 3).as_bytes());
    }

    #[test]
    #[should_panic]
    fn from_header_and_buffer_should_panic_when_dimensions_mismatch() {
        TGAImage::from_header_and_buffer(
            TGAHeader::get_rgb_header(10, 10),
            BGRPixelBuffer::new(3, 3, &BGRPixel::white())
        );
    }

    #[test]
    fn should_be_able_to_write_image() {
        let image = TGAImage::new(1, 2, &Pixel::white());
        let mut sync = Vec::new();
        assert!(image.write(&mut sync).is_ok());
        let original_data = image.header
            .as_bytes()
            .iter()
            .cloned()
            .chain(image.data.as_bytes().iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(sync, original_data)
    }
}
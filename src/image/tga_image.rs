use image::tga_header::TGAHeader;
use image::bgr_pixel_buffer::BGRPixelBuffer;
use image::bgr_pixel::BGRPixel;
use image::traits::pixel::Pixel;
use image::traits::pixel_buffer::PixelBuffer;

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

    pub fn width(&self) -> u16 {
        self.header.width
    }

    pub fn height(&self) -> u16 {
        self.header.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_image_header_should_have_correct_width_and_height() {
        let image = TGAImage::new(1, 2, &Pixel::black());
        assert_eq!(image.width(), 1);
        assert_eq!(image.height(), 2)
    }

    #[test]
    fn should_be_able_to_initialize_from_pixel_buffer() {

    }
}
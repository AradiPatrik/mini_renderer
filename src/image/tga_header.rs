use image::traits::image_header::ImageHeader;
use image::tga_pixel::BITS_IN_RGB_PIXEL;

pub const COLOR_MAPPED_IMAGE: u8 = 1u8;
pub const UNMAPPED_BGR: u8 = 2u8;

#[derive(Default)]
pub struct TGAHeader {
    id_length: u8,
    colormap_type: u8,
    data_type_code: u8,
    colormap_origin: u16,
    colormap_length: u16,
    colormap_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TGAHeader {
    pub fn get_rgb_header(width: u16, height: u16) -> Self {
        let mut header = TGAHeader::default();
        header.data_type_code = UNMAPPED_BGR;
        header.width = width;
        header.height = height;
        header.bits_per_pixel = BITS_IN_RGB_PIXEL;
        header
    }
}

impl ImageHeader for TGAHeader {
    fn get_bytes(&self) -> Vec<u8> {
        vec![
            self.id_length,
            self.colormap_type,
            self.data_type_code,
            get_low_bits(self.colormap_length),
            get_high_bits(self.colormap_length),
            get_low_bits(self.colormap_origin),
            get_high_bits(self.colormap_origin),
            self.colormap_depth,
            get_low_bits(self.x_origin),
            get_high_bits(self.x_origin),
            get_low_bits(self.y_origin),
            get_high_bits(self.y_origin),
            get_low_bits(self.width),
            get_high_bits(self.width),
            get_low_bits(self.height),
            get_high_bits(self.height),
            self.bits_per_pixel,
            self.image_descriptor
        ]
    }
}

fn get_low_bits(bit_field: u16) -> u8 {
    bit_field as u8
}

fn get_high_bits(bit_field: u16) -> u8 {
    (bit_field >> 8) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_low_bits_works() {
        let bit_field = 0b1111_1111_0000_0000u16;
        assert_eq!(get_low_bits(bit_field), 0b0000_0000u8);
    }

    #[test]
    fn get_high_bits_works() {
        let bit_field = 0b1111_1111_0000_0000u16;
        assert_eq!(get_high_bits(bit_field), 0b1111_1111u8);
    }

    #[test]
    fn get_rgb_header_bytes_work() {
        let bytes = TGAHeader::get_rgb_header(1, 1).get_bytes();
        assert_eq!(
            bytes,
            vec!
            [
                0u8, // id_length       u8
                0u8, // colormap type   u8
                2u8, // data_type_code  u8
                0u8, // colormap_origin u16
                0u8, // colormap_origin u16
                0u8, // colormap_length u16
                0u8, // colormap_length u16
                0u8, // colormap_depth  u8
                0u8, // x_origin        u16
                0u8, // x_origin        u16
                0u8, // y_origin        u16
                0u8, // y_origin        u16
                1u8, // width           u16
                0u8, // width           u16
                1u8, // height          u16
                0u8, // height          u16
                24u8, // bits_per_pixel  u8
                0u8, // image_desc      u8
            ]
        );
    }

}
use super::pixel;

pub const COLOR_MAPPED_IMAGE: u8 = 1u8;
pub const UNMAPPED_RGB: u8 = 2u8;

#[derive(Default)]
pub struct TGAHeader {
    _id_length: u8,
    _colormap_type: u8,
    data_type_code: u8,
    _color_map_origin: u16,
    _color_map_length: u16,
    _color_map_depth: u8,
    _x_origin: u16,
    _y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    _image_descriptor: u8,
}

impl TGAHeader {
    pub fn get_rgb_header(width: u16, height: u16) -> Self {
        let mut header = TGAHeader::default();
        header.data_type_code = UNMAPPED_RGB;
        header.width = width;
        header.height = height;
        header.bits_per_pixel = pixel::BITS_IN_RGB_PIXEL;
        header
    }
}
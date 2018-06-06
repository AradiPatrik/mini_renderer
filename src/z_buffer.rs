use image::GrayImage;
use image::Luma;

#[derive(Debug, PartialEq)]
pub enum PixelVisibility {
    Visible,
    Hidden
}

pub struct UpdateResult {
    pixel_visibility: PixelVisibility
}

impl UpdateResult {
    pub fn new(pixel_visibility: PixelVisibility) -> UpdateResult {
        UpdateResult { pixel_visibility }
    }

    pub fn if_pixel_visible<F: FnOnce()>(self, func: F) {
        if self.pixel_visibility == PixelVisibility::Visible {
            func();
        }
    }

    pub fn unwrap(self) -> PixelVisibility {
        self.pixel_visibility
    }
}

#[derive(Debug, Clone)]
pub struct ZBuffer {
    z_buffer: GrayImage
}

impl ZBuffer {
    pub fn new(width: u32, height: u32) -> ZBuffer {
        ZBuffer {
            z_buffer: GrayImage::new(width, height)
        }
    }

    pub fn update_buffer(&mut self, x: u32, y: u32, z: u32) -> UpdateResult {
        if *&self.z_buffer[(x, y)].data[0] <= z as u8 {
            *&mut self.z_buffer[(x, y)] = Luma([z as u8]);
            UpdateResult::new(PixelVisibility::Visible)
        } else {
            UpdateResult::new(PixelVisibility::Hidden)
        }
    }

    pub fn unpack(self) -> GrayImage {
        self.z_buffer
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_create_zbuffer_from_dimensions() {
        let _z_buffer = ZBuffer::new(2, 2);
    }

    #[test]
    fn should_be_able_to_update_z_buffer() {
        let (x, y) = (1, 1);
        let mut z_buffer = ZBuffer::new(2, 2);
        assert_eq!(z_buffer.update_buffer(x, y, 1).unwrap(), PixelVisibility::Visible);
        assert_eq!(z_buffer.z_buffer[(1, 1)].data, [1]);
    }
}
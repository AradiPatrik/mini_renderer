use renderer_error::RendererError;
use wavefront_obj::obj::Vertex;
use cgmath::{Point3};

pub struct VertexCoordinateMapper {
    buffer_width: u32,
    buffer_height: u32,
}

impl VertexCoordinateMapper {
    pub fn new(buffer_width: u32, buffer_height: u32) -> Self {
        VertexCoordinateMapper {
            buffer_width,
            buffer_height,
        }
    }

    pub fn map_vertex_coords_to_pixel_coords(&self, v: &Vertex) -> Result<Point3<u32>, RendererError> {
        check_if_in_normalized_device_coordinates(v)?;
        Ok(Point3::new(
            ((v.x + 1.0) * (self.buffer_width - 1) as f64 / 2.0) as u32,
            ((v.y + 1.0) * (self.buffer_height - 1) as f64 / 2.0) as u32,
            ((v.z + 1.0) * 255.0 / 2.0).round() as u32
        ))
    }
}

fn check_if_in_normalized_device_coordinates(v: &Vertex) -> Result<(), RendererError> {
    if v.x > 1.0 || v.y > 1.0 || v.x < -1.0 || v.y < -1.0 || v.z > 1.0 || v.z < -1.0 {
        Err(RendererError::NotInNormalizedDeviceCoords(v.clone()))
    } else {
        Ok(())
    }
}
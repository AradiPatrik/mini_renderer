use cgmath::Point2;
use wavefront_obj::obj::Vertex;


#[derive(Debug, PartialEq)]
pub enum RendererError {
    PixelOutOfImageBounds(u32, u32, Point2<u32>),
    NotInNormalizedDeviceCoords(Vertex),
}

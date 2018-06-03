use cgmath::{Vector3};
use wavefront_obj::obj::Vertex;

pub trait VecFrom<T> {
    fn from_vertex(vertex: T) -> Self;
}

impl<'a> VecFrom<&'a Vertex> for Vector3<f64> {
    fn from_vertex(vert: &'a Vertex) -> Self {
        Vector3 {
            x: vert.x,
            y: vert.y,
            z: vert.z,
        }
    }
}

impl VecFrom<Vertex> for Vector3<f64> {
    fn from_vertex(vert: Vertex) -> Self {
        Vector3 {
            x: vert.x,
            y: vert.y,
            z: vert.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Vector3, VecFrom, Vertex};

    #[test]
    fn should_be_able_to_create_vector3_from_vertex_ref() {
        let vert = Vertex {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec = Vector3::from_vertex(&vert);
        assert_eq!(vert.x, vec.x);
        assert_eq!(vert.y, vec.y);
        assert_eq!(vert.z, vec.z);

    }

    #[test]
    fn should_be_able_to_create_vector3_from_vertex() {
        let vert = Vertex {
            x: 1.0,
            y: 2.0,
            z: 3.0
        };
        let vec = Vector3::from_vertex(vert);
        assert_eq!(vert.x, vec.x);
        assert_eq!(vert.y, vec.y);
        assert_eq!(vert.z, vec.z);
    }
}
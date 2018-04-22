extern crate image;
extern crate wavefront_obj;
extern crate image_writer;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use wavefront_obj::obj;
use wavefront_obj::obj::Primitive;
use image_writer::Renderer;
use image::{Rgb, ImageRgb8};


fn main() {
    let mut reader = BufReader::new(File::open("resources/african_head.obj").unwrap());
    let mut obj_file_text= String::new();
    reader.read_to_string(&mut obj_file_text).unwrap();
    let scene = obj::parse(obj_file_text).unwrap();
    let the_mesh = &scene.objects[0];
    let mut renderer = Renderer::new(800, 800);
    for material_group in &the_mesh.geometry {
        for shape in material_group.shapes.iter() {
            if let Primitive::Triangle((a_ind, ..), (b_ind, ..), (c_ind, ..))  = shape.primitive {
                let vertex_a = the_mesh.vertices[a_ind];
                let vertex_b = the_mesh.vertices[b_ind];
                let vertex_c = the_mesh.vertices[c_ind];
                renderer.triangle_2d(&vertex_a, &vertex_b, &vertex_c, Rgb([255, 255, 255])).unwrap();
            } else {
                panic!("Invalid obj format (line or point detected)");
            }
        }
    }

    let mut image_buffer = ImageRgb8(renderer.unpack());
    image_buffer = image_buffer.flipv();
    image_buffer.as_rgb8().unwrap().save("image.png").unwrap();

}


#![allow(dead_code)]

extern crate image;
extern crate wavefront_obj;
extern crate cgmath;
extern crate mini_renderer;
extern crate rand;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use wavefront_obj::obj;
use wavefront_obj::obj::Primitive;
use cgmath::Vector3;
use cgmath::prelude::*;
use mini_renderer::{Renderer, VecFrom};
use image::{Rgb, ImageRgb8};
use rand::prelude::*;

fn main() {
    draw_obj();
}

fn write_renderer(renderer: Renderer) {
    let mut image_buffer = ImageRgb8(renderer.unpack());
    image_buffer = image_buffer.flipv();
    image_buffer.as_rgb8().unwrap().save("image.png").unwrap();
}

fn draw_obj() {
    let mut reader = BufReader::new(File::open("resources/african_head.obj").unwrap());
    let mut obj_file_text = String::new();
    reader.read_to_string(&mut obj_file_text).unwrap();
    let scene = obj::parse(obj_file_text).unwrap();
    let the_mesh = &scene.objects[0];
    let mut renderer = Renderer::new(2500, 2500);
    renderer.clear_to_color(Rgb([50, 50, 50]));
    let mut _rng = thread_rng();
    let light_direction = Vector3::new(0.0, 0.0, 1.0);
    for material_group in &the_mesh.geometry {
        for shape in material_group.shapes.iter() {
            if let Primitive::Triangle((a_vert_ind, ..), (b_vert_ind, ..), (c_ind, _, ..)) = shape.primitive {
                let vertex_a = the_mesh.vertices[a_vert_ind];
                let vertex_b = the_mesh.vertices[b_vert_ind];
                let vertex_c = the_mesh.vertices[c_ind];
                let vec_a = Vector3::from_vertex(&vertex_a);
                let vec_b = Vector3::from_vertex(&vertex_b);
                let vec_c = Vector3::from_vertex(&vertex_c);
                let norm = (vec_a - vec_b).cross(vec_a - vec_c).normalize();
                let intensity = norm.dot(light_direction.clone());
                if intensity > 0.0 {
                    let rgb_value = (intensity * 255.0) as u8;
                    renderer.draw_filled_triangle_2d(&vertex_a, &vertex_b, &vertex_c, Rgb([rgb_value, rgb_value, rgb_value])).unwrap();
                }
            } else {
                panic!("Invalid obj format (line or point detected)");
            }
        }
    }
    let mut image_buffer = ImageRgb8(renderer.unpack());
    image_buffer = image_buffer.flipv();
    image_buffer.as_rgb8().unwrap().save("image.png").unwrap();
}

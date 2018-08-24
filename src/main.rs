#![allow(dead_code)]

extern crate cgmath;
extern crate image;
extern crate mini_renderer;
extern crate rand;
extern crate wavefront_obj;
extern crate minifb;

use cgmath::prelude::*;
use cgmath::Vector3;
use image::{ImageRgb8, ImageLuma8, Rgb};
use mini_renderer::{renderer::Renderer, outside_trait_impls::VecFrom};
use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use wavefront_obj::obj;
use wavefront_obj::obj::Primitive;
use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 16711935;
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}

fn draw_obj() {
    let mut reader = BufReader::new(File::open("resources/african_head.obj").unwrap());
    let mut obj_file_text = String::new();
    reader.read_to_string(&mut obj_file_text).unwrap();
    let scene = obj::parse(obj_file_text).unwrap();
    let the_mesh = &scene.objects[0];
    let mut renderer = Renderer::new(2500, 2500);
    renderer.clear_to_color(Rgb([0, 20, 25]));
    let mut _rng = thread_rng();
    let light_direction = Vector3::new(0.0, 0.0, 1.0);
    for material_group in &the_mesh.geometry {
        for shape in material_group.shapes.iter() {
            if let Primitive::Triangle((a_vert_ind, ..), (b_vert_ind, ..), (c_ind, _, ..)) =
                shape.primitive
            {
                let vertex_a = the_mesh.vertices[a_vert_ind];
                let vertex_b = the_mesh.vertices[b_vert_ind];
                let vertex_c = the_mesh.vertices[c_ind];
                let vec_a = Vector3::from_vertex(&vertex_a);
                let vec_b = Vector3::from_vertex(&vertex_b);
                let vec_c = Vector3::from_vertex(&vertex_c);
                let norm = (vec_a - vec_b).cross(vec_a - vec_c).normalize();
                let intensity = norm.dot(light_direction.clone());
                if intensity > 0.0 {
                    let rgb_value = (intensity * 85.0) as u8;
                    renderer
                        .draw_filled_triangle_2d(
                            &vertex_a,
                            &vertex_b,
                            &vertex_c,
                            Rgb([rgb_value, rgb_value, rgb_value]),
                        )
                        .unwrap();
                }
                renderer
                    .draw_triangle_2d(
                        &vertex_a,
                        &vertex_b,
                        &vertex_c,
                        Rgb([230, 240, 250]),
                    )
                    .unwrap();
            } else {
                panic!("Invalid obj format (line or point detected)");
            }
        }
    }
    let  (mut image_buffer, mut z_buffer) = renderer.unpack();
    let image_buffer = ImageRgb8(image_buffer).flipv();

    image_buffer.as_rgb8().unwrap().save("image.png").unwrap();
    let z_buffer = ImageLuma8(z_buffer.unpack()).flipv();
    z_buffer.as_luma8().unwrap().save("image_z.png").unwrap();
}

#![allow(dead_code)]

extern crate image;
extern crate wavefront_obj;
extern crate image_writer;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use wavefront_obj::obj;
use wavefront_obj::obj::Primitive;
use image_writer::{Renderer, vector::Vector2};
use image::{Rgb, ImageRgb8};
use image_writer::lerp;


fn draw_filled_triangle(renderer: &mut Renderer, point_a: Vector2<u32>, point_b: Vector2<u32>, point_c: Vector2<u32>) {
    let mut points = vec![point_a, point_b, point_c];
    points.sort_by(|point_a, point_b| (point_a.y).cmp(&(point_b.y)));
    for y in points[0].y..points[2].y + 1 {
        let x_left = if y <= points[1].y {
            let left_lerp_amount = (y - points[0].y) as f64 / (points[1].y - points[0].y) as f64;
            lerp(points[0].x, points[1].x, left_lerp_amount)
        } else {
            let left_lerp_amount = (y - points[1].y) as f64 / (points[2].y - points[1].y) as f64;
            lerp(points[1].x, points[2].x, left_lerp_amount)
        };
        let right_lerp_amount = (y - points[0].y) as f64 / (points[2].y - points[0].y) as f64;
        let x_right = lerp(points[0].x, points[2].x, right_lerp_amount);
        renderer.line(Vector2::new(x_left, y), Vector2::new(x_right, y), Rgb([60, 10, 150])).unwrap();
    }
}

fn draw_triangle(renderer: &mut Renderer, point_a: &Vector2<u32>, point_b: &Vector2<u32>, point_c: &Vector2<u32>, col: &Rgb<u8>) {
    renderer.line(point_a.clone(), point_b.clone(), col.clone()).unwrap();
    renderer.line(point_b.clone(), point_c.clone(), col.clone()).unwrap();
    renderer.line(point_c.clone(), point_a.clone(), col.clone()).unwrap();
}

fn main() {
    // let mut renderer = Renderer::new(500, 500);
    // let red = Rgb([255, 0, 0]);
    // let green = Rgb([0, 255, 0]);
    // let blue = Rgb([0, 0, 255]);
    // let white = Rgb([255, 255, 255]);
    // let tri_a = (Vector2::new(10, 70), Vector2::new(50, 160), Vector2::new(70, 80));
    // let tri_b = (Vector2::new(180, 50), Vector2::new(150, 1), Vector2::new(70, 180));
    // let tri_c = (Vector2::new(180, 150), Vector2::new(120, 160), Vector2::new(130, 180));
    // draw_triangle(&mut renderer, &tri_a.0, &tri_a.1, &tri_a.2, &red);
    // draw_triangle(&mut renderer, &tri_b.0, &tri_b.1, &tri_b.2, &white);
    // draw_triangle(&mut renderer, &tri_c.0, &tri_c.1, &tri_c.2, &green);
    // draw_filled_triangle(&mut renderer, tri_a.0, tri_a.1, tri_a.2);
    // draw_filled_triangle(&mut renderer, tri_b.0, tri_b.1, tri_b.2);
    // draw_filled_triangle(&mut renderer, tri_c.0, tri_c.1, tri_c.2);

    draw_obj();

    // write_renderer(renderer);

}

fn write_renderer(renderer: Renderer) {
    let mut image_buffer = ImageRgb8(renderer.unpack());
    image_buffer = image_buffer.flipv();
    image_buffer.as_rgb8().unwrap().save("image.png").unwrap();
}

fn draw_obj() {
    let mut reader = BufReader::new(File::open("resources/diablo.obj").unwrap());
    let mut obj_file_text = String::new();
    reader.read_to_string(&mut obj_file_text).unwrap();
    let scene = obj::parse(obj_file_text).unwrap();
    let the_mesh = &scene.objects[0];
    let mut renderer = Renderer::new(800, 800);
    for material_group in &the_mesh.geometry {
        for shape in material_group.shapes.iter() {
            if let Primitive::Triangle((a_ind, ..), (b_ind, ..), (c_ind, ..)) = shape.primitive {
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

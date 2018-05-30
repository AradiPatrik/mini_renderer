#![allow(dead_code)]

extern crate image;
extern crate wavefront_obj;
extern crate cgmath;
extern crate image_writer;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use wavefront_obj::obj;
use wavefront_obj::obj::Primitive;
use image_writer::{Renderer, Triangle2};
use image::{Rgb, ImageRgb8};
use image_writer::lerp;
use cgmath::Point2;


fn draw_filled_triangle(renderer: &mut Renderer, point_a: Point2<u32>, point_b: Point2<u32>, point_c: Point2<u32>) {
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
        renderer.draw_line(Point2::new(x_left, y), Point2::new(x_right, y), Rgb([60, 10, 150])).unwrap();
    }
}

fn draw_optimized_filled_triangle(renderer: &mut Renderer, triangle: &Triangle2<u32>) {
    let bounding_box = triangle.get_bounding_box();
    for x in bounding_box.0.x ..= bounding_box.1.x {
        for y in bounding_box.0.y ..= bounding_box.1.y {
            if triangle.is_inside_point(Point2::new(x, y).clone()) {
                renderer.draw_point(&Point2::new(x, y), Rgb([128, 180, 50])).unwrap();
            }
        }
    }
}

fn draw_triangle(renderer: &mut Renderer, point_a: &Point2<u32>, point_b: &Point2<u32>, point_c: &Point2<u32>, col: &Rgb<u8>) {
    renderer.draw_line(point_a.clone(), point_b.clone(), col.clone()).unwrap();
    renderer.draw_line(point_b.clone(), point_c.clone(), col.clone()).unwrap();
    renderer.draw_line(point_c.clone(), point_a.clone(), col.clone()).unwrap();
}

fn main() {
    let mut renderer = Renderer::new(500, 500);
    let tri_a = (Point2::new(10, 70), Point2::new(50, 160), Point2::new(70, 80));
    let tri_b = (Point2::new(180, 50), Point2::new(150, 1), Point2::new(70, 180));
    let tri_c = (Point2::new(180, 150), Point2::new(120, 160), Point2::new(130, 180));
    draw_optimized_filled_triangle(&mut renderer, &Triangle2::new(tri_a.0, tri_a.1, tri_a.2));
    draw_optimized_filled_triangle(&mut renderer, &Triangle2::new(tri_b.0, tri_b.1, tri_b.2));
    draw_optimized_filled_triangle(&mut renderer, &Triangle2::new(tri_c.0, tri_c.1, tri_c.2));

    // draw_obj();

    write_renderer(renderer);

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
                panic!("Invalid obj format (draw_line or point detected)");
            }
        }
    }
    let mut image_buffer = ImageRgb8(renderer.unpack());
    image_buffer = image_buffer.flipv();
    image_buffer.as_rgb8().unwrap().save("image.png").unwrap();
}

extern crate wavefront_obj;
use wavefront_obj::obj;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use wavefront_obj::obj::Primitive;

#[test]
fn should_be_able_to_load_cube() {
    let mut reader = BufReader::new(File::open("resources/cube.obj").unwrap());
    let mut cube_source= String::new();
    reader.read_to_string(&mut cube_source);
    let cube_obj = obj::parse(cube_source).unwrap();
    let cube = &cube_obj.objects[0];
    for geometry in &cube.geometry {
        for shape in geometry.shapes.iter() {
            match shape.primitive {
                Primitive::Point(_) => { println!("it's a point")},
                Primitive::Line(_, _) => { println!("it's a line")},
                Primitive::Triangle(_, _, _) => { },
            }
        }
    }
}
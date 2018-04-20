extern crate image;
use image::*;

#[test]
fn should_annotate_pixel_buffer() {
    let image: RgbImage = ImageBuffer::new(2, 2);
    assert_eq!(image[(0, 0)], Rgb([0, 0, 0]));
    assert_eq!(image[(1, 0)], Rgb([0, 0, 0]));
    assert_eq!(image[(0, 1)], Rgb([0, 0, 0]));
    assert_eq!(image[(1, 1)], Rgb([0, 0, 0]));
}

#[test]
fn pixel_creation() {
    let x = Rgb([1, 2, 3]);
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 2);
    assert_eq!(x[2], 3);
}

#[should_panic]
#[test]
fn over_indexing_a_pixel_should_panic() {
    let x = Rgb([1,2,3]);
    x[3];
}

#[test]
fn can_iterate_over_pixels() {
    let image: RgbImage = ImageBuffer::new(3, 3);
    for pixel_ref in image.pixels() {
        assert_eq!(Rgb([0, 0, 0]), *pixel_ref);
    }
}
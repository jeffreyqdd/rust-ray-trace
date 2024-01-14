use image::*;
use ray_tracer::image;
use std::fs::File;
use std::path::Path;

// Unfortunately, these tests will have to be visually inspected
// I don't want to write an image decoder only for the sake of testing

#[test]
fn generate_1_pixel_image_red() {
    let mut frame = Frame::new(1, 1, 3);
    frame.write_color(0, 0, Color::new_rgb(1., 0., 0.));

    let file = File::create(Path::new(r"tests/image_test/single_red_pixel.png")).unwrap();
    frame.write_image(file, ImageType::Rgb8bit, false);
}

#[test]
fn generate_1_pixel_image_green() {
    let mut frame = Frame::new(1, 1, 3);
    frame.write_color(0, 0, Color::new_rgb(0., 1., 0.));

    let file = File::create(Path::new(r"tests/image_test/single_green_pixel.png")).unwrap();
    frame.write_image(file, ImageType::Rgb8bit, false);
}

#[test]
fn generate_1_pixel_image_blue() {
    let mut frame = Frame::new(1, 1, 3);
    frame.write_color(0, 0, Color::new_rgb(0., 0., 1.));

    let file = File::create(Path::new(r"tests/image_test/single_blue_pixel.png")).unwrap();
    frame.write_image(file, ImageType::Rgb8bit, false);
}

#[test]
fn red_green_gradient() {
    let img_width = 1000;
    let img_height = 1000;
    let mut frame = Frame::new(img_width, img_height, 3);

    for r in 0..img_height {
        for c in 0..img_width {
            let red_gain = r as f64 / (img_width - 1) as f64;
            let green_gain = c as f64 / (img_height - 1) as f64;
            let blue_gain = 0.;
            frame.write_color(r, c, Color::new_rgb(red_gain, green_gain, blue_gain));
        }
    }
    let file = File::create(Path::new(r"tests/image_test/red_green_gradient.png")).unwrap();
    frame.write_image(file, ImageType::Rgb8bit, false);
}

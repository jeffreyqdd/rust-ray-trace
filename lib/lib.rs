pub mod camera;
pub mod common;
pub mod geometry;
pub mod illumination;
pub mod image;

pub const TRACE_EPSILON: f64 = 1e-14;

use image::{Color, Frame};
use camera::Camera;
use common::{Ray, IntersectResult, Scene};
// use illumination::Illuminate;


fn shade(ray : Ray, scene : &Scene, depth : u32) -> Color {
    if depth == 0 {
        return Color::new_rgb(0., 0., 0.);
    }

    match scene.intersect(&ray) {
        IntersectResult::Hit { t: _, point, normal, material: _ } => {
            // should always be a hit
            let new_direction = nalgebra::Vector3::new_random().normalize() + normal.into_inner();
            return shade(Ray::new(point + new_direction * 1e-8, new_direction), scene, depth - 1) * 0.5;
        },
        IntersectResult::Miss => {
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            return (Color::new_rgb(1., 1., 1.) * (1. - a)) + (Color::new_rgb(0.5, 0.7, 1.) * a);
        }
    };
}

pub fn render_frame(camera : Box<dyn Camera>, scene : Scene, width_px : usize, height_px : usize) -> Frame {
    let mut frame_buffer = Frame::new(width_px, height_px, 3);

    for w in 0..width_px {
        for h in 0..height_px {
            let x1 : f64 = (w as f64 + 0.25) / (width_px as f64);
            let y1 : f64 = (h as f64 + 0.25) / (height_px as f64);

            let x2 : f64 = (w as f64 + 0.75) / (width_px as f64);
            let y2 : f64 = (h as f64 + 0.25) / (height_px as f64);

            let x3 : f64 = (w as f64 + 0.75) / (width_px as f64);
            let y3 : f64 = (h as f64 + 0.75) / (height_px as f64);

            let x4 : f64 = (w as f64 + 0.25) / (width_px as f64);
            let y4 : f64 = (h as f64 + 0.75) / (height_px as f64);

            let mut color1 = shade(camera.generate_ray(x1, y1), &scene, 20);
            let mut color2 = shade(camera.generate_ray(x2, y2), &scene, 20);
            let mut color3 = shade(camera.generate_ray(x3, y3), &scene, 20);
            let mut color4 = shade(camera.generate_ray(x4, y4), &scene, 20);

            color1.clamp();
            color2.clamp();
            color3.clamp();
            color4.clamp();

            let result = (color1 + color2 + color3 + color4) / 4.;
            println!("{}", result);
            frame_buffer.write_color(w, h, result);
            // match scene.intersect(&ray) {
            //     common::IntersectResult::Hit { .. } => frame_buffer.write_color(w, h, image::Color::new_rgb(1., 1., 1.)),
            //     common::IntersectResult::Miss => frame_buffer.write_color(w, h, image::Color::new_rgb(0., 0., 0.)),
            // };
        }
    }

    frame_buffer
}

pub mod camera;
pub mod common;
pub mod geometry;
pub mod illumination;
pub mod image;

pub const TRACE_EPSILON: f64 = 1e-14;

use camera::Camera;
use common::{IntersectResult, Ray, Scene};
use illumination::Illuminate;
use image::{Color, Frame};
// use illumination::Illuminate;

fn shade(ray: Ray, lights: &Vec<Box<dyn Illuminate>>, scene: &Scene, depth: u32) -> Color {
    // let (color, hit) = scene.shade(&ray);
    // shoot ray into scene and get intersection
    // color the hit/miss according to illumination of the material
    // bounce rays according to the material properties
    let hit_result = scene.intersect(&ray);
    let mut reflected_light: Color = Color::black_rgb();

    // if depth limit has not been reached, we shoot the reflected light off into the scene
    // if we're able to, we want to reflect the ray and shoot it off into the scene again.
    if depth > 0 {}
    // illuminate current pixel from current hit
    match hit_result {
        IntersectResult::Hit {
            t: _,
            point,
            normal,
            material,
        } => {
            for light in lights {
                reflected_light += light.illuminate(&ray, &point, &normal, &material);
            }
            reflected_light.clamp();
            reflected_light
        }
        IntersectResult::Miss => reflected_light,
    }
}

// pub fn shade(&self, ray: &Ray) -> (Color, IntersectResult) {
//     let hit = self.intersect(ray);
//     let mut result = Color::new_rgb(0., 0., 0.);
//     for light in &self.lights {
//         result += light.illuminate(ray, &hit);
//     }
//     (result, hit)
// }
pub fn render_frame(
    camera: Box<dyn Camera>,
    lights: Vec<Box<dyn Illuminate>>,
    scene: Scene,
    width_px: usize,
    height_px: usize,
) -> Frame {
    let mut frame_buffer = Frame::new(width_px, height_px, 3);

    for w in 0..width_px {
        for h in 0..height_px {
            let x1: f64 = (w as f64 + 0.25) / (width_px as f64);
            let y1: f64 = (h as f64 + 0.25) / (height_px as f64);

            let x2: f64 = (w as f64 + 0.75) / (width_px as f64);
            let y2: f64 = (h as f64 + 0.25) / (height_px as f64);

            let x3: f64 = (w as f64 + 0.75) / (width_px as f64);
            let y3: f64 = (h as f64 + 0.75) / (height_px as f64);

            let x4: f64 = (w as f64 + 0.25) / (width_px as f64);
            let y4: f64 = (h as f64 + 0.75) / (height_px as f64);

            let mut color1 = shade(camera.generate_ray(x1, y1), &lights, &scene, 20);
            let mut color2 = shade(camera.generate_ray(x2, y2), &lights, &scene, 20);
            let mut color3 = shade(camera.generate_ray(x3, y3), &lights, &scene, 20);
            let mut color4 = shade(camera.generate_ray(x4, y4), &lights, &scene, 20);

            color1.clamp();
            color2.clamp();
            color3.clamp();
            color4.clamp();

            let result = (color1 + color2 + color3 + color4) / 4.;
            frame_buffer.write_color(w, h, result);
            // match scene.intersect(&ray) {
            //     common::IntersectResult::Hit { .. } => frame_buffer.write_color(w, h, image::Color::new_rgb(1., 1., 1.)),
            //     common::IntersectResult::Miss => frame_buffer.write_color(w, h, image::Color::new_rgb(0., 0., 0.)),
            // };
        }
    }

    frame_buffer
}

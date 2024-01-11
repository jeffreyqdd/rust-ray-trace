#![allow(unused_imports)]
use camera::{Camera, PerspectiveCamera};
use common::{IntersectResult, Scene, Material};
use geometry::{Plane, Sphere, Surface};
use nalgebra::{point, vector, UnitVector3};
use ray_tracer::*;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let camera = PerspectiveCamera::new(
        1.,
        point![3., 0., 0.],
        point![-1., 0., 0.,],
        vector![0., 1., 0.],
        25.,
        16. / 9.,
    );

    // create scene
    let sphere1 = Sphere::new(point![-1., 0., 0.], 0.5, Material::none());
    let sphere2 = Sphere::new(point![0., -100.5, 0.], 100., Material::none());
    let scene = Scene::new(vec![Box::new(sphere1), Box::new(sphere2)], vec![]);

    // render image
    let img = ray_tracer::render_frame(Box::new(camera), scene, 256, 144);

    // export image
    let path = Path::new(r"stretched_norm.png");
    let file = File::create(path).unwrap();
    img.write_image(file, image::ImageType::Rgb16bit, None, None);
}

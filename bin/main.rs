#![allow(unused_imports)]
use camera::{Camera, PerspectiveCamera};
use common::{IllumModel, IntersectResult, Material, Scene};
use geometry::{Plane, Sphere, Surface};
use illumination::PointLight;
use image::Color;
use nalgebra::{point, vector, UnitVector3};
use png::ScaledFloat;
use ray_tracer::illumination::AmbientLight;
use ray_tracer::*;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let camera = PerspectiveCamera::new(
        1.,
        point![3., 1.2, 5.],
        point![0., -0.4, 0.,],
        vector![0., 1., 0.],
        24.,
        16. / 9.,
    );

    // tan color
    let tan = Material {
        k_a: Color::new_rgb(0.4, 0.4, 0.2),
        k_d: Color::new_rgb(0.4, 0.4, 0.2),
        k_s: Color::new_rgb(0.3, 0.3, 0.3),
        ns: 90.,
        ni: 0.,
        d: 1.,
        illum: IllumModel::Constant,
    };

    // blue color
    let blue = Material {
        k_a: Color::new_rgb(0.2, 0.2, 0.5),
        k_d: Color::new_rgb(0.2, 0.2, 0.5),
        k_s: Color::new_rgb(0., 0., 0.),
        ns: 20.,
        ni: 0.,
        d: 1.,
        illum: IllumModel::Constant,
    };

    // gray color
    let gray = Material {
        k_a: Color::new_rgb(0.2, 0.2, 0.2),
        k_d: Color::new_rgb(0.2, 0.2, 0.2),
        k_s: Color::new_rgb(0., 0., 0.),
        ns: 20.,
        ni: 0.,
        d: 1.,
        illum: IllumModel::Constant,
    };

    // create scene
    let sphere1 = Sphere::new(point![-0.7, 0., 0.], 0.5, tan);
    let sphere2 = Sphere::new(point![0.7, 0., 0.], 0.5, blue);
    let sphere3 = Sphere::new(point![0., -40., 0.], 39.5, gray);
    let scene = Scene::new(vec![
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
    ]);

    // render image
    let img = ray_tracer::render_frame(
        Box::new(camera),
        vec![
            Box::new(PointLight::new(
                point![12., 10., 5.],
                Color::new_rgb(300., 300., 300.),
            )),
            Box::new(AmbientLight::new(Color::new_rgb(0.1, 0.1, 0.1))),
        ],
        scene,
        256,
        144,
    );

    // export image
    let path = Path::new(r"output/lambertian_blinn_phong_cc.png");
    let file = File::create(path).unwrap();
    img.write_image(file, image::ImageType::Rgb16bit, true);
}

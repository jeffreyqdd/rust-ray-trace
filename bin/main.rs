use camera::{Camera, PerspectiveCamera};
use common::{IntersectResult, Scene};
use geometry::{Plane, Sphere, Surface};
use nalgebra::{point, vector, UnitVector3};
use ray_tracer::*;

fn main() {
    let camera = PerspectiveCamera::new(
        1.,
        point![3., 0., 0.],
        point![-1., 0., 0.,],
        vector![0., 1., 0.],
        25.,
        16. / 9.,
    );

    let image_width = 256;
    let image_height = 144;
    print!("P3\n{} {}\n{}\n", image_width, image_height, 255);

    // create scene
    let sphere1 = Sphere::new(point![-1., 0., 0.], 0.5);
    let sphere2 = Sphere::new(point![0., -100.5, 0.], 100.);
    let scene = Scene::new(vec![Box::new(sphere1), Box::new(sphere2)]);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = j as f64;
            let c = i as f64;

            let ray = camera.generate_ray(
                (c + 0.5) / (image_width as f64),
                (r + 0.5) / (image_height as f64),
            );

            // x y z
            // x z y

            // y x z
            // y z x

            // z x y
            // z y x
            match scene.intersect(&ray) {
                IntersectResult::Hit { t, point, normal } => {
                    println!(
                        "{} {} {}",
                        ((normal.z * 255. + 255.) * 0.5) as u8,
                        ((normal.y * 255. + 255.) * 0.5) as u8,
                        ((normal.x * 255. + 255.) * 0.5) as u8
                    );
                }
                IntersectResult::Miss => println!("0 0 0"),
            }
        }
    }
}

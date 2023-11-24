use camera::{Camera, PerspectiveCamera};
use common::IntersectResult;
use geometry::{Plane, Surface};
use nalgebra::{point, vector, UnitVector3};
use ray_tracer::*;

fn main() {
    let camera = PerspectiveCamera::new(
        1.,
        point![0., 0., 0.],
        point![1., 0., 0.,],
        vector![0., 1., 0.],
        25.,
        16. / 9.,
    );

    let plane = Plane::new(
        point![0., -20., 0.,],
        UnitVector3::new_normalize(vector![0.1, 1., 0.2]),
    );

    // println!("{:#?}", plane);
    // println!("{:#?}", plane);

    let image_width = 560;
    let image_height = 560;
    print!("P3\n{} {}\n{}\n", image_width, image_height, 255);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = j as f64;
            let c = i as f64;

            let ray = camera.generate_ray(
                (c + 0.5) / (image_width as f64),
                (r + 0.5) / (image_height as f64),
            );
            let hit = plane.intersect(&ray);

            match hit {
                IntersectResult::Hit { point, .. } => {
                    let scale = 0.01;
                    let pattern = (((scale * f64::abs(point.x)) % 1.) > 0.5)
                        ^ (((scale * f64::abs(point.z)) % 1.) > 0.5);
                    if pattern {
                        println!("255 255 255");
                    } else {
                        println!("55 55 55");
                    }
                }
                _ => {
                    println!("0 0 0")
                }
            };
        }
    }
}

use common::*;
use geometry::*;
use ray_tracer::*;

use nalgebra::{point, vector, UnitVector3};

#[test]
fn sphere_test_unit_simple() {
    let sphere = Sphere::new(point![0., 0., 0.], 1.);
    let ray = Ray::new(point![0., -2., 0.], vector![0., 1., 0.,]);

    // dead center hit
    assert_eq!(
        sphere.intersect(&ray),
        IntersectResult::Hit {
            t: 1.,
            point: point![0., -1., 0.],
            normal: UnitVector3::new_normalize(vector![0., -1., 0.,])
        }
    );

    // off center hit
    let ray = Ray::new(point![1., 0.5, 0.], vector![-1., 0., 0.,]);
    let expected_t = 1. - f64::sin(std::f64::consts::PI / 3.);
    let hit_point = ray.at(expected_t);
    assert_eq!(
        sphere.intersect(&ray),
        IntersectResult::Hit {
            t: expected_t,
            point: hit_point,
            normal: UnitVector3::new_normalize(hit_point - point![0., 0., 0.])
        }
    );

    // dead center, start inside
    let ray = Ray::new(point![0., 0., 0.], vector![-1., 0., 0.,]);
    assert_eq!(
        sphere.intersect(&ray),
        IntersectResult::Hit {
            t: 1.,
            point: point![-1., 0., 0.],
            normal: UnitVector3::new_normalize(vector![1., 0., 0.])
        }
    );
}

#[test]
fn sphere_test_unit_misses() {
    let sphere = Sphere::new(point![0., 0., 0.], 1.);

    // on axis miss
    let ray = Ray::new(point![2., 3., 0.], vector![-1., 0., 0.]);
    assert_eq!(sphere.intersect(&ray), IntersectResult::Miss);

    // off axis miss
    let ray = Ray::new(point![2., 3., 4.], vector![-1., -1., -1.]);
    assert_eq!(sphere.intersect(&ray), IntersectResult::Miss);

    // dead center miss, starts past sphere
    let ray = Ray::new(point![-1.01, 0., 0.], vector![-1., 0., 0.]);
    assert_eq!(sphere.intersect(&ray), IntersectResult::Miss);
}

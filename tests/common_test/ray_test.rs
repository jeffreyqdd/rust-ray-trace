use common::Ray;
use nalgebra::{point, vector};
use ray_tracer::*;

#[test]
fn test_ray_parametric_simple() {
    let ray = Ray::new(point![0., 0., 0.], vector![1., 0., 0.]);
    assert!(ray.at(0.) == point![0., 0., 0.]);
    assert!(ray.at(0.3) == point![0.3, 0., 0.]);
    assert!(ray.at(2.) == point![2., 0., 0.]);
    assert!(ray.at(-0.3) == point![-0.3, 0., 0.]);
    assert!(ray.at(-2.) == point![-2., 0., 0.]);

    let ray = Ray::new(point![0., 0., 0.], vector![0., 1., 0.]);
    assert!(ray.at(0.) == point![0., 0., 0.]);
    assert!(ray.at(0.3) == point![0., 0.3, 0.]);
    assert!(ray.at(2.) == point![0., 2., 0.]);
    assert!(ray.at(-0.3) == point![0., -0.3, 0.]);
    assert!(ray.at(-2.) == point![0., -2., 0.]);

    let ray = Ray::new(point![0., 0., 0.], vector![0., 0., 1.]);
    assert!(ray.at(0.) == point![0., 0., 0.]);
    assert!(ray.at(0.3) == point![0., 0., 0.3]);
    assert!(ray.at(2.) == point![0., 0., 2.]);
    assert!(ray.at(-0.3) == point![0., 0., -0.3]);
    assert!(ray.at(-2.) == point![0., 0., -2.]);
}

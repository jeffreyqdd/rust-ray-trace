use common::*;
use geometry::*;
use ray_tracer::*;

use nalgebra::{point, vector, UnitVector3};

#[test]
fn sphere_test_unit_simple() {
    let sphere = Sphere::new(point![0., 0., 0.], 1.);
    let ray = Ray::new(point![0., -2., 0.], vector![0., 1., 0.,]);

    assert_eq!(
        sphere.intersect(&ray),
        IntersectResult::Hit {
            t: 1.,
            point: point![0., -1., 0.],
            normal: UnitVector3::new_normalize(vector![0., -1., 0.,])
        }
    );
}

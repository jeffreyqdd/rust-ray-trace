use common::*;
use geometry::*;
use ray_tracer::*;
    
use nalgebra::{point, vector, UnitVector3};

#[test]
fn plane_test_perpendicular_from_front() {
    let plane = Plane::new(
        point![0., 0., 0.],
        UnitVector3::new_normalize(vector![0., 1., 0.]),
        Material::none()
    );
    let ray = Ray::new(point![0., 1., 0.], vector![0., -1.0, 0.]);
    let res = plane.intersect(&ray);
    assert_eq!(
        res,
        IntersectResult::Hit {
            t: 1.,
            point: point![0., 0., 0.,],
            normal: UnitVector3::new_normalize(vector![0., 1., 0.]),
            material: Material::none()
        }
    )
}

#[test]
fn plane_test_perpendicular_from_front_offcenter() {
    let plane = Plane::new(
        point![0., 0., 0.],
        UnitVector3::new_normalize(vector![0., 1., 0.]),
        Material::none()
    );
    let ray = Ray::new(point![-2., 1., 2.], vector![0., -2.0, 0.]);
    let res = plane.intersect(&ray);
    assert_eq!(
        res,
        IntersectResult::Hit {
            t: 0.5,
            point: point![-2., 0., 2.,],
            normal: UnitVector3::new_normalize(vector![0., 1., 0.]),
            material: Material::none()
        }
    )
}

#[test]
fn plane_test_perpendicular_from_back() {
    let plane = Plane::new(
        point![0., 0., 0.],
        UnitVector3::new_normalize(vector![0., 1., 0.]),
        Material::none()
    );
    let ray = Ray::new(point![1., -1., 2.], vector![0., 1.0, 0.]);
    let res = plane.intersect(&ray);
    assert_eq!(
        res,
        IntersectResult::Hit {
            t: 1.0,
            point: point![1., 0., 2.,],
            normal: UnitVector3::new_normalize(vector![0., -1., 0.]),
            material: Material::none()
        }
    );
}

#[test]
fn plane_test_parallel() {
    let plane = Plane::new(
        point![0., 0., 0.],
        UnitVector3::new_normalize(vector![0., 1., 0.]),
        Material::none()
    );
    let ray = Ray::new(point![1., -1., 2.], vector![1., 0., 0.]);
    let res = plane.intersect(&ray);
    assert_eq!(res, IntersectResult::Miss);
}

#[test]
fn plane_angled() {
    let plane = Plane::new(
        point![0., 0., 1.],
        UnitVector3::new_normalize(vector![3., 6., 10.]),
        Material::none(),
    );
    let ray = Ray::new(point![-20., -20., 7.], vector![1.4, 0., 0.]);
    let res = plane.intersect(&ray);

    assert_eq!(
        res,
        IntersectResult::Hit {
            t: 28.571428571428573,
            point: point![20., -20., 7.],
            normal: UnitVector3::new_normalize(-vector![3., 6., 10.]),
            material : Material::none()
        },
    );
}

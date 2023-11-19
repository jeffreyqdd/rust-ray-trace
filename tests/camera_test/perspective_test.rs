use camera::*;
use common::*;
use ray_tracer::{camera, common};

use approx::assert_relative_eq;
use nalgebra::{point, vector, Point3, Vector3};

#[test]
fn camera_placement_simple() {
    let camera = PerspectiveCamera::new(
        1.,
        point![3., 1.7, 5.],
        point![0., 0., 0.],
        vector![0., 1., 0.],
        25.,
        16. / 9.,
    );

    assert_relative_eq!(
        camera.generate_ray(0., 0.),
        Ray::new(
            Point3::new(2.1361848069990015, 1.632939039199566, 4.3263470713479295),
            Vector3::new(
                -0.8638151930009986,
                -0.06706096080043403,
                -0.6736529286520705
            )
        )
    );

    assert_relative_eq!(
        camera.generate_ray(0.24398, 0.984376),
        Ray::new(
            Point3::new(2.363947504120175, 1.2139222924002162, 4.332155146987004),
            Vector3::new(
                -0.6360524958798252,
                -0.48607770759978375,
                -0.6678448530129957
            )
        )
    );
}

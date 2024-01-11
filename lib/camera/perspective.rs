use crate::camera::Camera;
use crate::common::Ray;
use nalgebra::{Matrix4, Point3, Transform3, Unit, UnitVector3, Vector3};

#[derive(Debug, PartialEq)]
pub struct PerspectiveCamera {
    /// distance from image plane
    f: f64,

    /// height of the slit on the image plane
    height: f64,

    /// width of the slit on the image plane
    width: f64,

    /// position of camera in 3D space
    eye: Point3<f64>,

    // transformation from camera to world coordinates
    m: Transform3<f64>,
}

impl PerspectiveCamera {
    /// Parameters
    ///     f       distance from image plane
    ///     eye     camera's position in 3d space
    ///     target  point in 3d space the camera center is aligned at
    ///     up      3D vector that appears straight up in the camera's view
    ///     fov     field of view in degrees
    ///     aspect  aspect ratio of camera's view
    pub fn new(
        f: f64,
        eye: Point3<f64>,
        target: Point3<f64>,
        up: Vector3<f64>,
        fov: f64,
        aspect: f64,
    ) -> PerspectiveCamera {
        // calculate the width and height of image plane from fov and aspect
        let fov_rad = f64::to_radians(fov);
        let height = 2. * f * f64::tan(fov_rad / 2.);
        let width = height * aspect;

        // note that the vector from eye to target is the -z direction, so we want -(target - eye)
        // forward
        let z_vec: Unit<Vector3<f64>> = UnitVector3::new_normalize(eye - target);

        // right
        let x_vec: Unit<Vector3<f64>> = UnitVector3::new_normalize(up.cross(&z_vec));

        // up
        let y_vec: Vector3<f64> = z_vec.cross(&x_vec);

        let mut m = Matrix4::identity();

        m.set_column(0, &x_vec.to_homogeneous());
        m.set_column(1, &y_vec.to_homogeneous());
        m.set_column(2, &z_vec.to_homogeneous());
        let m = Transform3::from_matrix_unchecked(m);

        PerspectiveCamera {
            f,
            height,
            width,
            eye,
            m,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, x: f64, y: f64) -> Ray {
        // returns a ray whose direction is not necessarily normalized
        assert!(0. <= x && x <= 1.);
        assert!(0. <= y && y <= 1.);

        let img_x = self.width * (x - 0.5);
        let img_y = self.height * (y - 0.5);

        // have the ray direction travel in the negative z
        let direction: Vector3<f64> = self.m * Vector3::new(img_x, -img_y, -self.f);

        // t = 1 is the point where the ray intersections with the image plane
        // so we offset eye by an amount direction.
        Ray::new(self.eye + direction, direction)
    }
}

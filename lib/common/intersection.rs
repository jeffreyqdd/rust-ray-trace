use nalgebra::{Point3, Unit, Vector3};

use super::Material;

#[derive(Debug, PartialEq)]
pub enum IntersectResult {
    Hit {
        t: f64,
        point: Point3<f64>,
        normal: Unit<Vector3<f64>>,
        material : Material
    },
    Miss
}

impl IntersectResult {
    pub fn new_hit(
        t: f64,
        point: Point3<f64>,
        material : Material,
        outward_normal: Unit<Vector3<f64>>,
        ray_direction: &Vector3<f64>,
    ) -> IntersectResult {
        // TODO: figure out what this if statement does
        let normal = if outward_normal.dot(ray_direction) < 0. {
            outward_normal
        } else {
            -outward_normal
        };

        IntersectResult::Hit { t, point, normal, material}
    }
}

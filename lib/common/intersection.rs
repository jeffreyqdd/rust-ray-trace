use nalgebra::{Point3, Unit, Vector3};

#[derive(Debug, PartialEq)]
pub enum IntersectResult {
    Hit {
        t: f64,
        point: Point3<f64>,
        normal: Unit<Vector3<f64>>,
    },
    Miss,
}

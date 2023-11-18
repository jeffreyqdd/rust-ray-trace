use nalgebra::{Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
    pub start: f64,
    pub end: f64,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction,
            start: 0.0,
            end: f64::INFINITY,
        }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}

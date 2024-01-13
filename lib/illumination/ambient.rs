use nalgebra::{Point3, UnitVector3};

use crate::{
    common::{Material, Ray},
    image::Color,
};

use super::Illuminate;

pub struct AmbientLight {
    intensity: Color,
}
impl AmbientLight {
    pub fn new(intensity: Color) -> Self {
        Self { intensity }
    }
}

impl Illuminate for AmbientLight {
    fn illuminate(
        &self,
        _ray: &Ray,
        _point: &Point3<f64>,
        _normal: &UnitVector3<f64>,
        material: &Material,
    ) -> crate::image::Color {
        self.intensity * material.k_a
    }
}

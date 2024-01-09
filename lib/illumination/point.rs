use nalgebra::Vector3;
use crate::image::Color;
use crate::common::{IntersectResult, Ray};

use super::traits::Illuminate;

pub struct PointLight {
    position : Vector3<f64>,
    intensity : Color
}

impl PointLight {
    pub fn new(position : Vector3<f64>, intensity : Color) -> PointLight {
        PointLight {
            position, 
            intensity
        }
    }
}

impl Illuminate for PointLight {
    fn illuminate(ray : Ray, hit: IntersectResult) -> Color {
        if hit == IntersectResult::Miss  {
            return Color::new_rgb(0., 0., 0.)
        }
        Color::new_rgb(1., 1., 1.)
    }
}


use nalgebra::{Point3, UnitVector3};

use crate::common::{IntersectResult, Material, Ray};
use crate::image::Color;

pub trait Illuminate {
    fn illuminate(
        &self,
        ray: &Ray,
        point: &Point3<f64>,
        normal: &UnitVector3<f64>,
        material: &Material,
    ) -> Color;
}

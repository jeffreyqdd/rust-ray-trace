use std::ops::Mul;

use crate::common::{IntersectResult, Material, Ray};
use crate::image::Color;
use nalgebra::{clamp, distance_squared, Point3, UnitVector3};

use super::traits::Illuminate;

pub struct PointLight {
    position: Point3<f64>,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Point3<f64>, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

impl Illuminate for PointLight {
    fn illuminate(
        &self,
        ray: &Ray,
        point: &Point3<f64>,
        normal: &UnitVector3<f64>,
        material: &Material,
    ) -> Color {
        // // calculate distance from point light source to intersection point
        let light_to_hit_dist = distance_squared(&self.position, point);
        let hit_point_to_light = UnitVector3::new_normalize(self.position - point);

        // Diffuse shading using lambertian shading model: The reflection is calculated by
        // taking the dot product of the surface's normal N along with L, a normalized
        // vector pointing from the hit point to the light source. The number is then
        // multiplied by the color of the surface and the intensity of the light hitting
        // the surface.

        let diffuse = (material.k_d * self.intensity) / light_to_hit_dist
            * f64::max(hit_point_to_light.dot(normal), 0.);

        // Use the Blinn-Phong reflection model
        // Calculate the halfway point between the viewer ray and the light source ray, H,
        // and take the dot product with the surface normal. Raise that to the power alpha,
        // and that is the specular component

        // reverse the direction of the ray and normalize it
        // this is because the ray goes into the material, we want the ray to come out of the material,
        // similar to how the ray from material to light source comes out of the material
        let hit_point_to_viewer = UnitVector3::new_normalize(-ray.direction);
        let halfway_vector = UnitVector3::new_normalize(
            hit_point_to_light.into_inner() + hit_point_to_viewer.into_inner(),
        );
        // TODO: verify that NS is the correct attribute to use
        let alpha_p = f64::powf(normal.dot(&halfway_vector), material.ns);

        // sum specular with diffuse
        let specular = (material.k_d + material.k_s * alpha_p) * diffuse;
        let mut cfinal = specular + diffuse;
        cfinal.clamp();
        cfinal
    }
}

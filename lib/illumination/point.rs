use crate::common::{IllumModel, IntersectResult, Material, Ray, Scene};
use crate::image::Color;
use crate::RAY_STEP_EPSILON;
use nalgebra::{distance_squared, Point3, UnitVector3};

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
        scene: &Scene,
        point: &Point3<f64>,
        normal: &UnitVector3<f64>,
        material: &Material,
    ) -> Color {
        let hit_point_to_light = UnitVector3::new_normalize(self.position - point);

        // if hit_point_to_light is blocked by an object, do not illuminate with light
        // we want to movepoint by a bit to account for rounding errors
        let light_ray = Ray::new(
            point.to_owned() + hit_point_to_light.into_inner() * RAY_STEP_EPSILON,
            hit_point_to_light.into_inner().to_owned(),
        );
        if matches!(scene.intersect(&light_ray), IntersectResult::Hit { .. }) {
            return Color::black_rgb();
        }

        // Skip Diffuse and Specular Calculations IlumModel is CONSTANT
        if let IllumModel::Constant = material.illum {
            return material.k_d;
        }

        // Diffuse shading using lambertian shading model: The reflection is calculated by
        // taking the dot product of the surface's normal N along with L, a normalized
        // vector pointing from the hit point to the light source. The number is then
        // multiplied by the color of the surface and the intensity of the light hitting
        // the surface.
        let light_to_hit_dist = distance_squared(&self.position, point);
        let mut diffuse = (material.k_d * self.intensity) / light_to_hit_dist
            * f64::max(hit_point_to_light.dot(normal), 0.);

        // Skip Specular Calculation if IlumModel is Diffuse
        if let IllumModel::Diffuse = material.illum {
            diffuse.clamp();
            return diffuse;
        }

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

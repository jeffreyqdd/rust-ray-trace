use nalgebra::{UnitVector3, Point3, distance_squared, clamp};
use crate::image::Color;
use crate::common::{IntersectResult, Ray};

use super::traits::Illuminate;

pub struct PointLight {
    position : Point3<f64>,
    intensity : Color
}
 
impl PointLight {
    pub fn new(position : Point3<f64>, intensity : Color) -> PointLight {
        PointLight {
            position, 
            intensity
        }
    }
}

impl Illuminate for PointLight {
    fn illuminate(&self, _ray : &Ray, hit: &IntersectResult) -> Color {
        match hit {
            IntersectResult::Miss => Color::new_rgb(0., 0., 0.), // TODO: have black enum
            IntersectResult::Hit { t, point, normal, material } => {
                // calculate distance from point light source to intersection point
                let light_to_hit_dist = distance_squared(&self.position, point);
                let hit_point_to_light = UnitVector3::new_normalize(self.position - point);
                
                // Diffuse shading using lambertian shading model:
                // The luminous intensity of a surface is proportional to the cosine of the angle 
                // between the surface normal and the direction of the light
                let mut diffuse = self.intensity * f64::max(hit_point_to_light.dot(normal), 0.) / light_to_hit_dist;

                diffuse.clamp();
                diffuse
            }
        }
    }
}


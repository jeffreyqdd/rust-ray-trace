use crate::common::{IntersectResult, Ray, Material};
use crate::geometry::Surface;
use nalgebra::{Point3, UnitVector3};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material : Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material}
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> IntersectResult {
        let ray_center = ray.origin - self.center;

        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&ray_center);
        let c = ray_center.dot(&ray_center) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        if discriminant < 0. {
            return IntersectResult::Miss;
        }

        let d_sqrt = discriminant.sqrt();
        let solution_minus = (-b - d_sqrt) / a;
        let solution_plus = (-b + d_sqrt) / a;

        let (t, point) = if solution_minus >= 0. {
            (solution_minus, ray.at(solution_minus))
        } else if solution_plus >= 0. {
            (solution_plus, ray.at(solution_plus))
        } else {
            return IntersectResult::Miss;
        };

        let outwards_normal = UnitVector3::new_normalize(point - self.center);

        IntersectResult::new_hit(t, point, self.material.clone(), outwards_normal, &ray.direction)
    }
}

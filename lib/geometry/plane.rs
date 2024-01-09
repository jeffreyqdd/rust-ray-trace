use crate::common::{IntersectResult, Ray, Material};
use crate::geometry::traits::Surface;
use crate::TRACE_EPSILON;
use approx::{abs_diff_eq, relative_eq, AbsDiffEq, RelativeEq};
use nalgebra::{vector, Point3, Unit, UnitVector3, Vector3, Vector4};

#[derive(Debug, PartialEq)]
pub struct Plane {
    normal: Unit<Vector3<f64>>,
    equation: Vector4<f64>,
    material : Material
}

impl Plane {
    pub fn new(point: Point3<f64>, normal: Unit<Vector3<f64>>, material : Material) -> Plane {
        let equation: Vector4<f64> = Vector4::new(
            normal.x,
            normal.y,
            normal.z,
            -normal.dot(&vector![point.x, point.y, point.z]),
        );

        Plane { normal, equation, material }
    }
}

impl Surface for Plane {
    fn intersect(&self, ray: &Ray) -> IntersectResult {
        let denom = self.equation.dot(&ray.direction.to_homogeneous());

        if f64::abs(denom) >= 1e-14 {
            let t = -self.equation.dot(&ray.origin.to_homogeneous()) / denom;
            let norm = self
                .normal
                .component_mul(&Vector3::repeat(f64::signum(denom)));

            return IntersectResult::Hit {
                t,
                point: ray.at(t),
                normal: UnitVector3::new_unchecked(-norm),
                material: self.material.clone()
            };
        }
        IntersectResult::Miss
    }
}

impl AbsDiffEq for Plane {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        TRACE_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(&self.normal, &other.normal, epsilon = epsilon)
            && abs_diff_eq!(&self.equation, &other.equation, epsilon = epsilon)
    }
}

impl RelativeEq for Plane {
    fn default_max_relative() -> Self::Epsilon {
        Self::default_epsilon()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        relative_eq!(
            &self.normal,
            &other.normal,
            epsilon = epsilon,
            max_relative = max_relative
        ) && relative_eq!(
            &self.equation,
            &other.equation,
            epsilon = epsilon,
            max_relative = max_relative
        )
    }
}

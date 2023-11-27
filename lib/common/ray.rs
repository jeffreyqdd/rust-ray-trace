use crate::TRACE_EPSILON;
use approx::{abs_diff_eq, relative_eq, AbsDiffEq, RelativeEq};
use nalgebra::{Point3, Vector3};

#[derive(Debug, PartialEq)]
/// Represents a ray in the form o + dt where o is the origin, d is the
/// direction and t is the parametric variable
pub struct Ray {
    /// point on ray when t = 0
    pub origin: Point3<f64>,

    /// direction of ray
    pub direction: Vector3<f64>,
}

impl Ray {
    /// Construct a ray struct the provided origin and direction
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    /// evaluates the ray equation and returns the point on P(t)
    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}

impl AbsDiffEq for Ray {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        TRACE_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.origin, other.origin, epsilon = epsilon)
            && abs_diff_eq!(self.direction, other.direction, epsilon = epsilon)
    }
}
impl RelativeEq for Ray {
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
            self.origin,
            other.origin,
            epsilon = epsilon,
            max_relative = max_relative
        ) && relative_eq!(
            self.direction,
            other.direction,
            epsilon = epsilon,
            max_relative = max_relative
        )
    }
}

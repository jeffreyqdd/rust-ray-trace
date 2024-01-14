use nalgebra::{Point3, Unit, Vector3};

use super::Material;

#[derive(Debug, PartialEq)]
// Represents the result of a ray intersection with a geometric object.
pub enum IntersectResult {
    /// Represents a successful intersection, where the ray hits the object.
    Hit {
        /// t value, such that r(t) = o + dt is the intersection point.
        t: f64,

        /// The 3D point in space where the intersection occurs.
        point: Point3<f64>,

        /// The normalized surface normal that points outwards from the point of interesction.
        normal: Unit<Vector3<f64>>,

        /// The material properties of the geometric object
        material: Material,
    },

    /// Representing a miss, indicating that the ray does not intersect any object
    Miss,
}

impl IntersectResult {
    /// Creates a new `IntersectResult` representing a successful intersection.
    ///
    /// # Arguments
    ///
    /// * `t` - The parameter along the ray at which the intersection occurs.
    /// * `point` - The 3D point in space where the intersection occurs.
    /// * `material` - The material properties of the intersected object.
    /// * `surface_normal` - The surface normal, not necessarily the outward normal
    /// * `ray_direction` - The direction of the intersecting ray.
    ///
    /// # Returns
    ///
    /// A new `IntersectResult` representing a successful intersection.
    pub fn new_hit(
        t: f64,
        point: Point3<f64>,
        material: Material,
        surface_normal: Unit<Vector3<f64>>,
        ray_direction: &Vector3<f64>,
    ) -> IntersectResult {
        // there are two surface normals, for example, for any point on a sphere, there is a normal
        // pointing towards the center, and the normal pointing out take the dot product between the
        // surface normal and the ray direction. The dot product is positive for obtuse angles and
        // negative for acute angles. If the angle is obtuse, then it means the surface normal is
        // wrong and should be flipped.
        let normal = if surface_normal.dot(ray_direction) < 0. {
            surface_normal
        } else {
            -surface_normal
        };

        IntersectResult::Hit {
            t,
            point,
            normal,
            material,
        }
    }
}

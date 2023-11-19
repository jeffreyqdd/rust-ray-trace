use crate::common::Ray;

/// This trait defines the interface for creating a view within a virtual world.
pub trait Camera {
    /// Generates a `Ray` in world coordinates, coloring a specific point (`x`, `y`)
    /// on the resulting image. Both `x` and `y` should be floating-point values within
    /// the inclusive range of 0 to 1. The coordinate system considers the top-left corner
    /// as (0, 0) and the bottom-right corner as (1, 1). The starting point of the `Ray`
    /// is determined by its intersection with the image plane, ensuring that objects
    /// behind the image plane are not included in the rendering.
    fn generate_ray(&self, x: f64, y: f64) -> Ray;
}

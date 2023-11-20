use crate::color::Color;
use crate::common::{IntersectResult, Ray};
pub trait Illuminate {
    fn illuminate(ray: Ray, hit: IntersectResult) -> Color;
}

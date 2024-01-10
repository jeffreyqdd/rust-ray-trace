use crate::image::Color;
use crate::common::{IntersectResult, Ray};

pub trait Illuminate {
    fn illuminate(&self, ray: &Ray, hit: &IntersectResult) -> Color;
}

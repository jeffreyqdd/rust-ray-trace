use crate::common::{IntersectResult, Ray};

pub trait Surface {
    fn intersect(&self, ray: &Ray) -> IntersectResult;
}

use crate::common::{IntersectResult, Ray};
use crate::geometry::Surface;
use crate::illumination::Illuminate;
use crate::image::Color;
use std::vec::Vec;

pub struct Scene {
    objects: Vec<Box<dyn Surface>>,
}

impl Scene {
    pub fn empty() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn new(objects: Vec<Box<dyn Surface>>) -> Scene {
        Scene { objects }
    }

    pub fn add_surface(&mut self, object: Box<dyn Surface>) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: &Ray) -> IntersectResult {
        let mut best_t = f64::INFINITY;
        let mut res = IntersectResult::Miss;
        for obj in &self.objects {
            let result = obj.intersect(ray);
            match result {
                IntersectResult::Hit { t, .. } if t < best_t => {
                    best_t = t;
                    res = result;
                }
                _ => (),
            };
        }
        res
    }
}

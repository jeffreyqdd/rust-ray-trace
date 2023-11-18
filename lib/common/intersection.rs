use nalgebra::{distance, Point3, Unit, Vector3};

#[derive(Debug)]
pub enum IntersectResult {
    Hit {
        t: f64,
        point: Point3<f64>,
        normal: Unit<Vector3<f64>>,
    },
    Miss,
}

impl PartialEq for IntersectResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                IntersectResult::Hit {
                    t: t1,
                    point: p1,
                    normal: n1,
                },
                IntersectResult::Hit {
                    t: t2,
                    point: p2,
                    normal: n2,
                },
            ) => {
                let t_diff = f64::abs(t1 - t2);
                let p_diff = distance(p1, p2);
                t_diff < 1e-8 && p_diff < 1e-8 && n1 == n2
            }
            (IntersectResult::Miss, IntersectResult::Miss) => true,
            _ => false,
        }
    }
}

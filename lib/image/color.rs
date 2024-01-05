use nalgebra::Vector3;

pub struct Color {
    data: Vector3<f64>,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        assert!(0.0 <= r && r <= 1.0);
        assert!(0.0 <= g && g <= 1.0);
        assert!(0.0 <= b && b <= 1.0);
        Color {
            data: Vector3::new(r, g, b),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.data.x, self.data.y, self.data.x)
    }
}

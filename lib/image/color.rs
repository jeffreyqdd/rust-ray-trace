use nalgebra::Vector3;

pub enum ColorType {
    RGB,
}
pub struct Color {
    fmt : ColorType,
    data: Vector3<f64>,
}

impl Color {
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Color {
        assert!(0.0 <= r && r <= 1.0);
        assert!(0.0 <= g && g <= 1.0);
        assert!(0.0 <= b && b <= 1.0);
        Color {
            fmt: ColorType::RGB,
            data: Vector3::new(r, g, b),
        }
    }

    pub fn r(&self) -> f64 {
        match self.fmt {
            ColorType::RGB => self.data.x,
        }
    }

    pub fn g(&self) -> f64 {
        match self.fmt {
            ColorType::RGB => self.data.y,
        }
    }

    pub fn b(&self) -> f64 {
        match self.fmt {
            ColorType::RGB => self.data.z,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.data.x, self.data.y, self.data.x)
    }
}

use nalgebra::Vector3;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColorType {
    RGB,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    fmt: ColorType,
    data: Vector3<f64>,
}

// TODO: check if multiplication mutates original
impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        match (self.fmt, rhs.fmt) {
            (ColorType::RGB, ColorType::RGB) => Color {
                fmt: self.fmt,
                data: self.data.component_mul(&rhs.data),
            },
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        match self.fmt {
            ColorType::RGB => Color {
                fmt: self.fmt,
                data: self.data * rhs,
            },
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        match self.fmt {
            ColorType::RGB => Color {
                fmt: self.fmt,
                data: self.data / rhs,
            },
        }
    }
}
impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        match (self.fmt, rhs.fmt) {
            (ColorType::RGB, ColorType::RGB) => self.data += rhs.data,
            (_, _) => panic!(),
        };
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        match (self.fmt, rhs.fmt) {
            (ColorType::RGB, ColorType::RGB) => Color {
                fmt: self.fmt,
                data: self.data + rhs.data,
            },
            (_, _) => panic!(),
        }
    }
}

impl Color {
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Color {
        // assert!(0.0 <= r && r <= 1.0);
        // assert!(0.0 <= g && g <= 1.0);
        // assert!(0.0 <= b && b <= 1.0);
        Color {
            fmt: ColorType::RGB,
            data: Vector3::new(r, g, b),
        }
    }

    pub fn black_rgb() -> Color {
        Color {
            fmt: ColorType::RGB,
            data: Vector3::new(0., 0., 0.),
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

    /// sometimes when a color is scaled by an intensity, the values do not fall within [0,1] when
    /// exported to rgb, so we need to clamp aka normalize.
    pub fn clamp(&mut self) {
        match self.fmt {
            ColorType::RGB => {
                self.data.x = self.data.x.min(1.).max(0.);
                self.data.y = self.data.y.min(1.).max(0.);
                self.data.z = self.data.z.min(1.).max(0.);
            }
        };
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.data.x, self.data.y, self.data.x)
    }
}

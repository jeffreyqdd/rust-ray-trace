use std::{fs::File, io::BufWriter};

use png::{BitDepth, ColorType, Encoder, ScaledFloat, SourceChromaticities};

use super::color::Color;

/// This enum defines the possible export configurations of this image module
/// Note: I imagine most of the ray tracer will be done in RGB8 or RGB16
pub enum ImageType {
    Rgb8bit,
    Rgb16bit,
    Rgba8bit,
    Rgba16bit,
}

/// This struct defines an image buffer that can be written to
pub struct Frame {
    width: usize,
    height: usize,
    depth: usize,
    bytes: Vec<f64>,
}

impl Frame {
    /// creates instance of frame dims [width]x[height]x[depth]
    /// typically, depth=1 is for grayscale, depth=3 is for RGB and depth=4 is for RGBA.
    pub fn new(width: usize, height: usize, depth: usize) -> Frame {
        Frame {
            width,
            height,
            depth,
            bytes: vec![0_f64; width * height * depth],
        }
    }

    /// write a value to a specific [width], [height], and [depth]
    /// [data] must be in the range [0., 1.]
    pub fn write(&mut self, width: usize, height: usize, depth: usize, data: f64) {
        // assert that the pixel value is between 0 and 1
        assert!(data >= 0. && data <= 1.);
        self.bytes[(width * self.depth) + (self.width * self.depth * height) + depth] = data;
    }

    pub fn write_color(&mut self, width: usize, height: usize, color: Color) {
        assert!(self.depth == 3);
        self.write(width, height, 0, color.r());
        self.write(width, height, 1, color.g());
        self.write(width, height, 2, color.b());
    }

    pub fn as_slice(&self) -> &[f64] {
        self.bytes.as_slice()
    }

    pub fn write_image(
        &self,
        file: File,
        image_type: ImageType,
        source_gamma: Option<ScaledFloat>,
        source_chromaticities: Option<SourceChromaticities>,
    ) {
        let ref mut w = BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.width as u32, self.height as u32);
        let (color_type, bit_depth) = match image_type {
            ImageType::Rgb8bit => (ColorType::Rgb, BitDepth::Eight),
            ImageType::Rgb16bit => (ColorType::Rgb, BitDepth::Sixteen),
            ImageType::Rgba8bit => (ColorType::Rgba, BitDepth::Eight),
            ImageType::Rgba16bit => (ColorType::Rgba, BitDepth::Sixteen),
        };
        encoder.set_color(color_type);
        encoder.set_depth(bit_depth);

        // set values if provided
        if let Some(v) = source_gamma {
            encoder.set_source_gamma(v);
        }

        if let Some(v) = source_chromaticities {
            encoder.set_source_chromaticities(v);
        }

        let (num_bytes, float_to_bytes_closure): (usize, Box<dyn FnMut(Vec<u8>, &f64) -> Vec<u8>>) =
            match image_type {
                ImageType::Rgba16bit => {
                    if self.depth != 4 {
                        panic!("Cannot render as RGBA type with depth {}", self.depth);
                    }

                    (
                        self.bytes.len() * 2,
                        Box::new(|mut acc: Vec<u8>, item: &f64| {
                            let scaled_float = (item * u16::MAX as f64) as u16;
                            acc.push(((scaled_float & 0xFF00) >> 8) as u8);
                            acc.push((scaled_float & 0x00FF) as u8);
                            acc
                        }),
                    )
                }
                ImageType::Rgba8bit => {
                    if self.depth != 4 {
                        panic!("Cannot render as RGBA type with depth {}", self.depth);
                    }

                    (
                        self.bytes.len(),
                        Box::new(|mut acc: Vec<u8>, item: &f64| {
                            let scaled_float = (item * u8::MAX as f64) as u8;
                            acc.push(scaled_float);
                            acc
                        }),
                    )
                }
                ImageType::Rgb8bit => {
                    if self.depth != 3 {
                        panic!("Cannot render as RGB type with depth {}", self.depth);
                    }
                    (
                        self.bytes.len(),
                        Box::new(|mut acc: Vec<u8>, item: &f64| {
                            let scaled_float = (item * u8::MAX as f64) as u8;
                            acc.push(scaled_float);
                            acc
                        }),
                    )
                }
                ImageType::Rgb16bit => {
                    if self.depth != 3 {
                        panic!("Cannot render as RGB type with depth {}", self.depth);
                    }
                    (
                        self.bytes.len() * 2,
                        Box::new(|mut acc: Vec<u8>, item: &f64| {
                            let scaled_float = (item * u16::MAX as f64) as u16;
                            acc.push(((scaled_float & 0xFF00) >> 8) as u8);
                            acc.push((scaled_float & 0x00FF) as u8);
                            acc
                        }),
                    )
                }
            };
        let byte_arr = self.bytes.iter().fold(
            // two bytes required to represent a u16
            // scale the float to a range between 0 and u16::MAX and then use a bitmask
            Vec::<u8>::with_capacity(num_bytes),
            float_to_bytes_closure,
        );

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(byte_arr.as_slice()).unwrap();
    }
}

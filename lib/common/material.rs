use crate::image::Color;

// Specifies which illumination model to use when rendering the material.
#[derive(Debug, PartialEq, Clone)]
pub enum IllumModel {
    /// a constant color illumination model, using K_d for the material,
    Constant,

    /// A diffuse coloring model using lambertian shading which uses k_d, k_a, and position of each
    /// light scource and the angle at which it strikes the surface
    Diffuse,

    // A diffuse and specular illumination model using k_a, k_d, k_s, and the position of each
    // light source and the angle at which it strikes the surface
    DiffuseAndLambertian,
}

/// Material struct defined according to the wavefrom material template library format (.mtl)
#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    /// ambient color, to account for the light that is scattered about the entire scene using
    /// values between 0 and 1
    pub k_a: Color,

    /// diffuse color
    pub k_d: Color,

    /// specular color, the color scene when the surface is shiny and mirror-like
    pub k_s: Color,

    /// specular component, usually in the range [0, 1000]
    pub ns: f64,

    /// index of refraction for the material in the range [0.001, 10]. A value of 1.0 means the
    /// object does not bend as it passes through the object
    pub ni: f64,

    // dissolve factor in the range [0.0, 1.0] where 0 is completely transparent and 1 is
    // completely opaque
    pub d: f64,

    /// render method
    pub illum: IllumModel,
}

impl Material {
    /// instantiates a material that renders as a black void according to the
    /// MTL format
    pub fn none() -> Material {
        Material {
            k_a: Color::black_rgb(),
            k_d: Color::black_rgb(),
            k_s: Color::black_rgb(),
            ns: 0.,
            ni: 1.,
            d: 1.,
            illum: IllumModel::Constant,
        }
    }

    pub fn red_matte() -> Material {
        Material {
            k_a: Color::new_rgb(1., 0., 0.),
            k_d: Color::new_rgb(1., 0., 0.),
            k_s: Color::new_rgb(1., 0., 0.),
            ns: 0.,
            ni: 1.,
            d: 1.,
            illum: IllumModel::Constant,
        }
    }
}

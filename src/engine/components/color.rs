use crate::theory::math::traits::Zero;
use crate::theory::math::{Vec3, vec3};
use std::ops::{Add, Mul};

macro_rules! consts_lch {
    (
        $($name: ident = $l: expr, $c: expr, $h: expr);*
        $(;)?
    ) => {
        impl Color {
            $(
                #[allow(dead_code)]
                pub const $name: Self = Self::from_oklchf($l, $c, $h*std::f32::consts::PI/180.);
            )*
        }
    };
}

/// Sources:
/// https://gist.github.com/earthbound19/e7fe15fdf8ca3ef814750a61bc75b5ce
mod conversion {
    use crate::theory::math::const_fns::{cos, sin};
    use crate::theory::math::{Transform, Vec3, vec3};

    fn linear_to_gamma(channel: f32) -> f32 {
        if channel >= 0.0031308 {
            1.055 * channel.powf(1. / 2.4) - 0.055
        } else {
            12.92 * channel
        }
    }
    pub fn srgb_to_rgb(srgb: Vec3) -> Vec3 {
        srgb.map_comp(linear_to_gamma)
    }
    fn rgb_to_lms(rgb: Vec3) -> Vec3 {
        const RGB_TO_LMS: Transform = Transform::from_array([
            0.4122214708,
            0.2119034982,
            0.0883024619,
            0.5363325363,
            0.6806995451,
            0.2817188376,
            0.0514459929,
            0.1073969566,
            0.6299787005,
            0.,
            0.,
            0.,
        ]);
        RGB_TO_LMS.tr_vec(rgb)
    }
    fn lms_to_oklab(lms: Vec3) -> Vec3 {
        let lms_ = lms.map_comp(f32::cbrt);
        const LMS_TO_OKLAB: Transform = Transform::from_array([
            0.2104542553,
            1.9779984951,
            0.0259040371,
            0.7936177850,
            -2.4285922050,
            0.7827717662,
            -0.0040720468,
            0.4505937099,
            -0.8086757660,
            0.,
            0.,
            0.,
        ]);
        LMS_TO_OKLAB.tr_vec(lms_)
    }
    pub fn rgb_to_oklab(rgb: Vec3) -> Vec3 {
        lms_to_oklab(rgb_to_lms(rgb))
    }

    pub const fn oklch_to_oklab(oklch: Vec3) -> Vec3 {
        vec3(
            oklch.x(),
            cos(oklch.z()) * oklch.y(),
            sin(oklch.z()) * oklch.y(),
        )
    }
}

/// OKLAB color + alpha (0. or 1.)
#[derive(Clone, Copy, PartialEq, Debug, Default, Hash)]
pub struct Color(Vec3, bool);
impl Color {
    pub const TRANSPARENT: Self = Self(Vec3::ZERO, false);
    pub fn to_array(self) -> [f32; 3] {
        self.0.to_array()
    }
    pub fn is_transparent(self) -> bool {
        !self.1
    }
    pub fn from_rgbv(rgb: Vec3) -> Self {
        Self(conversion::rgb_to_oklab(rgb), true)
    }
    pub fn from_rgbf(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgbv(vec3(r, g, b))
    }
    pub const fn from_oklchv(oklch: Vec3) -> Self {
        Self(conversion::oklch_to_oklab(oklch), true)
    }
    pub const fn from_oklchf(l: f32, c: f32, h: f32) -> Self {
        Self::from_oklchv(vec3(l, c, h))
    }
    pub const fn from_oklabv(oklab: Vec3) -> Self {
        Self(oklab, true)
    }
    pub const fn from_oklabf(l: f32, a: f32, b: f32) -> Self {
        Self::from_oklabv(vec3(l, a, b))
    }
    pub const fn new(l: f32, a: f32, b: f32) -> Self {
        Self::from_oklabf(l, a, b)
    }
}

// codes found using https://oklch.com/
consts_lch!(
    WHITE = 1., 0., 0.;
    BLACK = 0., 0., 0.;
    DEBUG = 0.5, 0.2, -30.;
    RED = 0.5, 0.2, 30.;
    GREEN = 0.5, 0.2, 140.;
    BLUE = 0.5, 0.2, 267.;
    YELLOW = 0.95, 0.2, 104.;
);
impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * (self.1 as u8 as f32) + rhs.0 * (rhs.1 as u8 as f32),
            self.1 | rhs.1,
        )
    }
}
impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1)
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self(Vec3::new(value[0], value[1], value[2]), value[3] >= 0.5)
    }
}

impl From<Color> for [f32; 4] {
    fn from(value: Color) -> [f32; 4] {
        [value.0.x(), value.0.y(), value.0.z(), value.1 as u8 as f32]
    }
}

#[test]
fn test() {
    use crate::theory::math::transform::Transform;
    use crate::theory::math::vec3::vec3;
    dbg!(Color::WHITE, Color::RED, Color::BLUE);
    const OKLAB_TO_LMS: Transform = Transform::from_cols(
        vec3(1.0, 1.0, 1.0),
        vec3(0.3963377774, -0.1055613458, -0.0894841775),
        vec3(0.2158037573, -0.0638541728, -1.2914855480),
    );
    const LMS3_TO_SRGB: Transform = Transform::from_cols(
        vec3(4.0767245293, -1.2684380046, -0.0041960863),
        vec3(-3.3077115913, 2.6097574011, -0.7034186147),
        vec3(0.2309699292, -0.3413193965, 1.7076147010),
    );

    for (name, oklch) in [
        ("BLUE", Color::BLUE),
        ("RED", Color::RED),
        ("GREEN", Color::GREEN),
    ] {
        dbg!(name);
        let lms = OKLAB_TO_LMS.tr_vec(oklch.0);
        let lms3 = Vec3(lms.0 * lms.0 * lms.0);
        let srgb = LMS3_TO_SRGB.tr_vec(lms3);
        // let rgb = conversion::srgb_to_rgb(srgb) * 255.;
        dbg!(oklch.0, lms, lms3, srgb);
    }
}

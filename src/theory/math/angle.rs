use crate::theory::math::Vec2;
use crate::theory::math::traits::Zero;
use std::f32::consts::{PI, TAU};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A radian angle
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Angle(f32);
#[allow(dead_code)]
impl Angle {
    pub const HALF: Self = Self(PI);
    pub const QUARTER: Self = Self(PI/2.);
    pub const fn from_rad(rad: f32) -> Self {
        Self(rad)
    }
    pub fn from_deg(deg: f32) -> Self {
        Self(deg.to_radians())
    }
    pub fn from_turn(turn: f32) -> Self {
        Self(turn * TAU)
    }
    pub const fn rad(self) -> f32 {
        self.0
    }
    pub fn deg(self) -> f32 {
        self.0.to_degrees()
    }
    pub fn turn(self) -> f32 {
        self.0 / TAU
    }
    pub fn cos(self) -> f32 {
        self.0.cos()
    }
    pub fn sin(self) -> f32 {
        self.0.sin()
    }
    pub fn tan(self) -> f32 {
        self.0.tan()
    }
    pub fn cotan(self) -> f32 {
        1. / self.0.tan()
    }
    pub fn turns_normalised(self) -> f32 {
        self.turn().rem_euclid(1.)
    }
    pub fn rad_normalised(self) -> f32 {
        self.rad().rem_euclid(TAU)
    }
    pub fn from_vec2(v: Vec2) -> Self {
        Self(v.y().atan2(v.x()))
    }
    /// Angle (a b c). In [0; pi[ iff c is more counterclockwise than a
    pub fn from_points(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self::from_vec2(c - b) - Self::from_vec2(a - b)
    }
    /// Returns true iff self is in the (counterclockwise) oriented arc from a to b
    pub fn is_between(self, a: Self, b: Self) -> bool {
        (self - a).rad_normalised() <= (b-a).rad_normalised()
    }
    pub fn to_vec(self) -> Vec2 {
        Vec2::new(
            self.0.cos(),
            self.0.sin(),
        )
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<f32> for Angle {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Div<f32> for Angle {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl Neg for Angle {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl Zero for Angle {
    const ZERO: Self = Self(0.);
}

pub trait ToAngle {
    fn rad(self) -> Angle;
    fn deg(self) -> Angle;
    fn turn(self) -> Angle;
}
impl ToAngle for f32 {
    fn rad(self) -> Angle {
        Angle::from_rad(self)
    }
    fn deg(self) -> Angle {
        Angle::from_deg(self)
    }
    fn turn(self) -> Angle {
        Angle::from_turn(self)
    }
}

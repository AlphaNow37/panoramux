use crate::theory::math::traits::Length;
use crate::theory::math::{Dir, Plane, Vec3, vec3};
use rand::{Rng, RngExt};
use std::f32::consts::TAU;

#[derive(Clone, Copy, Debug)]
pub struct SpherePosition {
    z: f32,  // dans [-1; 1]
    theta: f32,  // angle (radians)
    radius: f32, // distance with the center of the sphere
}

impl SpherePosition {
    pub fn normal(self) -> Dir {
        let s = (1. - self.z * self.z).sqrt();
        Dir::from_normalized(vec3(
            self.theta.cos() * s,
            self.theta.sin() * s,
            self.z
        ))
    }
    pub fn to_vec3(self) -> Vec3 {
        *self.normal() * self.radius
    }
    pub fn local_plane(self) -> Plane {
        Plane::from_normal(self.normal())
    }
    pub fn from_vec3(v: Vec3) -> Self {
        let radius = v.length();
        let z = v.z() / radius;
        let theta = v.y().atan2(v.x());
        Self {
            z,
            theta,
            radius
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere(f32);

impl Sphere {
    pub fn sample_random(self, rng: &mut impl Rng) -> SpherePosition {
        SpherePosition {
            z: rng.random_range(-1.0..=1.0),
            theta: rng.random_range(0.0..TAU),
            radius: self.0,
        }
    }
}

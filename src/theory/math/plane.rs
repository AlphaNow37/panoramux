use crate::theory::math::{Dir, Transform, Vec3};
use std::ops::Neg;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Plane(Dir);
impl Plane {
    pub fn from_normal(dir: Dir) -> Self {
        Self(dir)
    }
    pub fn from_normal_vec(vec: Vec3) -> Option<Self> {
        vec.dir().map(Self)
    }
    pub fn from_ab(a: Vec3, b: Vec3) -> Option<Self> {
        Self::from_normal_vec(a.cross(b))
    }
    pub fn normal(self) -> Dir {
        self.0
    }
    pub fn project(self, vec: Vec3) -> Vec3 {
        vec - self.0.project(vec)
    }
    pub fn pointing_tr(self, y_dir: Vec3) -> Transform {
        let y = self
            .project(y_dir)
            .dir()
            .unwrap_or_else(|| self.0.any_ortho());
        Transform::from_cols(self.0.cross(*y), *y, *self.0)
    }
}
impl Neg for Plane {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

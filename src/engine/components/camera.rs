use crate::theory::math::{Angle, Mat4, Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub pos: Transform,
    pub fov: Angle,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Transform::ID,
            fov: Angle::from_deg(90.),
        }
    }
}
impl Camera {
    pub fn matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::new_perspective_infinite_lh(self.fov, aspect_ratio, 0.1)
            * self.pos.inverse().to_mat4()
    }
}

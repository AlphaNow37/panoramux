use super::super::app::keybinds::{KeyBinds, MoveKey, MoveModifierKey};
use crate::theory::math::traits::Zero;
use crate::engine::components::camera::Camera;
use crate::theory::math::{ToAngle, Vec2, Vec3, rotate_x, rotate_y};
use rand::RngExt;
use tracing::{info, info_span};
use winit::{dpi::PhysicalPosition, event::WindowEvent, window::Window};

fn move_key_to_dir(key: MoveKey) -> Vec3 {
    use MoveKey::*;
    match key {
        Left => -Vec3::X,
        Right => Vec3::X,
        Backward => -Vec3::Z,
        Forward => Vec3::Z,
        Down => -Vec3::Y,
        Up => Vec3::Y,
    }
}

fn move_modifier_key_to_factor(key: MoveModifierKey) -> f32 {
    use MoveModifierKey::*;
    match key {
        Fast => 5.,
        Slow => 0.2,
    }
}

pub struct ManualCamera {
    pub cam: Camera,
    pub current_cam_idx: isize,
    win_size: Vec2,
    cursor_locked: bool,
}
impl ManualCamera {
    pub fn new() -> Self {
        Self {
            cam: Camera::default(),
            current_cam_idx: 0,
            win_size: Vec2::ONE,
            cursor_locked: false,
        }
    }
    fn reset(&mut self) {
        info!("Reseting camera");
        self.cam = Camera::default();
        self.current_cam_idx = 0;
    }
    fn reset_current(&mut self) {
        self.current_cam_idx = 0;
        info!("Reseted current camera to 0");
    }
    fn rng_current(&mut self) {
        self.current_cam_idx = rand::rng().random::<i64>() as isize;
        info!("Randomed current camera to {}", self.current_cam_idx);
    }
    fn next_current(&mut self, off: isize) {
        self.current_cam_idx += off;
        info!(
            "Changing current camera to {} ({:+})",
            self.current_cam_idx, off
        );
    }
    fn toggle_lock(&mut self, win: &Window) {
        self.cursor_locked ^= true;
        if self.cursor_locked {
            info!("Locking cursor");
            win.set_cursor_position(winit::dpi::Position::Physical((self.win_size / 2.0).into()))
                .unwrap();
            win.set_cursor_visible(false);
        } else {
            info!("Unlocking cursor");
            win.set_cursor_visible(true);
        }
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.win_size.x() / self.win_size.y()
    }
    pub fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.win_size = new_size.into();
    }
    pub fn on_event(&mut self, event: &WindowEvent, win: &Window) {
        if let WindowEvent::CursorMoved { position, .. } = event {
            if self.cursor_locked {
                let pos: Vec2 = (*position).into();
                let angle = (pos - self.win_size / 2.) / self.win_size.y() * self.cam.fov.rad();
                self.cam.pos *= rotate_x(angle.y().rad()) * rotate_y(angle.x().rad());
                win.set_cursor_position(PhysicalPosition::new(
                    self.win_size.x() / 2.,
                    self.win_size.y() / 2.,
                ))
                .unwrap();
            }
        }
    }
    pub fn update(&mut self, dt: f32, win: &Window, binds: &KeyBinds) {
        let _span = info_span!("camera").entered();
        let mut off = Vec3::ZERO;
        for mk in MoveKey::ARRAY {
            if binds.camera_moves[mk as usize].is_active() {
                off += move_key_to_dir(mk) * dt
            }
        }
        for mmk in MoveModifierKey::ARRAY {
            if binds.camera_move_modifiers[mmk as usize].is_active() {
                off *= move_modifier_key_to_factor(mmk)
            }
        }
        self.cam.pos += off;
        if off != Vec3::ZERO {
            info!("Moved camera to {} ({:+})", self.cam.pos.trans(), off);
            // let mat = self.cam.matrix(self.aspect_ratio);
            // info!("(0., 0., 0.) => {}:{}", mat.transform_point3(Vec3::new(0., 0., 0.)), mat.project_point3(Vec3::new(0., 0., 0.)));
            // info!("(0., 0., 1.) => {}:{}", mat.transform_point3(Vec3::new(0., 0., 1.)), mat.project_point3(Vec3::new(0., 0., 1.)));
        }
        if binds.camera_change.prev_cam.is_active() {
            self.next_current(-1);
        }
        if binds.camera_change.next_cam.is_active() {
            self.next_current(1);
        }
        if binds.camera_change.reset_cam.is_active() {
            self.reset_current();
        }
        if binds.camera_change.rng_cam.is_active() {
            self.rng_current();
        }
        if binds.camera_change.toggle_lock.is_active() {
            self.toggle_lock(win);
        }
        if binds.camera_change.reset_pos.is_active() {
            self.reset();
        }
    }
}

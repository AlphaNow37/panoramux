use crate::theory::math::Vec2;
use winit::dpi::{PhysicalPosition, PhysicalSize, Pixel};

impl<T: Pixel> Into<Vec2> for PhysicalPosition<T> {
    fn into(self) -> Vec2 {
        let t: (f32, f32) = self.cast::<f32>().into();
        Vec2::new(t.0, t.1)
    }
}
impl<T: Pixel> Into<Vec2> for PhysicalSize<T> {
    fn into(self) -> Vec2 {
        let t: (f32, f32) = self.cast::<f32>().into();
        Vec2::new(t.0, t.1)
    }
}
impl<T: Pixel> Into<PhysicalPosition<T>> for Vec2 {
    fn into(self) -> PhysicalPosition<T> {
        (self.x(), self.y()).into()
    }
}
impl<T: Pixel> Into<PhysicalSize<T>> for Vec2 {
    fn into(self) -> PhysicalSize<T> {
        (self.x(), self.y()).into()
    }
}

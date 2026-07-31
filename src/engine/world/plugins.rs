use crate::engine::world::World;
use std::cell::OnceCell;

pub trait Plugin {
    fn update(&mut self, world: &mut World);
}

enum ClosurePlugin<P: FnMut(&mut World), F: FnOnce(&mut World) -> P> {
    Uninitialized(F),
    Initialized(P),
    None,
}
impl<P: FnMut(&mut World), F: FnOnce(&mut World) -> P> Plugin for ClosurePlugin<P, F> {
    fn update(&mut self, world: &mut World) {
        match std::mem::replace(self, Self::None) {
            Self::None => {
                println!("weird...");
            }
            Self::Uninitialized(builder) => {
                let mut f = builder(world);
                f(world);
                *self = Self::Initialized(f);
            },
            Self::Initialized(mut f) => {
                f(world);
                *self = Self::Initialized(f);
            }
        }
    }
}

pub fn plugin_closure<P: FnMut(&mut World)>(maker: impl FnOnce(&mut World) -> P) -> impl Plugin {
    ClosurePlugin::Uninitialized(maker)
}

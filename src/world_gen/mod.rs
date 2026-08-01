use crate::engine::world::plugins::{Plugin, plugin_closure};
use crate::world_gen::materials::load_materials;

pub mod biomes;
pub mod flocon;
pub mod ground;
pub mod materials;
pub mod tests;

pub fn plugin() -> impl Plugin {
    let mut plugin = flocon::put_many_flocon_plugin();
    plugin_closure(move |world| {
        load_materials(world);
        move |world| plugin.update(world)
    })
}

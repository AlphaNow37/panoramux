use crate::engine::pipelines::material::{Material, MaterialHandle};
use crate::engine::world::World;
use std::cell::Cell;

thread_local! {
    pub static UNIFORM_MATERIAL: Cell<MaterialHandle> = const {Cell::new(MaterialHandle::NONE)};
    pub static DEBUG_MATERIAL: Cell<MaterialHandle> = const {Cell::new(MaterialHandle::NONE)};
}

pub fn load_materials(world: &mut World) {
    UNIFORM_MATERIAL.set(world.add_material(Material {
        entry_point: "fs_uniform",
        name: "Couleur uniforme",
    }));
}

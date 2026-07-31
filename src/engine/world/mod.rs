pub mod plugins;

use crate::engine::components::camera::Camera;
use crate::engine::pipelines::instance::InstancesBuffer;
use crate::engine::pipelines::material::{Material, MaterialHandle};
use crate::engine::pipelines::mesh::MeshesBuffer;
use crate::engine::pipelines::storages::ColorItemStore;
use crate::theory::datastrutures::arena::{Arena, ArenaIndex};


pub struct WorldData {
    pub cameras: Arena<Camera>,
    pub manual_camera: ArenaIndex<Camera>,
    pub current_tick: usize,
}

pub struct World<'a> {
    pub cameras: &'a mut Arena<Camera>,
    pub manual_camera: ArenaIndex<Camera>,
    pub colors: &'a mut ColorItemStore,
    pub instances: &'a mut InstancesBuffer,
    pub meshes: &'a mut MeshesBuffer,
    pub current_tick: usize,
    pub(super) pending_materials_to_push: &'a mut Vec<Material>,
    pub(super) next_material_id: &'a mut usize,
}
impl<'a> World<'a> {
    pub fn add_material(&mut self, mat: Material) -> MaterialHandle {
        self.pending_materials_to_push.push(mat);
        *self.next_material_id += 1;
        MaterialHandle(*self.next_material_id - 1)
    }
}

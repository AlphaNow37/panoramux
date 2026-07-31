use crate::engine::components::color::Color;
use crate::engine::pipelines::buffer_allocator::{AllocSlot, BufferAllocator};
use crate::engine::pipelines::material::MaterialHandle;
use crate::engine::pipelines::mesh::MeshHandle;
use crate::engine::pipelines::storages::ItemHandle;
use crate::theory::math::Transform;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstanceElt {
    global: [f32; 12],  // as a list of 4 columns of 3 floats (column major)
    local: [f32; 12],
    material_idx: u32,
}
impl RawInstanceElt {
    pub const ATTRS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x3,
        6 => Float32x3,
        7 => Float32x3,
        8 => Uint32,
    ];
    pub const ELT_SIZE_U32: usize = 25;
    pub const ELT_SIZE_U8: usize = Self::ELT_SIZE_U32 * 4;
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::ELT_SIZE_U8 as wgpu::BufferAddress,
        attributes: Self::ATTRS,
        step_mode: wgpu::VertexStepMode::Instance,
    };
}

pub struct InstancesBuffer {
    buffer: wgpu::Buffer,
    pub allocator: BufferAllocator<RawInstanceElt>,
}
impl InstancesBuffer {
    const DEFAULT_BUFFER_SIZE: usize = 2048;
    fn get_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            size: (capacity * RawInstanceElt::ELT_SIZE_U8) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            label: Some("Instance mega buffer"),
            mapped_at_creation: false,
        })
    }
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            buffer: Self::get_buffer(device, Self::DEFAULT_BUFFER_SIZE),
            allocator: BufferAllocator::new(Self::DEFAULT_BUFFER_SIZE),
        }
    }
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let required_size = self.allocator.required_total_size() * RawInstanceElt::ELT_SIZE_U8;
        if required_size > self.buffer.size() as usize {
            self.buffer.destroy();
            self.buffer = Self::get_buffer(device, required_size)
        }
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&self.allocator.values),
        );
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
    }

    pub fn reserve_permanent(&mut self, material: MaterialHandle, mesh: MeshHandle) -> AllocSlot {
        self.allocator.reserve_permanent(material, mesh)
    }
    pub fn set(
        &mut self,
        slot: AllocSlot,
        global: Transform,
        local: Transform,
        material_idx: ItemHandle<Color>,
    ) {
        self.allocator.set(
            slot,
            RawInstanceElt {
                global: global.to_array(),
                local: local.to_array(),
                material_idx: material_idx.to_int() as u32,
            },
        )
    }
    pub fn add(
        &mut self,
        material: MaterialHandle,
        mesh: MeshHandle,
        global: Transform,
        local: Transform,
        material_idx: ItemHandle<Color>,
    ) {
        self.allocator.add(
            material,
            mesh,
            RawInstanceElt {
                global: global.to_array(),
                local: local.to_array(),
                material_idx: material_idx.to_int() as u32,
            },
        )
    }
    pub fn add_permanent(
        &mut self,
        material: MaterialHandle,
        mesh: MeshHandle,
        global: Transform,
        local: Transform,
        material_idx: ItemHandle<Color>,
    ) -> AllocSlot {
        let slot = self.reserve_permanent(material, mesh);
        self.set(slot, global, local, material_idx);
        slot
    }
}

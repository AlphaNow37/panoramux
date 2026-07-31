use crate::theory::math::traits::Length;
use crate::theory::math::Vec3;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
pub struct MeshVertex {
    position: [f32; 3],
    normal: [f32; 3],
    material_offset: u32,
}
impl MeshVertex {
    pub const ATTRS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        20 => Float32x3,
        21 => Float32x3,
        22 => Uint32,
    ];
    pub const ELT_SIZE_U32: usize = 7;
    pub const ELT_SIZE_U8: usize = Self::ELT_SIZE_U32 * 4;
    pub const BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: Self::ELT_SIZE_U8 as wgpu::BufferAddress,
        attributes: Self::ATTRS,
        step_mode: wgpu::VertexStepMode::Vertex,
    };
}

#[derive(Clone, Copy, Debug)]
pub struct MeshTriangle {
    pub positions: [Vec3; 3],
    pub normals: [Vec3; 3],
    pub material_offset: usize,
}
impl MeshTriangle {
    pub fn flat_triangle(positions: [Vec3; 3], material_offset: usize) -> MeshTriangle {
        let normal = (positions[0] - positions[1])
            .cross(positions[2] - positions[1])
            .normalize_or_zero();
        Self {
            positions,
            normals: [normal; 3],
            material_offset,
        }
    }
    pub fn push_to_buffer(&self, buffer: &mut Vec<MeshVertex>) {
        for i in 0..3 {
            let vertex = MeshVertex {
                position: self.positions[i].to_array(),
                normal: self.normals[i].to_array(),
                material_offset: self.material_offset as u32,
            };
            buffer.push(vertex);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub triangles: Vec<MeshTriangle>,
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }
    pub fn add_tri(&mut self, tri: MeshTriangle) {
        self.triangles.push(tri);
    }
    pub fn with_tri(mut self, tri: MeshTriangle) -> Self {
        self.add_tri(tri);
        self
    }
    pub fn push_to_buffer(&self, buffer: &mut Vec<MeshVertex>) {
        for tri in self.triangles.iter() {
            tri.push_to_buffer(buffer);
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct MeshHandle {
    pub start: u32,
    pub end: u32,
}

pub struct MeshesBuffer {
    buffer: wgpu::Buffer,
    current_pushed_size: usize, // nb of MeshVertex already pushed
    vertices: Vec<MeshVertex>,
}
impl MeshesBuffer {
    fn get_buffer(device: &wgpu::Device, size_vertex: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            size: (size_vertex * MeshVertex::ELT_SIZE_U8) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
            label: Some("Mesh buffer"),
        })
    }
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            current_pushed_size: 0,
            buffer: Self::get_buffer(device, 1<<15),
            vertices: Vec::new(),
        }
    }
    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        let bef_size = self.vertices.len();
        mesh.push_to_buffer(&mut self.vertices);
        MeshHandle {
            start: bef_size as u32,
            end: self.vertices.len() as u32,
        }
    }
    pub fn push_pending(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.current_pushed_size == self.vertices.len() {
            return;
        }
        let offset = (self.current_pushed_size * MeshVertex::ELT_SIZE_U8) as wgpu::BufferAddress;
        let final_size = self.vertices.len();
        let required_size = (final_size * MeshVertex::ELT_SIZE_U8) as wgpu::BufferAddress;
        if required_size >= self.buffer.size() {
            self.buffer.destroy();
            self.buffer = Self::get_buffer(device, (required_size * 2) as usize);
            queue.write_buffer(
                &self.buffer,
                0,
                bytemuck::cast_slice(&self.vertices)
            )
        } else {
            queue.write_buffer(
                &self.buffer,
                offset,
                bytemuck::cast_slice(&self.vertices[self.current_pushed_size..])
            );
        }
        self.current_pushed_size = self.vertices.len();
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));
    }
}

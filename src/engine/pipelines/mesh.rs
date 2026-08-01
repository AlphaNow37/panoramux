use crate::theory::math::polygon::Polygon;
use crate::theory::math::traits::{Length, Zero};
use crate::theory::math::{Transform, Vec2, Vec3, trans, vec3};
use std::sync::LazyLock;

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
    /// The normal is added ccw
    pub fn flat_triangle(positions: [Vec3; 3], material_offset: usize) -> MeshTriangle {
        let normal = (positions[1] - positions[0])
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
    /// The normal is added ccw
    pub fn add_convex_polygon(&mut self, polygon: &[Vec3], material_offset: usize) {
        if polygon.len() <= 2 {
            return;
        }
        let normal = (polygon[1] - polygon[0]).cross(polygon[2] - polygon[1]);
        for i in 1..(polygon.len() - 1) {
            self.add_tri(MeshTriangle {
                positions: [polygon[0], polygon[i], polygon[i + 1]],
                normals: [normal; 3],
                material_offset,
            });
        }
    }
    pub fn add_square(&mut self, tr: Transform, material_offset: usize) {
        let pts = [Vec3::ZERO, Vec3::X, Vec3::X + Vec3::Y, Vec3::Y].map(|p| tr.tr_point(p));
        self.add_convex_polygon(&pts, material_offset)
    }
    pub fn add_mesh(&mut self, material_offset: usize, other: &Mesh, tr: Transform) {
        for t in &other.triangles {
            self.triangles.push(MeshTriangle {
                normals: t.normals.map(|n| tr.tr_vec(n)),
                positions: t.positions.map(|p| tr.tr_point(p)),
                material_offset: t.material_offset + material_offset,
            })
        }
    }
    // pub fn add_cube(&mut self, tr: Transform, material_offset: usize) {
    //     for side in [
    //         [vec3(0., 0., 0.), vec3(0., 1., 0.), vec3(1., 1., 0.), vec3(1., 0., 0.)],
    //     ]
    // }

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
            size: size_vertex as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
            label: Some("Mesh buffer"),
        })
    }
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            current_pushed_size: 0,
            buffer: Self::get_buffer(device, (1 << 15) * MeshVertex::ELT_SIZE_U8),
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
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.vertices))
        } else {
            queue.write_buffer(
                &self.buffer,
                offset,
                bytemuck::cast_slice(&self.vertices[self.current_pushed_size..]),
            );
        }
        self.current_pushed_size = self.vertices.len();
    }
    pub fn put(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(1, self.buffer.slice(..));
    }
}

pub static SQUARE_MESH: LazyLock<Mesh> = LazyLock::new(|| {
    let mut mesh = Mesh::new();
    mesh.add_convex_polygon(&[Vec3::ZERO, Vec3::X, Vec3::X + Vec3::Y, Vec3::Y], 0);
    mesh
});

pub static CUBE_MESH: LazyLock<Mesh> = LazyLock::new(|| {
    let mut mesh = Mesh::new();
    mesh.add_convex_polygon(&[Vec3::ZERO, Vec3::Y, Vec3::X + Vec3::Y, Vec3::X], 0);
    mesh.add_convex_polygon(&[Vec3::ZERO, Vec3::X, Vec3::X + Vec3::Z, Vec3::Z], 0);
    mesh.add_convex_polygon(&[Vec3::ZERO, Vec3::Z, Vec3::Y + Vec3::Z, Vec3::Y], 0);
    mesh.add_convex_polygon(
        &[
            Vec3::X + Vec3::Y + Vec3::Z,
            Vec3::Y + Vec3::Z,
            Vec3::Z,
            Vec3::X + Vec3::Z,
        ],
        0,
    );
    mesh.add_convex_polygon(
        &[
            Vec3::X + Vec3::Y + Vec3::Z,
            Vec3::X + Vec3::Z,
            Vec3::X,
            Vec3::Y + Vec3::X,
        ],
        0,
    );
    mesh.add_convex_polygon(
        &[
            Vec3::X + Vec3::Y + Vec3::Z,
            Vec3::X + Vec3::Y,
            Vec3::Y,
            Vec3::Y + Vec3::Z,
        ],
        0,
    );
    mesh
});

pub static CENTERED_CUBE_MESH: LazyLock<Mesh> = LazyLock::new(|| {
    let mut mesh = Mesh::new();
    mesh.add_mesh(0, &CUBE_MESH, trans(-0.5, -0.5, -0.5));
    mesh
});


/// Holes: in the z direction
pub static SQUARE_PIPE_MESH: LazyLock<Mesh> = LazyLock::new(|| {
    let mut mesh = Mesh::new();
    mesh.add_convex_polygon(&[Vec3::ZERO, Vec3::X, Vec3::X + Vec3::Z, Vec3::Z], 0);
    mesh.add_convex_polygon(&[Vec3::ZERO, Vec3::Z, Vec3::Y + Vec3::Z, Vec3::Y], 0);
    mesh.add_convex_polygon(
        &[
            Vec3::X + Vec3::Y + Vec3::Z,
            Vec3::X + Vec3::Z,
            Vec3::X,
            Vec3::Y + Vec3::X,
        ],
        0,
    );
    mesh.add_convex_polygon(
        &[
            Vec3::X + Vec3::Y + Vec3::Z,
            Vec3::X + Vec3::Y,
            Vec3::Y,
            Vec3::Y + Vec3::Z,
        ],
        0,
    );
    mesh
});

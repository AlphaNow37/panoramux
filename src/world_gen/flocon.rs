use crate::engine::pipelines::mesh::{
    CUBE_MESH, Mesh, MeshTriangle, SQUARE_MESH, SQUARE_PIPE_MESH,
};
use crate::theory::math::{rotate_x, rotate_z, scale, trans, vec2, vec3, Angle, Transform, Transform2, Vec2, Vec3};
use crate::theory::utils::random::WeightedDistr;
use rand::{Rng, RngExt, rng};
use crate::engine::pipelines::storages::ItemHandle;
use crate::engine::world::plugins::{plugin_closure, Plugin};
use crate::world_gen::materials::{DEBUG_MATERIAL, UNIFORM_MATERIAL};

struct FloconParameters {
    adjust_ratio: f32,
    side_length_base: f32,
    extr_type: ExtrType,
    side_extr_type: ExtrType,
    nb_parts: usize,
    sides_angle: Angle,
    core_type: CoreType,
    core_width: f32,
    part_length: f32,
    line_width: f32,
    height: f32,
}
impl FloconParameters {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            adjust_ratio: rng.random_range(0.0..0.6),
            side_length_base: rng.random_range(0.5..1.5),
            extr_type: ExtrType::Brute,
            side_extr_type: ExtrType::Brute,
            nb_parts: rng.random_range(2..=5),
            sides_angle: Angle::from_deg(rng.random_range(30.0..60.0)),
            core_type: CoreType::Filled,
            core_width: rng.random_range(0.5..0.8),
            part_length: rng.random_range(1.0..1.5),
            line_width: rng.random_range(0.05..0.25),
            height: rng.random_range(0.05..0.25),
        }
    }
    pub fn side_length(&self, part_i: usize) -> f32 {
        self.side_length_base + (self.nb_parts - part_i) as f32 * self.adjust_ratio
    }
}

enum ExtrType {
    Brute,
}

impl ExtrType {
    pub fn add_on_mesh(&self, mesh: &mut Mesh, transform: Transform, params: &FloconParameters) {
        match self {
            Self::Brute => mesh.add_mesh(
                0,
                &SQUARE_MESH,
                transform
                    .scaled(vec3(params.line_width, 1., params.height))
                    .translate(vec3(-0.5, 0., 0.))
                    .swap_yz(),
            ),
        }
    }
}

enum CoreType {
    Empty,
    Filled,
}

enum BranchePart {
    Sides { idx: usize },
}

impl BranchePart {
    pub fn add_on_mesh(
        &self,
        mesh: &mut Mesh,
        transform: Transform,
        params: &FloconParameters,
    ) -> Transform {
        match self {
            Self::Sides { idx } => {
                mesh.add_mesh(
                    0,
                    &SQUARE_PIPE_MESH,
                    transform
                        .scaled(vec3(params.line_width, params.part_length, params.height))
                        .translate(vec3(-0.5, 0., 0.))
                        .swap_yz(),
                );
                let tr_middle = transform.translate(vec3(0., params.part_length / 3., 0.));
                let tr_ccw = tr_middle * rotate_z(params.sides_angle);
                let tr_cw = tr_middle * rotate_z(-params.sides_angle);

                let mut add_side = |tr: Transform| {
                    let length = params.side_length(*idx);
                    mesh.add_mesh(
                        0,
                        &SQUARE_PIPE_MESH,
                        tr.scaled(vec3(
                            params.line_width,
                            length,
                            params.height,
                        ))
                        .translate(vec3(-0.5, 0., 0.))
                        .swap_yz(),
                    );
                    params.side_extr_type.add_on_mesh(mesh, tr.translate(vec3(0., length, 0.)), params);
                };

                add_side(tr_cw);
                add_side(tr_ccw);

                transform.translate(vec3(0., params.part_length, 0.))
            }
        }
    }
}

pub struct Flocon {
    params: FloconParameters,
    parts: Vec<BranchePart>,
}
impl Flocon {
    pub fn new() -> Self {
        let mut rng = rng();
        let params = FloconParameters::new(&mut rng);
        let mut parts = Vec::new();
        let mut curr_part_i = 0;
        while curr_part_i < params.nb_parts {
            let part = BranchePart::Sides { idx: curr_part_i };
            parts.push(part);
            curr_part_i += 1;
        }
        Self { params, parts }
    }
    pub fn mesh(&self) -> Mesh {
        let mut mesh = Mesh::new();
        for i in 0..6 {
            let angle = Angle::from_turn(i as f32 / 6.0);
            let mut transform = Transform::from_rotate_z(angle).scaled(Vec3::ONE * 0.1);
            for p in &self.parts {
                transform = p.add_on_mesh(&mut mesh, transform, &self.params)
            }
            self.params.extr_type.add_on_mesh(&mut mesh, transform, &self.params)
        }
        mesh
    }
}

pub fn put_many_flocon_plugin() -> impl Plugin {
    plugin_closure(|world| {
        let spacing = 1.5;
        for x in 0..10 {
            for y in 0..10 {
                let flocon = Flocon::new();
                let mesh = flocon.mesh();
                let mesh_h = world.meshes.add_mesh(mesh);
                world.instances.add_permanent(
                    DEBUG_MATERIAL.get(),
                    mesh_h,
                    trans(x as f32 * spacing - spacing * 5., y as f32 * spacing - spacing * 5., 2.),
                    Transform::ID,
                    ItemHandle::N0NE,
                );
            }
        }

        |_| {}
    })
}

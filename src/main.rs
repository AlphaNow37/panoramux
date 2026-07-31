#![feature(portable_simd)]

use crate::engine::components::color::Color;
use crate::engine::pipelines::material::Material;
use crate::engine::pipelines::mesh::{Mesh, MeshTriangle};
use crate::engine::run;
use crate::engine::world::plugins::{Plugin, plugin_closure};
use theory::math::{Transform, Vec3, trans};

pub mod world_gen;

pub mod engine;
pub mod theory;

/// 1: generate biomes
///     1: generate vertices
///     2: take the voronoi
///     3: assign each region a random biome
///         - ocean
///         - city
///         - mountain
///         - small town
///         - fields
/// 2: generate transitions
///     1: for each edge, take a random transition type
///         - highway
///         - beach
///         - river
///         - no transition
///     2: remove the required space on each side
/// 3: generate oceans
///     1: generate waves (animated: wind)
///     2: generate details
///         - boats (animated: moving)
///         - monsters (animated: moving)
///         - islands
/// 4: generate cities
///     1: take an hex grid
///     2: take random positions
///     3: generate a graph, such that the distance between two points is at most 3x the distance by flying
///     4: put in roads (1 hex wides, straight)
///     5: put in intersections, some being wider
///     6: fill regions:
///         - towers
///         - parcs
///     7: add details
///         - cars, .. (animated: moving)
/// 5: generate mountains
///     1: generate the heightmap. Scale depending on the distance to the edge of the biome
///     2: add details
///         - trees (animated: wind)
///         - animals (animated: moving)
/// 6: generate small towns
/// ...
///


fn main() {

    let mut plugin = plugin_closure(move |world| {
        println!("Fist pass !");
        let mesh = Mesh::new().with_tri(MeshTriangle::flat_triangle([Vec3::X, Vec3::Y, Vec3::Z], 0));
        let mesh_handle = world.meshes.add_mesh(mesh);
        let mat = world.add_material(Material {
            name: "uniform",
            entry_point: "fs_none"
        });
        let col = world.colors.add(Color::RED);
        let slot = world.instances.add_permanent(
            mat,
            mesh_handle,
            Transform::ID,
            Transform::ID,
            col,
        );
        move |world| {
            println!("hello ! {}", world.current_tick);
            world.instances.set(
                slot,
                Transform::ID,
                trans(world.current_tick as f32 / 50., 0., 0.),
                col,
            )
        }
    });

    run(move |world| {
        plugin.update(world)
    });
}

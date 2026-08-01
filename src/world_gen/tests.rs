use crate::engine::components::color::Color;
use crate::engine::pipelines::material::Material;
use crate::engine::pipelines::mesh::{Mesh, MeshTriangle};
use crate::engine::world::plugins::{plugin_closure, Plugin};
use crate::theory::math::{trans, Transform, Vec3};

fn test_a() -> impl Plugin {
    plugin_closure(move |world| {
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
    })
}

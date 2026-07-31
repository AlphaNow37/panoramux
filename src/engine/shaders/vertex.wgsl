
struct InstanceInput {
    @location(0) global_x: vec3<f32>,
    @location(1) global_y: vec3<f32>,
    @location(2) global_z: vec3<f32>,
    @location(3) global_t: vec3<f32>,
    @location(4) local_x: vec3<f32>,
    @location(5) local_y: vec3<f32>,
    @location(6) local_z: vec3<f32>,
    @location(7) local_t: vec3<f32>,
    @location(8) mat_id: u32,
}

struct MeshVertex {
    @location(20) position: vec3<f32>,
    @location(21) normal: vec3<f32>,
    @location(22) mat_off: u32,
}

@vertex
fn vs_main(
    instance: InstanceInput,
    vertex: MeshVertex,
    @builtin(vertex_index) v_idx: u32,
) -> FragInput {
    var out: FragInput;

    let global = mat4x4(vec4(instance.global_x, 0.), vec4(instance.global_y, 0.), vec4(instance.global_z, 0.), vec4(instance.global_t, 1.));
    let local = mat4x4(vec4(instance.local_x, 0.), vec4(instance.local_y, 0.), vec4(instance.local_z, 0.), vec4(instance.local_t, 1.));

    let normal = global * (local * vec4(vertex.normal, 0.));
    let local_pos = local * vec4(vertex.position, 1.);
    let global_pos = global * local_pos;
    out.uv = local_pos.xyz;
    out.clip_position = camera * global_pos;
    out.delta_pos = global_pos.xyz - camera_transform[3].xyz;
    out.normal = normal.xyz / length(normal.xyz);
    out.mat_id = instance.mat_id + vertex.mat_off;
    return out;
}


struct FragInput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) delta_pos: vec3<f32>,
    @location(3) mat_id: u32,
};

@fragment
fn fs_none(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec4<f32> = vec4(0.35, 0.12, -0.12, 1.); // Purple
    let p = pass_all(in.normal, in.clip_position.z, in.delta_pos);
    col.x *= p;
    col.y *= pow(p, 0.5);
    col.z *= pow(p, 0.5);
    return frag_out(col);
}

@fragment
fn fs_uniform(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec4<f32>= colors[in.mat_id];
    // col.x *= pass_all(in.normal, in.clip_position.z, in.delta_pos);
    let p = pass_all(in.normal, in.clip_position.z, in.delta_pos);
    col.x *= p;
    col.y *= pow(p, 0.5);
    col.z *= pow(p, 0.5);
    return frag_out(col);
}

@fragment
fn fs_sponge(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec4<f32> = colors[in.mat_id];
    if(is_on_sponge(in.uv)) {
        col = colors[in.mat_id+1];
    }
    let p = pass_all(in.normal, in.clip_position.z, in.delta_pos);
    col.x *= p;
    col.y *= pow(p, 0.5);
    col.z *= pow(p, 0.5);
    return frag_out(col);
}


@fragment
fn fs_border(in: FragInput) -> @location(0) vec4<f32> {
    var col: vec4<f32>= colors[in.mat_id];

    if(!is_on_border(in.uv)) {
        discard;
    }
    let p = pass_all(in.normal, in.clip_position.z, in.delta_pos);
    col.x *= p;
    col.y *= pow(p, 0.5);
    col.z *= pow(p, 0.5);
    return frag_out(col);
}

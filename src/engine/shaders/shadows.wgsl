
fn pass_dist(clip_z: f32) -> f32 {
//    return 1.;
    return (1.-clip_z)/(1.-clip_z*0.999);
}
//fn pass_normal(normal: vec3<f32>) -> f32 {
//    let dot = abs(dot(camera_transform[2].xyz, normal));
//    return dot * 0.1 + 0.9;
//}
fn pass_normal(normal: vec3<f32>, delta_pos: vec3<f32>) -> f32 {
    let dot = abs(dot(normal, delta_pos)/length(delta_pos));
    return dot * 0.5 + 0.6;
}
fn pass_all(normal: vec3<f32>, clip_z: f32, delta_pos: vec3<f32>) -> f32 {
    return pass_dist(clip_z) * pass_normal(normal, delta_pos);
}

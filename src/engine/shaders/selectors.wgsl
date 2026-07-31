
const SPONGE_ITERATIONS: u32 = 5;
fn is_on_sponge(puv: vec3<f32>) -> bool {
    var uv = abs(puv);
    uv = uv % 1.;
    uv = min(uv, vec3(1., 1., 1.)-uv);
    for(var i: u32=0; i<SPONGE_ITERATIONS; i++) {
        if(u32(uv.x < 1./6.) + u32(uv.y < 1./6.) + u32(uv.z < 1./6.) > 1) {
            return true;
        }
        uv = min(uv, vec3(1./3., 1./3., 1./3.)-uv);
        uv = abs(uv) * 3.;
    }
    return false;
}

const BORDER_SIZE: f32 = 0.01;
fn is_on_border(puv: vec3<f32>) -> bool {
    return
        i32(puv.x > BORDER_SIZE - 1. && puv.x < 1. - BORDER_SIZE)
        + i32(puv.y > BORDER_SIZE - 1. && puv.y < 1. - BORDER_SIZE)
        + i32(puv.z > BORDER_SIZE - 1. && puv.z < 1. - BORDER_SIZE)
         <= 1;
}

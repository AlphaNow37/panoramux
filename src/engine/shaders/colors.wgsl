// consts from https://lygia.xyz/v1.2.0/color/space/oklab2rgb
const OKLAB_TO_LMS : mat3x3<f32>  = mat3x3<f32>(
    vec3f(1.0, 1.0, 1.),
    vec3f(0.3963377774, -0.1055613458, -0.0894841775),
    vec3f(0.2158037573, -0.0638541728, -1.2914855480)
);
const LMS3_TO_SRGB : mat3x3<f32>  = mat3x3<f32>(
    vec3f(4.0767245293, -1.2684380046, -0.0041960863),
    vec3f(-3.3077115913, 2.6097574011, -0.7034186147),
    vec3f(0.2309699292, -0.3413193965, 1.7076147010)
);

fn oklab_to_srgb(oklab: vec3<f32>) -> vec3<f32> {
    let lms = OKLAB_TO_LMS * oklab;
    return LMS3_TO_SRGB * (lms * lms * lms);
}
fn linear_to_gamma(channel: f32) -> f32 {
    if (channel >= 0.0031308) {
        return 1.055 * pow(channel, 1 / 2.4) - 0.055;
    } else {
        return 12.92 * channel;
    }

    // c >= 0.0031308 ? 1.055 * Math.pow(c, 1 / 2.4) - 0.055 : 12.92 * c
    // if (channel <= 0.04045) {
    //     return channel * 0.0773993808; // 1.0 / 12.92;
    //  } else {
    //     return pow((channel + 0.055) / 1.055, 2.4);
    // }
}
fn srgb_to_rgb(srgb: vec3<f32>) -> vec3<f32> {
    return srgb;
    // return vec3(
    //     linear_to_gamma(srgb.r),
    //     linear_to_gamma(srgb.g),
    //     linear_to_gamma(srgb.b)
    // );
}
fn oklab_to_rgb(oklab: vec3<f32>) -> vec3<f32> {
    var rgb = srgb_to_rgb(oklab_to_srgb(oklab));
    // if(rgb.x < 0.) {
    //     rgb = vec3(0.0, rgb.y + 0.5 * rgb.x, rgb.z + 0.5 * rgb.x);
    // }
    // if(rgb.y < 0.) {
    //     rgb = vec3(rgb.x + 0.5 * rgb.y, 0.0, rgb.z + 0.5 * rgb.y);
    // }
    // if(rgb.z < 0.) {
    //     rgb = vec3(rgb.x + 0.5 * rgb.z, rgb.y + 0.5 * rgb.z, 0.0);
    // }
    return rgb;
}
fn frag_out(oklab: vec4<f32>) -> vec4<f32> {
    return vec4(oklab_to_rgb(oklab.xyz), 1.);
}

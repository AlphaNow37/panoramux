
// Base bindings

@group(0) @binding(0)
var<uniform> time: f32;

@group(0) @binding(1)
var<uniform> camera: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> camera_transform: mat4x4<f32>;

/// Store bindings

@group(1) @binding(0)
var<storage> colors: array<vec4<f32>>;

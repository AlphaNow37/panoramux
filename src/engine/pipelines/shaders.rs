use std::borrow::Cow;

macro_rules! sources {
    (
        $($filename: literal)*
    ) => {
        concat!($(
            include_str!(concat!("../shaders/", $filename, ".wgsl")),
        )*)
    };
}

fn shader_sources() -> &'static str {
    sources!(
        "bindings"
        "colors"
        "shadows"
        "vertex"
        "selectors"
        "fragment"
    )
}

#[derive(Clone, Debug)]
pub struct Shaders {
    shaders: wgpu::ShaderModule,
}
impl Shaders {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            shaders: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shaders"),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_sources())),
            }),
        }
    }
    pub fn get(&self) -> &wgpu::ShaderModule {
        &self.shaders
    }
}

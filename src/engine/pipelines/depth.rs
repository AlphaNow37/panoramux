pub struct DepthBuffer {
    // pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}
impl DepthBuffer {
    pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth buffer"),
            size: wgpu::Extent3d {
                width: surf_config.width,
                height: surf_config.height,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            format: Self::FORMAT,
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        //     label: Some("depth sampler desc"),
        //     min_filter: wgpu::FilterMode::Linear,
        //     mag_filter: wgpu::FilterMode::Linear,
        //     compare: Some(wgpu::CompareFunction::LessEqual),
        //     lod_min_clamp: 0.,
        //     lod_max_clamp: 100.,
        //     ..Default::default()
        // });
        Self {
            // texture,
            view,
        }
    }
}

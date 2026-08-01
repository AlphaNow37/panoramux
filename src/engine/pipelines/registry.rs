use crate::engine::pipelines::bind_group_base::BaseBindings;
use crate::engine::pipelines::depth::DepthBuffer;
use crate::engine::pipelines::instance::{InstancesBuffer, RawInstanceElt};
use crate::engine::pipelines::material::{Material, MaterialHandle};
use crate::engine::pipelines::mesh::{MeshHandle, MeshVertex, MeshesBuffer};
use crate::engine::pipelines::shaders::Shaders;
use crate::engine::pipelines::storages::ItemStores;
use std::ops::Range;
use tracing::{info, info_span};

fn create_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    buffers_descriptor: &[wgpu::VertexBufferLayout],
    shaders: Shaders,
    texture_format: &wgpu::TextureFormat,
    polygon_mode: wgpu::PolygonMode,
    material: Material,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&format!("render pipeline for {}", material.name)),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            buffers: &buffers_descriptor,
            module: shaders.get(),
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shaders.get(),
            entry_point: Some(material.entry_point),
            targets: &[Some(wgpu::ColorTargetState {
                format: *texture_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            polygon_mode,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            bias: wgpu::DepthBiasState::default(),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            depth_write_enabled: Some(true),
            format: DepthBuffer::FORMAT,
            stencil: wgpu::StencilState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

pub struct Pipeline {
    pipeline_layout: wgpu::PipelineLayout,
    render_pipeline: wgpu::RenderPipeline,
    wireframe_render_pipeline: Option<wgpu::RenderPipeline>,
    material: Material,
    shaders: Shaders,
    buffers_descriptor: Vec<wgpu::VertexBufferLayout<'static>>,
    texture_format: wgpu::TextureFormat,
    device: wgpu::Device,
}
impl Pipeline {
    pub fn new(
        material: Material,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        base_bindings_layout: &wgpu::BindGroupLayout,
        store_bindings_layout: &wgpu::BindGroupLayout,
        shaders: Shaders,
    ) -> Self {
        let _span = info_span!("pipeline").entered();
        info!("Creating pipeline {}", material.name);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("Pipeline layout {}", material.name)),
            bind_group_layouts: &[Some(base_bindings_layout), Some(store_bindings_layout)],
            immediate_size: 0,
        });

        let buffers_descriptor = vec![RawInstanceElt::BUFFER_LAYOUT, MeshVertex::BUFFER_LAYOUT];

        let render_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &buffers_descriptor,
            shaders.clone(),
            &surface_config.format,
            wgpu::PolygonMode::Fill,
            material,
        );

        Self {
            pipeline_layout,
            render_pipeline,
            wireframe_render_pipeline: None,
            material,
            shaders,
            buffers_descriptor,
            texture_format: surface_config.format,
            device: device.clone(),
        }
    }
    fn generate_wire_pipeline(&mut self) {
        self.wireframe_render_pipeline = Some(create_pipeline(
            &self.device,
            &self.pipeline_layout,
            &self.buffers_descriptor,
            self.shaders.clone(),
            &self.texture_format,
            wgpu::PolygonMode::Line,
            self.material,
        ))
    }
    pub fn put(&mut self, render_pass: &mut wgpu::RenderPass, render_wires: bool) {
        if render_wires {
            if self.wireframe_render_pipeline.is_none()
                && self
                    .device
                    .features()
                    .contains(wgpu::Features::POLYGON_MODE_LINE)
            {
                self.generate_wire_pipeline();
            }
            if let Some(wire_render_pipeline) = &self.wireframe_render_pipeline {
                render_pass.set_pipeline(wire_render_pipeline);
                return;
            }
        }
        render_pass.set_pipeline(&self.render_pipeline);
    }
    pub fn render(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        render_wires: bool,
        instance_range: Range<u32>,
        mesh: MeshHandle,
    ) {
        let mut wires_pipe_loaded = false;
        if render_wires {
            if self.wireframe_render_pipeline.is_none()
                && self
                    .device
                    .features()
                    .contains(wgpu::Features::POLYGON_MODE_LINE)
            {
                self.generate_wire_pipeline();
            }
            if let Some(wire_render_pipeline) = &self.wireframe_render_pipeline {
                render_pass.set_pipeline(wire_render_pipeline);
                wires_pipe_loaded = true;
            }
        }
        if !wires_pipe_loaded {
            render_pass.set_pipeline(&self.render_pipeline);
        }
        render_pass.draw(mesh.start..mesh.end, instance_range);
    }
}

pub struct PipelinesRegistry {
    pub base_bindings: BaseBindings,
    pub depth_buffer: DepthBuffer,
    pub meshes: MeshesBuffer,
    pub instances: InstancesBuffer,
    pub shaders: Shaders,
    pub pipes: Vec<Pipeline>,
    pub pending_materials_to_push: Vec<Material>,
    pub next_material_id: usize,
    pub items: ItemStores,
}
impl PipelinesRegistry {
    pub fn new(device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration) -> Self {
        let mut s = Self {
            shaders: Shaders::new(device),
            base_bindings: BaseBindings::new(device),
            depth_buffer: DepthBuffer::new(device, surf_config),
            meshes: MeshesBuffer::new(device),
            instances: InstancesBuffer::new(device),
            pipes: Vec::new(),
            pending_materials_to_push: Vec::new(),
            next_material_id: 0,
            items: ItemStores::new(device),
        };
        let none_handle = s.add_material(Material {
            name: "Debug material",
            entry_point: "fs_none",
        });
        debug_assert_eq!(none_handle, MaterialHandle::NONE);
        s
    }
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surf_config: &wgpu::SurfaceConfiguration,
    ) {
        self.meshes.push_pending(device, queue);
        self.instances.update(device, queue);
        self.items.update(device, queue);
        for mat in self.pending_materials_to_push.drain(..) {
            self.pipes.push(Pipeline::new(
                mat,
                device,
                surf_config,
                &self.base_bindings.layout,
                &self.items.bind_group_layout,
                self.shaders.clone(),
            ))
        }
    }
    pub fn on_resize(&mut self, device: &wgpu::Device, surf_config: &wgpu::SurfaceConfiguration) {
        self.depth_buffer = DepthBuffer::new(device, surf_config);
    }
    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        render_wires: bool,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        self.base_bindings.put(&mut render_pass);
        self.items.put(&mut render_pass);
        self.instances.put(&mut render_pass);
        self.meshes.put(&mut render_pass);

        for (i, pipe) in self.pipes.iter_mut().enumerate() {
            pipe.put(&mut render_pass, render_wires);
            self.instances
                .allocator
                .foreach_block(MaterialHandle(i), |instance_range, mesh| {
                    debug_assert!(!instance_range.is_empty());
                    debug_assert_ne!(mesh.start, mesh.end);
                    render_pass.draw(mesh.start..mesh.end, instance_range);
                })
        }

        self.instances.allocator.reset_after_frame();
    }
    pub fn add_material(&mut self, mat: Material) -> MaterialHandle {
        self.pending_materials_to_push.push(mat);
        self.next_material_id += 1;
        MaterialHandle(self.next_material_id - 1)
    }
}

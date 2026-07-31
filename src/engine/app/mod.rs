mod camera;
mod exit;
mod keybinds;
mod render;
mod resize;
mod surface_holder;
mod update;
mod screenshots;

use super::app::exit::check_exit;
use super::app::render::check_render;
use super::app::resize::check_resize;
use super::app::surface_holder::SurfaceHolder;
use super::app::update::{Clock, check_update};
use camera::ManualCamera;
use tracing::{info, info_span};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use crate::engine::world::World;
use super::app::keybinds::KeyBinds;
use super::app::screenshots::check_screenshot;

fn get_adapter(surf: Option<&wgpu::Surface>, inst: &wgpu::Instance) -> wgpu::Adapter {
    let options = surf
        .map(|s| wgpu::RequestAdapterOptions {
            compatible_surface: Some(&s),
            ..Default::default()
        })
        .unwrap_or_default();
    pollster::block_on(inst.request_adapter(&options)).unwrap()
}

fn fetch_usable_features(adapter: &wgpu::Adapter) -> wgpu::Features {
    let mut features = wgpu::Features::default();
    let supported = adapter.features();
    features |= supported & wgpu::Features::POLYGON_MODE_LINE;
    features
}

fn get_device_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let desc = wgpu::DeviceDescriptor {
        label: Some("Device get desc"),
        required_limits: wgpu::Limits {
            max_vertex_attributes: 32,
            // max_vertex_attributes: 23,
            ..Default::default()
        },
        required_features: fetch_usable_features(adapter),
        ..Default::default()
    };
    pollster::block_on(adapter.request_device(&desc)).unwrap()
}

pub struct App {
    pub key_binds: KeyBinds,
    pub clock: Clock,
    pub window: Option<SurfaceHolder>,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub camera: ManualCamera,
    pub update_fn: Box<dyn FnMut(&mut World)>,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let _span = info_span!("restart").entered();
        let holder = SurfaceHolder::new(self, event_loop);
        if !self.adapter.is_surface_supported(&holder.surface) {
            self.adapter = get_adapter(Some(&holder.surface), &self.instance);
            // usefull ?
            // (self.device, self.queue) = get_device_queue(&self.adapter);
            // self.shaders = Shaders::load(&self.device);
        }
        self.window = Some(holder);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let _span = info_span!("app_update").entered();
        if check_exit(self, &event) {
            event_loop.exit();
            return;
        }
        check_resize(self, &event);
        check_update(self, &event);
        check_render(self, &event);

        if let Some(holder) = &self.window {
            self.camera.on_event(&event, &holder.window);
            check_screenshot(self);
        }
        self.key_binds.process(&event);
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(win) = &self.window {
            if self.clock.should_update() {
                win.window.request_redraw()
            }
        };
    }
}
impl App {
    pub fn new(update_fn: impl FnMut(&mut World) + 'static) -> Self {
        info!("Creating app");
        let instance = wgpu::Instance::default();
        let adapter = get_adapter(None, &instance);
        let (device, queue) = get_device_queue(&adapter);
        Self {
            key_binds: KeyBinds::base_binds(),
            clock: Clock::new(),
            window: None,
            adapter,
            instance,
            device,
            queue,
            camera: ManualCamera::new(),
            update_fn: Box::new(update_fn),
        }
    }
    pub fn run(&mut self) {
        info!("Running app");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).unwrap()
    }
}

use crate::engine::app::App;
use crate::engine::settings::perf_level;
use crate::engine::world::World;
use std::time::Instant;
use tracing::{info, info_span};
use winit::event::WindowEvent;

pub struct Clock {
    startup: Instant,
    last_render: Instant,
    min_delta: f32,
}
impl Clock {
    pub fn new() -> Self {
        let target_fps = perf_level!(
            30.0
            => VeryHighPerf
            60.0
        );

        Self {
            startup: Instant::now(),
            last_render: Instant::now(),
            min_delta: 1. / target_fps,
        }
    }
    pub fn should_update(&self) -> bool {
        self.last_render.elapsed().as_secs_f32() > self.min_delta
    }
}

pub fn check_update(app: &mut App, event: &WindowEvent) {
    if !matches!(event, WindowEvent::RedrawRequested) {
        return;
    }
    let _span = info_span!("update").entered();
    let now = Instant::now();
    let delta = now - app.clock.last_render;
    let time = (now - app.clock.startup).as_secs_f32();
    if app.key_binds.window_debug.show_fps.is_active() {
        info!(
            "delta={}ms, fps={}/{}, time={}",
            delta.as_millis(),
            1. / delta.as_secs_f32(),
            1. / app.clock.min_delta,
            time
        );
    }
    app.clock.last_render = now;

    if let Some(holder) = &mut app.window {
        app.camera
            .update(delta.as_secs_f32(), &holder.window, &app.key_binds);
        holder
            .world_data
            .cameras
            .set(holder.world_data.manual_camera, app.camera.cam);
        let mut world = World {
            instances: &mut holder.registry.instances,
            meshes: &mut holder.registry.meshes,
            colors: &mut holder.registry.items.colors,
            cameras: &mut holder.world_data.cameras,
            manual_camera: holder.world_data.manual_camera,
            current_tick: holder.world_data.current_tick,
            next_material_id: &mut holder.registry.next_material_id,
            pending_materials_to_push: &mut holder.registry.pending_materials_to_push,
        };
        (app.update_fn)(&mut world);
        holder.world_data.current_tick += 1;

        let camera = holder
            .world_data
            .cameras
            .get(holder.world_data.manual_camera);
        holder
            .registry
            .base_bindings
            .set_camera(&app.queue, camera.matrix(app.camera.aspect_ratio()));
        holder
            .registry
            .base_bindings
            .set_camera_transform(&app.queue, camera.pos.to_mat4());
        holder.registry.base_bindings.set_time(&app.queue, time); //, app.clock.loop_time);
        
        holder
            .registry
            .update(&app.device, &app.queue, &holder.surface_config);
    }
}

use super::super::app::App;
use tracing::{error, info_span};
use winit::event::WindowEvent;

pub fn check_render(app: &mut App, event: &WindowEvent) {
    if !matches!(event, WindowEvent::RedrawRequested) {
        return;
    }
    let _span = info_span!("render").entered();
    // info!("Rendering");

    let Some(holder) = &mut app.window else {
        error!("No window while rendering");
        return;
    };

    let output = match holder.surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(output) => output,
        wgpu::CurrentSurfaceTexture::Suboptimal(_) | wgpu::CurrentSurfaceTexture::Outdated => {
            let Some(win) = &app.window else {return;};
            win.surface.configure(&app.device, &win.surface_config);
            return;
        }
        _ => return,
    };
    
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = app
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render encoder"),
        });
    holder.registry.render(&mut encoder, &view, app.key_binds.window_debug.show_wires.is_active());
    app.queue.submit([encoder.finish()]);
    output.present();
}

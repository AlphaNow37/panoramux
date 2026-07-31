use super::super::app::App;
use tracing::{error, info, info_span};
use winit::event::WindowEvent;

pub fn check_resize(app: &mut App, event: &WindowEvent) {
    if let WindowEvent::Resized(new_size) = event {
        let _span = info_span!("resize").entered();
        if new_size.width == 0 || new_size.height == 0 {
            info!("Zero size: don't resize");
            return;
        }
        if let Some(win) = &mut app.window {
            info!("Resizing window to {}:{}", new_size.width, new_size.height);
            win.window.request_redraw();
            app.camera.on_resize(*new_size);
            win.surface_config.width = new_size.width;
            win.surface_config.height = new_size.height;
            win.surface.configure(&app.device, &win.surface_config);
            win.registry.on_resize(&app.device, &win.surface_config);
        } else {
            error!("No window while resizing");
        }
    }
}

use super::super::app::App;
use tracing::{info, info_span};
use winit::event::{KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug)]
enum ExitReason {
    Esc,
    Closed,
}

fn exit_reason(event: &WindowEvent) -> Option<ExitReason> {
    match event {
        WindowEvent::CloseRequested => Some(ExitReason::Closed),
        WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
            ..
        } => Some(ExitReason::Esc),
        _ => None,
    }
}

pub fn check_exit(_app: &mut App, event: &WindowEvent) -> bool {
    if let Some(reason) = exit_reason(event) {
        let _span = info_span!("exiting").entered();
        info!("Exiting with reason: {:?}", reason);
        true
    } else {
        false
    }
}

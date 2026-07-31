use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, info_span};
use super::super::app::App;

const OUTPUT_DIRECTORY: &'static str = "../out/screenshots";

fn get_output_path() -> PathBuf {
    let mut path = PathBuf::from(OUTPUT_DIRECTORY);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to fetch system time");
    let filename = format!("screenshot-{}.png", now.as_millis());
    path.push(filename);
    path
}

pub fn capture_screenshot() {
    let _span = info_span!("screenshot").entered();
    let path = get_output_path();

    info!("Taking a screenshot saved at {path:?}");


}

pub fn check_screenshot(app: &mut App) {
    if app.key_binds.window_utility.screenshot.is_active() {
        capture_screenshot();
    }
}

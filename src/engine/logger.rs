use std::sync::atomic::{AtomicBool, Ordering};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt};

static LOGGER_IS_INIT: AtomicBool = AtomicBool::new(false);

pub fn init_logger() {
    if !LOGGER_IS_INIT.load(Ordering::Relaxed) {
        LOGGER_IS_INIT.store(true, Ordering::Relaxed);
        tracing_subscriber::registry()
            .with(fmt::layer().without_time())
            .with(EnvFilter::new(
                r#"
            info,
            wgpu_hal=warn,
            wgpu_core=warn,
            naga=warn,
        "#
                    .replace([' ', '\n', '\t'], ""),
            ))
            .init()
    }
}

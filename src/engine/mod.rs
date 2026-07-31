use crate::engine::app::App;
use crate::engine::logger::init_logger;
use crate::engine::world::World;

pub mod pipelines;
pub mod utils;
pub mod world;
pub mod components;
pub mod settings;
pub mod logger;
pub mod app;


pub fn run(update_fn: impl FnMut(&mut World) + 'static) {
    init_logger();
    let mut app = App::new(update_fn);
    app.run();
}

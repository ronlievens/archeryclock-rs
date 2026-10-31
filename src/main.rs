mod config;
mod core;
mod lang;
mod scenario;

use core::app::App;

fn main() {
    env_logger::init();
    let mut app = App::new();
    app.run();
}

mod config;
mod arch;

fn main() {
    let _startup = arch::startup::Startup::init();
}

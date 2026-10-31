use crate::config::{paths::Paths, settings::Settings};

#[derive(Clone, Debug)]
pub struct Startup {
    pub paths: Paths,
    pub settings: Settings,
}

impl Startup {
    pub fn init() -> Self {
        let paths = Paths::discover();
        let settings = Settings::load(&paths).unwrap_or_default();
        Startup { paths, settings }
    }
}

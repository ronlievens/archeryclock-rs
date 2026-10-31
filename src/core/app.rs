use crate::config::{paths::Paths, settings::Settings};
use crate::lang::loader::LanguageLoader;
use crate::scenario::loader::ScenarioLoader;
use log::info;

#[derive(Debug)]
pub struct App {
    pub paths: Paths,
    pub settings: Settings,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        let paths = Paths::discover();
        let settings = Settings::load(&paths).unwrap_or_default();
        let _lang = LanguageLoader::load(&paths, &settings.language);
        let _scenarios = ScenarioLoader::init(&paths);
        App { paths, settings, running: true }
    }

    pub fn run(&mut self) {
        info!("ArcheryClock starting");
    }
}

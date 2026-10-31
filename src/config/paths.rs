use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Paths {
    pub exe_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub settings_file: PathBuf,
    pub scenarios_dir: PathBuf,
    pub sounds_dir: PathBuf,
    pub images_dir: PathBuf,
    pub language_dir: PathBuf,
}

impl Paths {
    pub fn discover() -> Self {
        let exe_dir = env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| env::current_dir().unwrap_or_default());
        let mut assets_dir = exe_dir.join("assets");
        if !assets_dir.exists() {
            assets_dir = exe_dir.join("..").join("assets");
        }
        if !assets_dir.exists() {
            assets_dir = env::current_dir().unwrap_or_default().join("assets");
        }
        Paths {
            settings_file: exe_dir.join("settings"),
            scenarios_dir: assets_dir.join("scenarios"),
            sounds_dir: assets_dir.join("sounds"),
            images_dir: assets_dir.join("images"),
            language_dir: assets_dir.join("language"),
            assets_dir,
            exe_dir,
        }
    }
}

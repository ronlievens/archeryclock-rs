use archeryclock_rs::config::paths::Paths;
use archeryclock_rs::config::settings::Settings;
use std::env;
use std::fs;
use std::path::PathBuf;

fn tmpdir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push(format!("archeryclock_test_{}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn uc001_first_run_creates_settings() {
    let dir = tmpdir();
    let paths = Paths {
        exe_dir: dir.clone(),
        assets_dir: dir.clone(),
        settings_file: dir.join("settings"),
        scenarios_dir: dir.join("scenarios"),
        sounds_dir: dir.join("sounds"),
        images_dir: dir.join("images"),
        language_dir: dir.join("language"),
    };
    assert!(!paths.settings_file.exists());
    let s = Settings::load(&paths).expect("load");
    assert!(paths.settings_file.exists());
    assert_eq!(s.pre_count_minutes, 4);
    assert!(s.hints);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn uc001_load_save_roundtrip() {
    let dir = tmpdir();
    let paths = Paths {
        exe_dir: dir.clone(),
        assets_dir: dir.clone(),
        settings_file: dir.join("settings"),
        scenarios_dir: dir.join("scenarios"),
        sounds_dir: dir.join("sounds"),
        images_dir: dir.join("images"),
        language_dir: dir.join("language"),
    };
    let mut s = Settings::default();
    s.language = "English".to_string();
    s.save(&paths).expect("save");
    let s2 = Settings::load(&paths).expect("load");
    assert_eq!(s2.language, "English");
    let _ = fs::remove_dir_all(&dir);
}

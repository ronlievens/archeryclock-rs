#[path = "../src/scenario/loader.rs"]
mod loader;
use loader::*;
use std::env;
use std::fs;
use std::path::PathBuf;

fn tmpdir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push(format!("archeryclock_scen_{}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn uc010_save_load() {
    let dir = tmpdir();
    let content = vec!["fita".to_string(), "90".to_string()];
    save_scenario(&dir, "F1", &content).unwrap();
    let s = load_scenario(&dir, "F1").unwrap();
    assert_eq!(s.content.len(), 2);
    assert_eq!(s.content[0], "fita");
    let _ = fs::remove_dir_all(&dir);
}

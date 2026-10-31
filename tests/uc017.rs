#[path = "../src/config/persistence.rs"]
mod persistence;
use persistence::*;
use std::env;
use std::fs;
use std::path::PathBuf;

fn tmpdir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push(format!("archeryclock_pers_{}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn uc017_roundtrip() {
    let dir = tmpdir();
    let p = dir.join("settings");
    let lines = vec!["a".to_string(), "b".to_string()];
    write_lines(&p, &lines).unwrap();
    let r = read_lines(&p).unwrap();
    assert_eq!(r.len(), 2);
    assert_eq!(r[0], "a");
    let _ = fs::remove_dir_all(&dir);
}

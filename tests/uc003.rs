#[path = "../src/format/fita.rs"]
mod fita;
use fita::*;

#[test]
fn uc003_fita_times_defaults() {
    let t = FitaTimes::default();
    assert_eq!(t.green, 120);
    assert_eq!(t.orange, 60);
    assert_eq!(t.red, 30);
}

#[test]
fn uc003_fita_config_default() {
    let c = FitaConfig::default();
    assert_eq!(c.pattern, "oneone");
    assert!(c.abcd_per_end);
    assert_eq!(c.practice_turns, 2);
}

#[test]
fn uc003_apply_fita_fixes() {
    let mut c = FitaConfig { times: FitaTimes { green: 0, orange: -1, red: 0, green_so: 60, orange_so: 30, red_so: 10 }, ..Default::default() };
    apply_fita(&mut c);
    assert_eq!(c.times.green, 120);
    assert_eq!(c.times.orange, 60);
    assert_eq!(c.times.red, 30);
}

#[path = "../src/format/finals.rs"]
mod finals;
use finals::*;

#[test]
fn uc005_defaults() {
    let c = FinalsConfig::default();
    assert!(c.mode_arrow);
    assert_eq!(c.archer, "left");
    assert!(c.time_per_end);
}

#[test]
fn uc005_target_mode() {
    let c = FinalsConfig { mode_arrow: false, ..Default::default() };
    assert!(!c.mode_arrow);
}

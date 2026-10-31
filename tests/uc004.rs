#[path = "../src/format/twentyfivem.rs"]
mod twentyfivem;
use twentyfivem::*;

#[test]
fn uc004_defaults() {
    let c = TwentyFiveMConfig::default();
    assert_eq!(c.shooters, 4);
    assert_eq!(c.green, 120);
    assert!(c.rotate);
}

#[test]
fn uc004_rotation_flags() {
    let c = TwentyFiveMConfig { rotate: false, rotate_p: false, ..Default::default() };
    assert!(!c.rotate && !c.rotate_p);
}

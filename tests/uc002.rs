#[path = "../src/arch/system.rs"]
mod system;
#[path = "../src/config/settings.rs"]
mod settings;

use system::*;
use settings::Archery;

#[test]
fn uc002_select_fita() {
    let a = init_system("fita");
    assert_eq!(a.archerysystem, "fita");
    assert!(is_fita(&a));
}

#[test]
fn uc002_select_fitafin() {
    let a = init_system("fitafin");
    assert_eq!(a.archerysystem, "fitafin");
    assert!(is_fitafin(&a));
}

#[test]
fn uc002_select_25m1p() {
    let a = init_system("25m1p");
    assert_eq!(a.archerysystem, "25m1p");
    assert!(is_25m1p(&a));
}

#[test]
fn uc002_select_manual() {
    let a = init_system("manual");
    assert_eq!(a.archerysystem, "manual");
    assert!(is_manual(&a));
}

#[test]
fn uc002_select_follow() {
    let a = init_system("follow");
    assert_eq!(a.archerysystem, "follow");
    assert!(is_follow(&a));
}

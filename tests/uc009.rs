#[path = "../src/core/emergency.rs"]
mod emergency;
use emergency::*;

#[test]
fn uc009_emergency() {
    let mut e = Emergency::default();
    assert!(!e.active);
    e.trigger();
    assert!(e.active);
    e.resume();
    assert!(!e.active);
}

#[test]
fn uc009_actions() {
    let e = Emergency { action: EmergencyAction::StopCurrent, ..Default::default() };
    assert_eq!(e.action, EmergencyAction::StopCurrent);
}

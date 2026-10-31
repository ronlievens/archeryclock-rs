#[path = "../src/mode/manual.rs"]
mod manual;
use manual::*;

#[test]
fn uc006_manual_flow() {
    let mut m = Manual::default();
    assert_eq!(m.state, ManualState::Stopped);
    m.start();
    assert_eq!(m.state, ManualState::Running);
    m.pause();
    assert_eq!(m.state, ManualState::Paused);
    m.stop();
    assert_eq!(m.state, ManualState::Stopped);
}

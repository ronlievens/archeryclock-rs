#[path = "../src/audio/signals.rs"]
mod signals;
use signals::*;

#[test]
fn uc011_signals() {
    assert_eq!(signal_for_phase("green"), "start");
    assert_eq!(signal_for_phase("red"), "end");
    assert_eq!(signal_for_phase("emergency"), "alarm");
}

#[test]
fn uc011_config() {
    let a = AudioConfig::default();
    assert!(a.enabled);
    assert_eq!(a.sound_mode, "Buzzer");
}

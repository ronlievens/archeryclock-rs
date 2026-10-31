#[path = "../src/core/timer.rs"]
mod timer;
use timer::*;

#[test]
fn uc008_transitions() {
    let mut c = Countdown::default();
    c.start(90);
    assert_eq!(c.phase, Phase::Green);
    c.seconds = 35;
    c.tick();
    assert_eq!(c.phase, Phase::Orange);
    c.seconds = 15;
    c.tick();
    assert_eq!(c.phase, Phase::Red);
    c.next();
    assert_eq!(c.phase, Phase::Wait);
    assert_eq!(c.turn, 2);
}

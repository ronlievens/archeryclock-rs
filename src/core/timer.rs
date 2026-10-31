#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Wait,
    Green,
    Orange,
    Red,
    PreCount,
    Emergency,
}

#[derive(Clone, Debug)]
pub struct Countdown {
    pub phase: Phase,
    pub seconds: i32,
    pub end: i32,
    pub turn: i32,
}

impl Default for Countdown {
    fn default() -> Self {
        Countdown { phase: Phase::Wait, seconds: 0, end: 1, turn: 1 }
    }
}

impl Countdown {
    pub fn start(&mut self, secs: i32) {
        self.phase = Phase::Green;
        self.seconds = secs;
    }

    pub fn tick(&mut self) {
        if self.seconds > 0 && self.phase != Phase::Wait && self.phase != Phase::PreCount && self.phase != Phase::Emergency {
            self.seconds -= 1;
        }
        if self.phase == Phase::Green && self.seconds <= 30 {
            self.phase = Phase::Orange;
        } else if self.phase == Phase::Orange && self.seconds <= 10 {
            self.phase = Phase::Red;
        }
    }

    pub fn next(&mut self) {
        self.turn += 1;
        self.phase = Phase::Wait;
        self.seconds = 0;
    }
}

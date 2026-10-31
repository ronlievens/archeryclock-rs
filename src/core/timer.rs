#[derive(Debug)]
pub struct CountdownTimer {
    pub remaining: i32,
}

impl CountdownTimer {
    pub fn new(s: i32) -> Self {
        CountdownTimer { remaining: s }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClockState {
    Wait,
    Green,
    Orange,
    Red,
    Countdown,
    Emergency,
}

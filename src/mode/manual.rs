#[derive(Clone, Debug, PartialEq)]
pub enum ManualState {
    Stopped,
    Running,
    Paused,
}

#[derive(Clone, Debug)]
pub struct Manual {
    pub state: ManualState,
}

impl Default for Manual {
    fn default() -> Self {
        Manual { state: ManualState::Stopped }
    }
}

impl Manual {
    pub fn start(&mut self) { self.state = ManualState::Running; }
    pub fn pause(&mut self) { if self.state == ManualState::Running { self.state = ManualState::Paused; } }
    pub fn stop(&mut self) { self.state = ManualState::Stopped; }
    pub fn next(&mut self) { self.state = ManualState::Running; }
}

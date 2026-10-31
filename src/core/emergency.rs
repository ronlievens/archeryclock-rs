#[derive(Clone, Debug, PartialEq)]
pub enum EmergencyAction {
    StopCurrent,
    StopNext,
    Pause,
}

#[derive(Clone, Debug)]
pub struct Emergency {
    pub active: bool,
    pub action: EmergencyAction,
    pub beeps: i32,
    pub keep_buzzing: bool,
    pub text: String,
}

impl Default for Emergency {
    fn default() -> Self {
        Emergency { active: false, action: EmergencyAction::Pause, beeps: 5, keep_buzzing: false, text: "Stop".to_string() }
    }
}

impl Emergency {
    pub fn trigger(&mut self) { self.active = true; }
    pub fn resume(&mut self) { self.active = false; }
}

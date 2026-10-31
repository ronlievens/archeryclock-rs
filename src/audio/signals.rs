#[derive(Clone, Debug)]
pub struct AudioConfig {
    pub sound_mode: String,
    pub volume: i32,
    pub enabled: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig { sound_mode: "Buzzer".to_string(), volume: 80, enabled: true }
    }
}

pub fn signal_for_phase(phase: &str) -> &'static str {
    match phase {
        "green" => "start",
        "orange" => "warning",
        "red" => "end",
        "emergency" => "alarm",
        _ => "none",
    }
}

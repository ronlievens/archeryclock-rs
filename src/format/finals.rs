#[derive(Clone, Debug)]
pub struct FinalsConfig {
    pub mode_arrow: bool,
    pub archer: String,
    pub time_per_end: bool,
}

impl Default for FinalsConfig {
    fn default() -> Self {
        FinalsConfig { mode_arrow: true, archer: "left".to_string(), time_per_end: true }
    }
}

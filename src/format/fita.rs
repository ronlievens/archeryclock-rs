#[derive(Clone, Debug)]
pub struct FitaTimes {
    pub green: i32,
    pub orange: i32,
    pub red: i32,
    pub green_so: i32,
    pub orange_so: i32,
    pub red_so: i32,
}

impl Default for FitaTimes {
    fn default() -> Self {
        FitaTimes { green: 120, orange: 60, red: 30, green_so: 60, orange_so: 30, red_so: 10 }
    }
}

#[derive(Clone, Debug)]
pub struct FitaConfig {
    pub pattern: String,
    pub abcd_per_end: bool,
    pub practice_turns: i32,
    pub times: FitaTimes,
}

impl Default for FitaConfig {
    fn default() -> Self {
        FitaConfig { pattern: "oneone".to_string(), abcd_per_end: true, practice_turns: 2, times: FitaTimes::default() }
    }
}

pub fn apply_fita(cfg: &mut FitaConfig) {
    if cfg.times.green <= 0 { cfg.times.green = 120; }
    if cfg.times.orange <= 0 { cfg.times.orange = 60; }
    if cfg.times.red <= 0 { cfg.times.red = 30; }
}

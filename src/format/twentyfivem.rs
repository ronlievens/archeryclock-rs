#[derive(Clone, Debug)]
pub struct TwentyFiveMConfig {
    pub shooters: i32,
    pub green: i32,
    pub orange: i32,
    pub red: i32,
    pub rotate: bool,
    pub rotate_p: bool,
}

impl Default for TwentyFiveMConfig {
    fn default() -> Self {
        TwentyFiveMConfig { shooters: 4, green: 120, orange: 60, red: 30, rotate: true, rotate_p: true }
    }
}

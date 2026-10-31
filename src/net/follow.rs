#[derive(Clone, Debug)]
pub struct FollowConfig {
    pub enabled: bool,
    pub host_ip: String,
    pub port: u16,
}

impl Default for FollowConfig {
    fn default() -> Self {
        FollowConfig { enabled: false, host_ip: "224.25.26.27".to_string(), port: 4665 }
    }
}

pub fn map_follow_system(code: &str) -> String {
    match code {
        "0" | "1" => "fita".to_string(),
        "2" => "fitafin".to_string(),
        "3" | "4" => "25m1p".to_string(),
        "5" | "6" => "manual".to_string(),
        _ => "fita".to_string(),
    }
}

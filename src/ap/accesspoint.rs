#[derive(Clone, Debug)]
pub struct ApConfig {
    pub enabled: bool,
    pub ssid: String,
    pub password: String,
    pub channel: u8,
    pub interface: String,
}

impl Default for ApConfig {
    fn default() -> Self {
        ApConfig { enabled: false, ssid: "ArcheryClock".to_string(), password: "".to_string(), channel: 6, interface: "wlan0".to_string() }
    }
}

pub fn validate(c: &ApConfig) -> bool {
    !c.ssid.is_empty()
}

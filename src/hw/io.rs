#[derive(Clone, Debug)]
pub struct HwConfig {
    pub interface: String,
    pub serial1: String,
    pub arduino1: String,
    pub serial2: String,
    pub arduino2: String,
}

impl Default for HwConfig {
    fn default() -> Self {
        HwConfig {
            interface: if cfg!(windows) { "K8055".to_string() } else { "na".to_string() },
            serial1: if cfg!(windows) { "COM1".to_string() } else { "/dev/ttyACM0".to_string() },
            arduino1: "nonarduino1".to_string(),
            serial2: if cfg!(windows) { "COM2".to_string() } else { "/dev/ttyUSB0".to_string() },
            arduino2: "nonarduino2".to_string(),
        }
    }
}

pub fn is_hw_enabled(c: &HwConfig) -> bool {
    c.interface != "na"
}

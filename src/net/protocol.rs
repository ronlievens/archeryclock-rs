#[derive(Clone, Debug)]
pub struct NetConfig {
    pub enabled: bool,
    pub port: u16,
    pub multicast: (u8,u8,u8,u8),
}

impl Default for NetConfig {
    fn default() -> Self {
        NetConfig { enabled: false, port: 4665, multicast: (224,25,26,27) }
    }
}

pub fn format_multicast(cfg: &NetConfig) -> String {
    format!("{}.{}.{}.{}:{}", cfg.multicast.0, cfg.multicast.1, cfg.multicast.2, cfg.multicast.3, cfg.port)
}

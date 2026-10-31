#[derive(Clone, Debug)]
pub struct StreamConfig {
    pub enabled: bool,
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate: String,
}

impl Default for StreamConfig {
    fn default() -> Self {
        StreamConfig { enabled: false, url: "rtmp://".to_string(), width: 1280, height: 720, fps: 30, bitrate: "2000k".to_string() }
    }
}

pub fn build_args(cfg: &StreamConfig) -> Vec<String> {
    vec![
        "-f".to_string(), "rawvideo".to_string(),
        "-pix_fmt".to_string(), "rgb24".to_string(),
        "-s".to_string(), format!("{}x{}", cfg.width, cfg.height),
        "-r".to_string(), cfg.fps.to_string(),
        "-i".to_string(), "-".to_string(),
        "-c:v".to_string(), "libx264".to_string(),
        "-b:v".to_string(), cfg.bitrate.clone(),
        cfg.url.clone(),
    ]
}

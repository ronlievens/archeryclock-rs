#[path = "../src/stream/ffmpeg.rs"]
mod ffmpeg;
use ffmpeg::*;

#[test]
fn uc016_args() {
    let c = StreamConfig::default();
    let args = build_args(&c);
    assert!(args.contains(&"-f".to_string()));
    assert!(args.contains(&"rtmp://".to_string()));
}

#[test]
fn uc016_defaults() {
    let c = StreamConfig::default();
    assert_eq!(c.fps, 30);
    assert_eq!(c.width, 1280);
}

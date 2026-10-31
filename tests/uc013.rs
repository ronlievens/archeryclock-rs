#[path = "../src/net/protocol.rs"]
mod protocol;
use protocol::*;

#[test]
fn uc013_multicast() {
    let c = NetConfig::default();
    assert_eq!(format_multicast(&c), "224.25.26.27:4665");
}

#[test]
fn uc013_config() {
    let c = NetConfig { enabled: true, ..Default::default() };
    assert!(c.enabled);
}

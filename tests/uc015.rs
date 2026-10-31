#[path = "../src/ap/accesspoint.rs"]
mod accesspoint;
use accesspoint::*;

#[test]
fn uc015_defaults() {
    let c = ApConfig::default();
    assert_eq!(c.ssid, "ArcheryClock");
    assert!(validate(&c));
}

#[test]
fn uc015_invalid() {
    let c = ApConfig { ssid: "".to_string(), ..Default::default() };
    assert!(!validate(&c));
}

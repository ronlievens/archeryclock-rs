#[path = "../src/hw/io.rs"]
mod io;
use io::*;

#[test]
fn uc014_hw_defaults() {
    let c = HwConfig::default();
    assert!(c.interface.len() > 0);
    assert!(c.serial1.len() > 0);
}

#[test]
fn uc014_hw_enabled() {
    let c = HwConfig { interface: "K8055".to_string(), ..Default::default() };
    assert!(is_hw_enabled(&c));
}

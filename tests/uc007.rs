#[path = "../src/net/follow.rs"]
mod follow;
use follow::*;

#[test]
fn uc007_map() {
    assert_eq!(map_follow_system("0"), "fita");
    assert_eq!(map_follow_system("2"), "fitafin");
    assert_eq!(map_follow_system("3"), "25m1p");
    assert_eq!(map_follow_system("5"), "manual");
    assert_eq!(map_follow_system("x"), "fita");
}

#[test]
fn uc007_defaults() {
    let c = FollowConfig::default();
    assert!(!c.enabled);
    assert_eq!(c.port, 4665);
}

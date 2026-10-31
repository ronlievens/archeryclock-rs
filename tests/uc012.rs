#[path = "../src/display/theme.rs"]
mod theme;
use theme::*;

#[test]
fn uc012_scheme() {
    let mut t = Theme::default();
    set_scheme(&mut t, 2);
    assert_eq!(t.scheme, 2);
    assert_eq!(t.color_green, "clLime");
}

#[test]
fn uc012_defaults() {
    let t = Theme::default();
    assert_eq!(t.scheme, 1);
}

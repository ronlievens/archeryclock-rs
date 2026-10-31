#[derive(Clone, Debug)]
pub struct Theme {
    pub color_green: String,
    pub color_orange: String,
    pub color_red: String,
    pub scheme: i32,
}

impl Default for Theme {
    fn default() -> Self {
        Theme { color_green: "clAqua".to_string(), color_orange: "clYellow".to_string(), color_red: "clRed".to_string(), scheme: 1 }
    }
}

pub fn set_scheme(t: &mut Theme, scheme: i32) {
    t.scheme = scheme;
    match scheme {
        1 => { t.color_green = "clAqua".to_string(); t.color_orange = "clYellow".to_string(); t.color_red = "clRed".to_string(); },
        2 => { t.color_green = "clLime".to_string(); t.color_orange = "clOrange".to_string(); t.color_red = "clRed".to_string(); },
        _ => {}
    }
}

use crate::config::paths::Paths;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone)]
pub struct Settings {
    pub language: String,
    pub time_format: String,
    pub hide_icons: bool,
    pub sound_mode: String,
    pub startup_scenario: String,
    pub hints: bool,
    pub traffic_lights: String,
    pub traffic_lights_pos: String,
    pub sound_name: String,
    pub pre_count_minutes: i32,
    pub hw_interface: String,
    pub serial_port1: String,
    pub serial_arduino1: String,
    pub pre_count_enabled: String,
    pub network_port: String,
    pub network_enabled: String,
    pub multicast_ip1: i32,
    pub multicast_ip2: i32,
    pub multicast_ip3: i32,
    pub multicast_ip4: i32,
    pub serial_port2: String,
    pub serial_arduino2: String,
    pub val1: i32,
    pub val2: i32,
    pub val3: i32,
    pub val4: i32,
    pub val5: i32,
    pub val6: i32,
    pub val7: i32,
    pub val8: i32,
    pub val9: i32,
    pub val10: i32,
    pub val11: i32,
    pub color_scheme: String,
    pub val12: i32,
    pub val13: i32,
    pub val14: i32,
    pub val15: i32,
    pub val16: i32,
    pub remote_mode: String,
    pub remote_name: String,
    pub val17: i32,
    pub f_keys_screen: String,
    pub emergency_alt_text: String,
    pub emergency_default: String,
    pub emergency_action: String,
    pub emergency_beeps: i32,
    pub emergency_keep_buzzing: String,
    pub zero_display: String,
    pub color_prep: String,
    pub color_green: String,
    pub color_black: String,
    pub color_gray: String,
    pub color_maroon: String,
    pub color_red: String,
    pub color_green2: String,
    pub color_lime: String,
    pub color_olive: String,
    pub color_yellow: String,
    pub color_lime2: String,
    pub color_red2: String,
    pub time_checkbox: String,
    pub am_pm: i32,
    pub banner_mode: String,
    pub banner_path: String,
    pub slides_mode: String,
    pub fullscreen: String,
    pub slides_seconds: i32,
    pub remote_shutdown: String,
    pub show_end_nr_slides: String,
    pub prelayout: String,
    pub music_player: String,
    pub music_host: String,
    pub music_port: i32,
    pub music_vol_state1: i32,
    pub music_action_state1: String,
    pub music_vol_state2: i32,
    pub music_action_state2: String,
    pub music_vol_state3: i32,
    pub music_action_state3: String,
    pub music_vol_state4: i32,
    pub music_action_state4: String,
    pub music_vol_state5: i32,
    pub music_action_state5: String,
    pub music_vol_state6: i32,
    pub music_action_state6: String,
    pub audio_control_enabled: String,
    pub val18: i32,
    pub fullscreen_windows: String,
    pub val19: i32,
    pub val20: i32,
    pub val21: i32,
    pub val22: i32,
    pub val23: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: if cfg!(windows) { "English".to_string() } else { "Nederlands".to_string() },
            time_format: "Sec".to_string(),
            hide_icons: false,
            sound_mode: "Buzzer".to_string(),
            startup_scenario: if cfg!(windows) { "shiftF5".to_string() } else { "--".to_string() },
            hints: true,
            traffic_lights: "TL".to_string(),
            traffic_lights_pos: "TLright".to_string(),
            sound_name: "AC1".to_string(),
            pre_count_minutes: 4,
            hw_interface: if cfg!(windows) { "K8055".to_string() } else { "na".to_string() },
            serial_port1: if cfg!(windows) { "COM1".to_string() } else { "/dev/ttyACM0".to_string() },
            serial_arduino1: "nonarduino1".to_string(),
            pre_count_enabled: "noPRECOUNT".to_string(),
            network_port: "4665".to_string(),
            network_enabled: "nonLAN".to_string(),
            multicast_ip1: 224,
            multicast_ip2: 25,
            multicast_ip3: 26,
            multicast_ip4: 27,
            serial_port2: if cfg!(windows) { "COM2".to_string() } else { "/dev/ttyUSB0".to_string() },
            serial_arduino2: "nonarduino2".to_string(),
            val1: 2,
            val2: 0,
            val3: 3,
            val4: 8,
            val5: 4,
            val6: 5,
            val7: 10,
            val8: 9,
            val9: 0,
            val10: 2,
            val11: 3,
            color_scheme: "col1".to_string(),
            val12: 3,
            val13: 1,
            val14: 0,
            val15: 4,
            val16: 5,
            remote_mode: "nonremote".to_string(),
            remote_name: "archery".to_string(),
            val17: 2,
            f_keys_screen: "Fnotonscreen".to_string(),
            emergency_alt_text: "Stop".to_string(),
            emergency_default: "def".to_string(),
            emergency_action: "pause".to_string(),
            emergency_beeps: 5,
            emergency_keep_buzzing: "notkeep".to_string(),
            zero_display: "nonzero".to_string(),
            color_prep: "clBlue".to_string(),
            color_green: "clAqua".to_string(),
            color_black: "clBlack".to_string(),
            color_gray: "clGray".to_string(),
            color_maroon: "clMaroon".to_string(),
            color_red: "clRed".to_string(),
            color_green2: "clGreen".to_string(),
            color_lime: "clLime".to_string(),
            color_olive: "clOlive".to_string(),
            color_yellow: "clYellow".to_string(),
            color_lime2: "clLime".to_string(),
            color_red2: "clRed".to_string(),
            time_checkbox: "FALSE".to_string(),
            am_pm: 0,
            banner_mode: "standard".to_string(),
            banner_path: "banner2.png".to_string(),
            slides_mode: "slides".to_string(),
            fullscreen: "nonfullscreen".to_string(),
            slides_seconds: 10,
            remote_shutdown: "TRUE".to_string(),
            show_end_nr_slides: "TRUE".to_string(),
            prelayout: "prelayout1".to_string(),
            music_player: "VLC".to_string(),
            music_host: "localhost".to_string(),
            music_port: 5005,
            music_vol_state1: 100,
            music_action_state1: "play".to_string(),
            music_vol_state2: 70,
            music_action_state2: "play".to_string(),
            music_vol_state3: 50,
            music_action_state3: "play".to_string(),
            music_vol_state4: 50,
            music_action_state4: "play".to_string(),
            music_vol_state5: 100,
            music_action_state5: "play".to_string(),
            music_vol_state6: 0,
            music_action_state6: "pause".to_string(),
            audio_control_enabled: "FALSE".to_string(),
            val18: 0,
            fullscreen_windows: if cfg!(windows) { "TRUE".to_string() } else { "FALSE".to_string() },
            val19: 4,
            val20: 5,
            val21: 6,
            val22: 7,
            val23: 0,
        }
    }
}

impl Settings {
    fn next(lines: &Vec<String>, i: &mut usize) -> String {
        if *i < lines.len() {
            let v = lines[*i].clone();
            *i += 1;
            v
        } else {
            String::new()
        }
    }

    pub fn load(paths: &Paths) -> std::io::Result<Self> {
        if !paths.settings_file.exists() {
            let d = Settings::default();
            d.save(paths)?;
            return Ok(d);
        }
        let f = File::open(&paths.settings_file)?;
        let r = BufReader::new(f);
        let mut lines: Vec<String> = Vec::new();
        for l in r.lines() {
            if let Ok(x) = l {
                lines.push(x.trim_end_matches('\r').to_string());
            }
        }
        let mut i = 0;
        let language = Self::next(&lines, &mut i);
        let time_format = Self::next(&lines, &mut i);
        let hide_icons_str = Self::next(&lines, &mut i);
        let sound_mode = Self::next(&lines, &mut i);
        let startup_scenario = Self::next(&lines, &mut i);
        let hints_str = Self::next(&lines, &mut i);
        let traffic_lights = Self::next(&lines, &mut i);
        let traffic_lights_pos = Self::next(&lines, &mut i);
        let sound_name = Self::next(&lines, &mut i);
        let pre_count_minutes: i32 = Self::next(&lines, &mut i).parse().unwrap_or(4);
        let hw_interface = Self::next(&lines, &mut i);
        let serial_port1 = Self::next(&lines, &mut i);
        let serial_arduino1 = Self::next(&lines, &mut i);
        let pre_count_enabled = Self::next(&lines, &mut i);
        let network_port = Self::next(&lines, &mut i);
        let network_enabled = Self::next(&lines, &mut i);
        let multicast_ip1: i32 = Self::next(&lines, &mut i).parse().unwrap_or(224);
        let multicast_ip2: i32 = Self::next(&lines, &mut i).parse().unwrap_or(25);
        let multicast_ip3: i32 = Self::next(&lines, &mut i).parse().unwrap_or(26);
        let multicast_ip4: i32 = Self::next(&lines, &mut i).parse().unwrap_or(27);
        let serial_port2 = Self::next(&lines, &mut i);
        let serial_arduino2 = Self::next(&lines, &mut i);
        let val1: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val2: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val3: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val4: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val5: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val6: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val7: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val8: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val9: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val10: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val11: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let color_scheme = Self::next(&lines, &mut i);
        let val12: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val13: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val14: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val15: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val16: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let remote_mode = Self::next(&lines, &mut i);
        let remote_name = Self::next(&lines, &mut i);
        let val17: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let f_keys_screen = Self::next(&lines, &mut i);
        let emergency_alt_text = Self::next(&lines, &mut i);
        let emergency_default = Self::next(&lines, &mut i);
        let emergency_action = Self::next(&lines, &mut i);
        let emergency_beeps: i32 = Self::next(&lines, &mut i).parse().unwrap_or(5);
        let emergency_keep_buzzing = Self::next(&lines, &mut i);
        let zero_display = Self::next(&lines, &mut i);
        let color_prep = Self::next(&lines, &mut i);
        let color_green = Self::next(&lines, &mut i);
        let color_black = Self::next(&lines, &mut i);
        let color_gray = Self::next(&lines, &mut i);
        let color_maroon = Self::next(&lines, &mut i);
        let color_red = Self::next(&lines, &mut i);
        let color_green2 = Self::next(&lines, &mut i);
        let color_lime = Self::next(&lines, &mut i);
        let color_olive = Self::next(&lines, &mut i);
        let color_yellow = Self::next(&lines, &mut i);
        let color_lime2 = Self::next(&lines, &mut i);
        let color_red2 = Self::next(&lines, &mut i);
        let time_checkbox = Self::next(&lines, &mut i);
        let am_pm: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let banner_mode = Self::next(&lines, &mut i);
        let banner_path = Self::next(&lines, &mut i);
        let slides_mode = Self::next(&lines, &mut i);
        let fullscreen = Self::next(&lines, &mut i);
        let slides_seconds: i32 = Self::next(&lines, &mut i).parse().unwrap_or(10);
        let remote_shutdown = Self::next(&lines, &mut i);
        let show_end_nr_slides = Self::next(&lines, &mut i);
        let prelayout = Self::next(&lines, &mut i);
        let music_player = Self::next(&lines, &mut i);
        let music_host = Self::next(&lines, &mut i);
        let music_port: i32 = Self::next(&lines, &mut i).parse().unwrap_or(5005);
        let music_vol_state1: i32 = Self::next(&lines, &mut i).parse().unwrap_or(100);
        let music_action_state1 = Self::next(&lines, &mut i);
        let music_vol_state2: i32 = Self::next(&lines, &mut i).parse().unwrap_or(70);
        let music_action_state2 = Self::next(&lines, &mut i);
        let music_vol_state3: i32 = Self::next(&lines, &mut i).parse().unwrap_or(50);
        let music_action_state3 = Self::next(&lines, &mut i);
        let music_vol_state4: i32 = Self::next(&lines, &mut i).parse().unwrap_or(50);
        let music_action_state4 = Self::next(&lines, &mut i);
        let music_vol_state5: i32 = Self::next(&lines, &mut i).parse().unwrap_or(100);
        let music_action_state5 = Self::next(&lines, &mut i);
        let music_vol_state6: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let music_action_state6 = Self::next(&lines, &mut i);
        let audio_control_enabled = Self::next(&lines, &mut i);
        let val18: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let fullscreen_windows = Self::next(&lines, &mut i);
        let val19: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val20: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val21: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val22: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);
        let val23: i32 = Self::next(&lines, &mut i).parse().unwrap_or(0);

        Ok(Settings {
            language,
            time_format,
            hide_icons: hide_icons_str == "Hide",
            sound_mode,
            startup_scenario,
            hints: hints_str == "Hint",
            traffic_lights,
            traffic_lights_pos,
            sound_name,
            pre_count_minutes,
            hw_interface,
            serial_port1,
            serial_arduino1,
            pre_count_enabled,
            network_port,
            network_enabled,
            multicast_ip1,
            multicast_ip2,
            multicast_ip3,
            multicast_ip4,
            serial_port2,
            serial_arduino2,
            val1,
            val2,
            val3,
            val4,
            val5,
            val6,
            val7,
            val8,
            val9,
            val10,
            val11,
            color_scheme,
            val12,
            val13,
            val14,
            val15,
            val16,
            remote_mode,
            remote_name,
            val17,
            f_keys_screen,
            emergency_alt_text,
            emergency_default,
            emergency_action,
            emergency_beeps,
            emergency_keep_buzzing,
            zero_display,
            color_prep,
            color_green,
            color_black,
            color_gray,
            color_maroon,
            color_red,
            color_green2,
            color_lime,
            color_olive,
            color_yellow,
            color_lime2,
            color_red2,
            time_checkbox,
            am_pm,
            banner_mode,
            banner_path,
            slides_mode,
            fullscreen,
            slides_seconds,
            remote_shutdown,
            show_end_nr_slides,
            prelayout,
            music_player,
            music_host,
            music_port,
            music_vol_state1,
            music_action_state1,
            music_vol_state2,
            music_action_state2,
            music_vol_state3,
            music_action_state3,
            music_vol_state4,
            music_action_state4,
            music_vol_state5,
            music_action_state5,
            music_vol_state6,
            music_action_state6,
            audio_control_enabled,
            val18,
            fullscreen_windows,
            val19,
            val20,
            val21,
            val22,
            val23,
        })
    }

    pub fn save(&self, paths: &Paths) -> std::io::Result<()> {
        let mut f = File::create(&paths.settings_file)?;
        writeln!(f, "{}", self.language)?;
        writeln!(f, "{}", self.time_format)?;
        writeln!(f, "{}", if self.hide_icons { "Hide" } else { "nonHide" })?;
        writeln!(f, "{}", self.sound_mode)?;
        writeln!(f, "{}", self.startup_scenario)?;
        writeln!(f, "{}", if self.hints { "Hint" } else { "nonHint" })?;
        writeln!(f, "{}", self.traffic_lights)?;
        writeln!(f, "{}", self.traffic_lights_pos)?;
        writeln!(f, "{}", self.sound_name)?;
        writeln!(f, "{}", self.pre_count_minutes)?;
        writeln!(f, "{}", self.hw_interface)?;
        writeln!(f, "{}", self.serial_port1)?;
        writeln!(f, "{}", self.serial_arduino1)?;
        writeln!(f, "{}", self.pre_count_enabled)?;
        writeln!(f, "{}", self.network_port)?;
        writeln!(f, "{}", self.network_enabled)?;
        writeln!(f, "{}", self.multicast_ip1)?;
        writeln!(f, "{}", self.multicast_ip2)?;
        writeln!(f, "{}", self.multicast_ip3)?;
        writeln!(f, "{}", self.multicast_ip4)?;
        writeln!(f, "{}", self.serial_port2)?;
        writeln!(f, "{}", self.serial_arduino2)?;
        writeln!(f, "{}", self.val1)?;
        writeln!(f, "{}", self.val2)?;
        writeln!(f, "{}", self.val3)?;
        writeln!(f, "{}", self.val4)?;
        writeln!(f, "{}", self.val5)?;
        writeln!(f, "{}", self.val6)?;
        writeln!(f, "{}", self.val7)?;
        writeln!(f, "{}", self.val8)?;
        writeln!(f, "{}", self.val9)?;
        writeln!(f, "{}", self.val10)?;
        writeln!(f, "{}", self.val11)?;
        writeln!(f, "{}", self.color_scheme)?;
        writeln!(f, "{}", self.val12)?;
        writeln!(f, "{}", self.val13)?;
        writeln!(f, "{}", self.val14)?;
        writeln!(f, "{}", self.val15)?;
        writeln!(f, "{}", self.val16)?;
        writeln!(f, "{}", self.remote_mode)?;
        writeln!(f, "{}", self.remote_name)?;
        writeln!(f, "{}", self.val17)?;
        writeln!(f, "{}", self.f_keys_screen)?;
        writeln!(f, "{}", self.emergency_alt_text)?;
        writeln!(f, "{}", self.emergency_default)?;
        writeln!(f, "{}", self.emergency_action)?;
        writeln!(f, "{}", self.emergency_beeps)?;
        writeln!(f, "{}", self.emergency_keep_buzzing)?;
        writeln!(f, "{}", self.zero_display)?;
        writeln!(f, "{}", self.color_prep)?;
        writeln!(f, "{}", self.color_green)?;
        writeln!(f, "{}", self.color_black)?;
        writeln!(f, "{}", self.color_gray)?;
        writeln!(f, "{}", self.color_maroon)?;
        writeln!(f, "{}", self.color_red)?;
        writeln!(f, "{}", self.color_green2)?;
        writeln!(f, "{}", self.color_lime)?;
        writeln!(f, "{}", self.color_olive)?;
        writeln!(f, "{}", self.color_yellow)?;
        writeln!(f, "{}", self.color_lime2)?;
        writeln!(f, "{}", self.color_red2)?;
        writeln!(f, "{}", self.time_checkbox)?;
        writeln!(f, "{}", self.am_pm)?;
        writeln!(f, "{}", self.banner_mode)?;
        writeln!(f, "{}", self.banner_path)?;
        writeln!(f, "{}", self.slides_mode)?;
        writeln!(f, "{}", self.fullscreen)?;
        writeln!(f, "{}", self.slides_seconds)?;
        writeln!(f, "{}", self.remote_shutdown)?;
        writeln!(f, "{}", self.show_end_nr_slides)?;
        writeln!(f, "{}", self.prelayout)?;
        writeln!(f, "{}", self.music_player)?;
        writeln!(f, "{}", self.music_host)?;
        writeln!(f, "{}", self.music_port)?;
        writeln!(f, "{}", self.music_vol_state1)?;
        writeln!(f, "{}", self.music_action_state1)?;
        writeln!(f, "{}", self.music_vol_state2)?;
        writeln!(f, "{}", self.music_action_state2)?;
        writeln!(f, "{}", self.music_vol_state3)?;
        writeln!(f, "{}", self.music_action_state3)?;
        writeln!(f, "{}", self.music_vol_state4)?;
        writeln!(f, "{}", self.music_action_state4)?;
        writeln!(f, "{}", self.music_vol_state5)?;
        writeln!(f, "{}", self.music_action_state5)?;
        writeln!(f, "{}", self.music_vol_state6)?;
        writeln!(f, "{}", self.music_action_state6)?;
        writeln!(f, "{}", self.audio_control_enabled)?;
        writeln!(f, "{}", self.val18)?;
        writeln!(f, "{}", self.fullscreen_windows)?;
        writeln!(f, "{}", self.val19)?;
        writeln!(f, "{}", self.val20)?;
        writeln!(f, "{}", self.val21)?;
        writeln!(f, "{}", self.val22)?;
        writeln!(f, "{}", self.val23)?;
        Ok(())
    }
}

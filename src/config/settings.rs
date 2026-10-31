use crate::config::paths::Paths;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Clone, Debug)]
pub struct Settings {
    pub language: String,
    pub time_format: String,
    pub hide_icons: bool,
    pub hints: bool,
    pub pre_count_minutes: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: if cfg!(windows) { "English".to_string() } else { "Nederlands".to_string() },
            time_format: "Sec".to_string(),
            hide_icons: false,
            hints: true,
            pre_count_minutes: 4,
        }
    }
}

impl Settings {
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
        let language = if !lines.is_empty() { lines[0].clone() } else { Settings::default().language };
        let time_format = if lines.len() > 1 { lines[1].clone() } else { Settings::default().time_format };
        let hide_icons_str = if lines.len() > 2 { lines[2].clone() } else { "nonHide".to_string() };
        let hints_str = if lines.len() > 5 { lines[5].clone() } else { "Hint".to_string() };
        let pre_count_minutes: i32 = if lines.len() > 9 { lines[9].parse().unwrap_or(4) } else { 4 };
        Ok(Settings {
            language,
            time_format,
            hide_icons: hide_icons_str == "Hide",
            hints: hints_str == "Hint",
            pre_count_minutes,
        })
    }

    pub fn save(&self, paths: &Paths) -> std::io::Result<()> {
        let mut f = File::create(&paths.settings_file)?;
        writeln!(f, "{}", self.language)?;
        writeln!(f, "{}", self.time_format)?;
        writeln!(f, "{}", if self.hide_icons { "Hide" } else { "nonHide" })?;
        writeln!(f, "Buzzer")?;
        writeln!(f, "shiftF5")?;
        writeln!(f, "{}", if self.hints { "Hint" } else { "nonHint" })?;
        writeln!(f, "TL")?;
        writeln!(f, "TLright")?;
        writeln!(f, "AC1")?;
        writeln!(f, "{}", self.pre_count_minutes)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Archery {
    pub archerysystem: String,
    pub followsystem: String,
}

impl Default for Archery {
    fn default() -> Self {
        Archery {
            archerysystem: "fita".to_string(),
            followsystem: "fita".to_string(),
        }
    }
}

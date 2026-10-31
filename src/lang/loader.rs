use crate::config::paths::Paths;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct LanguageLoader {
    pub strings: HashMap<String, String>,
}

impl LanguageLoader {
    pub fn load(paths: &Paths, lang: &str) -> std::io::Result<Self> {
        let lang_path = paths.language_dir.join(lang);
        let mut strings = HashMap::new();
        if lang_path.exists() {
            let f = File::open(&lang_path)?;
            let r = BufReader::new(f);
            for line in r.lines() {
                if let Ok(l) = line {
                    let l = l.trim_end_matches('\r').to_string();
                    if l.contains('=') {
                        let mut p = l.splitn(2, '=');
                        if let (Some(k), Some(v)) = (p.next(), p.next()) {
                            strings.insert(k.to_string(), v.to_string());
                        }
                    } else if !l.is_empty() {
                        strings.insert(l.clone(), l);
                    }
                }
            }
        }
        Ok(LanguageLoader { strings })
    }
}

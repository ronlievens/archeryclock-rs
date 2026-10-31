use crate::config::paths::Paths;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Scenario {
    pub name: String,
    pub lines: Vec<String>,
}

#[derive(Debug)]
pub struct ScenarioLoader {
    pub scenarios: HashMap<String, Scenario>,
}

impl ScenarioLoader {
    pub fn init(paths: &Paths) -> std::io::Result<Self> {
        let mut scenarios = HashMap::new();
        if paths.scenarios_dir.exists() {
            for i in 1..=12 {
                let name = format!("F{}", i);
                let p = paths.scenarios_dir.join(&name);
                if p.exists() {
                    if let Ok(s) = Self::load_scenario(name.clone(), &p) {
                        scenarios.insert(name, s);
                    }
                }
            }
            let gen = paths.scenarios_dir.join("gen");
            if gen.exists() {
                if let Ok(s) = Self::load_scenario("gen".to_string(), &gen) {
                    scenarios.insert("gen".to_string(), s);
                }
            }
        }
        Ok(ScenarioLoader { scenarios })
    }

    fn load_scenario(name: String, path: &std::path::Path) -> std::io::Result<Scenario> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        let mut lines: Vec<String> = Vec::new();
        for l in r.lines() {
            if let Ok(x) = l {
                lines.push(x.trim_end_matches('\r').to_string());
            }
        }
        Ok(Scenario { name, lines })
    }
}

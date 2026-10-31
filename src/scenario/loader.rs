use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: String,
    pub content: Vec<String>,
}

pub fn load_scenario(dir: &Path, name: &str) -> std::io::Result<Scenario> {
    let path = dir.join(name);
    let text = fs::read_to_string(path)?;
    let content = text.lines().map(|l| l.trim_end_matches('\r').to_string()).collect();
    Ok(Scenario { name: name.to_string(), content })
}

pub fn save_scenario(dir: &Path, name: &str, content: &[String]) -> std::io::Result<()> {
    let path = dir.join(name);
    let mut data = String::new();
    for line in content {
        data.push_str(line);
        data.push('\n');
    }
    fs::write(path, data)
}

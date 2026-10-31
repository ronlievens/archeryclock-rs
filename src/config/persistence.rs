use std::fs;
use std::path::Path;

pub fn read_lines(p: &Path) -> std::io::Result<Vec<String>> {
    let text = fs::read_to_string(p)?;
    Ok(text.lines().map(|l| l.trim_end_matches('\r').to_string()).collect())
}

pub fn write_lines(p: &Path, lines: &[String]) -> std::io::Result<()> {
    let mut data = String::new();
    for l in lines {
        data.push_str(l);
        data.push('\n');
    }
    fs::write(p, data)
}

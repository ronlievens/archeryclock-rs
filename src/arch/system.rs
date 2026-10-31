use crate::config::settings::Archery;

pub fn init_system(archerysystem: &str) -> Archery {
    match archerysystem {
        "fita" => Archery { archerysystem: "fita".to_string(), followsystem: "fita".to_string() },
        "fitafin" => Archery { archerysystem: "fitafin".to_string(), followsystem: "fitafin".to_string() },
        "25m1p" => Archery { archerysystem: "25m1p".to_string(), followsystem: "25m1p".to_string() },
        "manual" => Archery { archerysystem: "manual".to_string(), followsystem: "manual".to_string() },
        "follow" => Archery { archerysystem: "follow".to_string(), followsystem: "fita".to_string() },
        _ => Archery { archerysystem: "fita".to_string(), followsystem: "fita".to_string() },
    }
}

pub fn is_fita(a: &Archery) -> bool { a.archerysystem == "fita" }
pub fn is_fitafin(a: &Archery) -> bool { a.archerysystem == "fitafin" }
pub fn is_25m1p(a: &Archery) -> bool { a.archerysystem == "25m1p" }
pub fn is_manual(a: &Archery) -> bool { a.archerysystem == "manual" }
pub fn is_follow(a: &Archery) -> bool { a.archerysystem == "follow" }

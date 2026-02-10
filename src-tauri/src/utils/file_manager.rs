use std::fs;
use std::path::{Path, PathBuf};

pub fn check_and_create_dir(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_app_config_dir() -> Option<PathBuf> {
    // Check for LOCALAPPDATA environment variable (Windows)
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let mut path = PathBuf::from(local_app_data);
        path.push("SSMT4Configs");
        return Some(path);
    }
    
    // Simple fallback for Linux/Mac if needed, though user is on Windows
    if let Ok(home) = std::env::var("HOME") {
        let mut path = PathBuf::from(home);
        path.push(".config");
        path.push("SSMT4Configs");
        return Some(path);
    }

    None
}

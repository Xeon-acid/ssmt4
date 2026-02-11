use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BasicSettings {
    pub game_preset: String,
    #[serde(default = "default_bg_type")]
    pub background_type: String, 
}

fn default_bg_type() -> String {
    "image".to_string()
}

impl Default for BasicSettings {
    fn default() -> Self {
        Self {
            game_preset: "Default".to_string(),
            background_type: "image".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
    #[serde(default)]
    pub basic: BasicSettings,
    #[serde(default)]
    pub three_d_migoto: serde_json::Value,
    #[serde(default)]
    pub other: serde_json::Value,
}

fn get_game_config_path(app: &AppHandle, game_name: &str) -> PathBuf {
    // Helper from game_scanner (but private there), so we duplicate or move logic. 
    // Ideally we share "find_games_dir" logic.
    // For now, let's look at how "scan_games" finds it. 
    // It's better to expose "find_games_dir" from game_scanner or move it to common utils.
    // Or just re-implement simple check for now since I can't easily refactor shared code without multiple edits.
    
    // We will assume the standard location logic:
    // 1. resource_dir/Games
    // 2. adjacent Games folder
    
    let mut games_dir = PathBuf::from("Games"); // Default fallback
    
    if let Ok(resource_dir) = app.path().resource_dir() {
        let p = resource_dir.join("Games");
        if p.exists() {
            games_dir = p;
        }
    }
    
    // Check adjacent to exe if not found in resources (dev mode fallback mostly)
    if !games_dir.exists() {
         if let Ok(mut exec_dir) = std::env::current_exe() {
            exec_dir.pop(); 
            let p1 = exec_dir.join("resources").join("Games");
            if p1.exists() { games_dir = p1; }
            else if exec_dir.join("Games").exists() { games_dir = exec_dir.join("Games"); }
        }
    }
    
    games_dir.join(game_name).join("Config.json")
}

#[tauri::command]
pub fn load_game_config(app: AppHandle, game_name: String) -> Result<GameConfig, String> {
    let config_path = get_game_config_path(&app, &game_name);
    
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        let config: GameConfig = serde_json::from_str(&content)
             .map_err(|e| format!("Failed to parse config: {}", e))?;
             
        Ok(config)
    } else {
        // Return default if not exists
        Ok(GameConfig::default())
    }
}

#[tauri::command]
pub fn save_game_config(app: AppHandle, game_name: String, config: GameConfig) -> Result<(), String> {
    let config_path = get_game_config_path(&app, &game_name);
    
    // Ensure parent dir exists (it should, since game exists, but just in case)
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
    fs::write(config_path, content)
        .map_err(|e| format!("Failed to write config: {}", e))?;
        
    Ok(())
}

#[tauri::command]
pub fn create_new_config(app: AppHandle, new_name: String, config: GameConfig) -> Result<(), String> {
    // Reuse specific path logic or just construct it
    // We assume "Games/<new_name>/Config.json"
    
    // Find Games dir root
    let games_root = get_game_config_path(&app, "Dummy").parent().unwrap().parent().unwrap().to_path_buf();
    
    let new_dir = games_root.join(&new_name);
    if !new_dir.exists() {
        fs::create_dir_all(&new_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    
    let config_path = new_dir.join("Config.json");
    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
    fs::write(config_path, content)
        .map_err(|e| format!("Failed to write config: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_game_config_folder(app: AppHandle, game_name: String) -> Result<(), String> {
    let config_path = get_game_config_path(&app, &game_name);
    let game_dir = config_path.parent().ok_or("Invalid path")?;
    
    if game_dir.exists() {
        fs::remove_dir_all(game_dir)
             .map_err(|e| format!("Failed to delete directory: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn set_game_background(app: AppHandle, game_name: String, file_path: String, bg_type: String) -> Result<(), String> {
    let config_path = get_game_config_path(&app, &game_name);
    let game_dir = config_path.parent().ok_or("Invalid game path")?;
    
    let source_path = PathBuf::from(&file_path);
    if !source_path.exists() {
        return Err(format!("Source file does not exist: {}", file_path));
    }
    
    let extension = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
        
    let target_name = format!("Background.{}", extension);
    let target_path = game_dir.join(&target_name);

    if bg_type == "image" {
        let candidates = ["Background.png", "Background.webp", "Background.jpg", "Background.jpeg"]; 
        for c in candidates {
            let p = game_dir.join(c);
            if p.exists() { let _ = fs::remove_file(p); }
        }
    } else if bg_type == "video" {
        let candidates = ["Background.mp4", "Background.webm", "Background.mkv"]; 
        for c in candidates {
            let p = game_dir.join(c);
            if p.exists() { let _ = fs::remove_file(p); }
        }
    } else {
        return Err(format!("Unknown background type: {}", bg_type));
    }

    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("Failed to copy file from {} to {:?}: {}", file_path, target_path, e))?;
        
    Ok(())
}

#[tauri::command]
pub fn set_game_icon(app: AppHandle, game_name: String, file_path: String) -> Result<(), String> {
    let config_path = get_game_config_path(&app, &game_name);
    let game_dir = config_path.parent().ok_or("Invalid game path")?;
    
    let source_path = PathBuf::from(&file_path);
    if !source_path.exists() { return Err(format!("File not found: {}", file_path)); }
    
    let target_path = game_dir.join("Icon.png");
    
    if target_path.exists() {
        let _ = fs::remove_file(&target_path);
    }
    
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("Failed to copy icon: {}", e))?;
        
    Ok(())
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct NoteData {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn save_note(dir: &PathBuf, content: String) -> Result<(), String> {
    let data = NoteData { content };
    let json_string = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(dir.join("notes.json"), json_string).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_note(dir: &PathBuf) -> Result<NoteData, String> {
    let json_string = fs::read_to_string(dir.join("notes.json"))
        .unwrap_or_else(|_| serde_json::to_string(&NoteData { content: String::new() }).unwrap());
    let data: NoteData = serde_json::from_str(&json_string).map_err(|e| e.to_string())?;
    Ok(data)
}

pub fn save_config(dir: &PathBuf, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
    let data = ConfigData { x, y, width, height };
    let json_string = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(dir.join("config.json"), json_string).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_config(dir: &PathBuf) -> Result<ConfigData, String> {
    let default = ConfigData { x: 100, y: 100, width: 500, height: 600 };
    let json_string = fs::read_to_string(dir.join("config.json"))
        .unwrap_or_else(|_| serde_json::to_string(&default).unwrap());
    let data: ConfigData = serde_json::from_str(&json_string).map_err(|e| e.to_string())?;
    Ok(data)
}
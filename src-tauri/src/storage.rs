use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub x: i32, 
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn save_config(x: i32, y: i32, width: u32, height: u32)-> Result<(), String>{
    let data = ConfigData{ x, y, width, height};
    let json_string = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write("data/config.json", json_string).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_config() -> Result<ConfigData, String>{
    let default = ConfigData{x: 100, y: 100, width: 300, height: 400};
    let json_string = fs::read_to_string("data/config.json")/unwrap_or_else(|_| serde_json::to_string(&default).unwrap());

    let data: ConfigData = serde_json::from_str(&json_string).map_err(|e| e.to_string())?;

    Ok(data)

}

#[derive(Serialize, Deserialize)]
pub struct NoteData {
    pub content: String,
}

pub fn save_note(content: String) -> Result<(), String> {
    let data = NoteData{ content };
    let json_string = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;

    fs::write("data/notes.json", json_string).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_note() -> Result<NoteData, String>{
    let json_string =  fs::read_to_string("data/notes.json").unwrap_or_else(|_| String::from("{\"content\":\"\"}"));

    let data: NoteData = serde_json::from_str(&json_string).map_err(|e| e.to_string())?;

    Ok(data)
}
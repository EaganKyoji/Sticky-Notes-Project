use serde::{Deserialize, Serialize};
use std::fs;

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
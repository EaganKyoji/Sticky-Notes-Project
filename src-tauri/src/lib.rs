// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod storage;
use storage::{NoteData, ConfigData};


#[tauri::command]
fn save_note(content: String) -> Result<(), String>{
    storage::save_note(content)
}

#[tauri::command]
fn load_note() -> Result<NoteData, String>{
    storage::load_note()
}

#[tauri::command]
fn save_config(x: i32, y: i32, width: u32, height: u32) -> Result<(), String>{
    storage::save_config(x, y, width, height)
}

#[tauri::command]
fn load_config() -> Result<ConfigData, String>{
    storage::load_config()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![save_note, load_note, save_config, load_config]).run(tauri::generate_context!()).expect("error saat menjalankan aplikasi");
}

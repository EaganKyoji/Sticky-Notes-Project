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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![save_note, load_note]).run(tauri::generate_context!()).expect("error saat menjalankan aplikasi");
}

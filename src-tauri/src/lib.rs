// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod storage;
use storage::{NoteData, ConfigData};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn save_note(content: String) -> Result<(), String> {
    storage::save_note(content)
}

#[tauri::command]
fn load_note() -> Result<NoteData, String> {
    storage::load_note()
}

#[tauri::command]
fn save_config(x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
    storage::save_config(x, y, width, height)
}

#[tauri::command]
fn load_config() -> Result<ConfigData, String> {
    storage::load_config()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new().with_handler(|app, shortcut, event| {
                let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyN);
                if shortcut == &toggle_shortcut && event.state() == ShortcutState::Pressed {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            })
            .build()
        )
        .setup(|app| {
            let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyN);
            app.global_shortcut().register(toggle_shortcut)?;
            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();

            let show_item = MenuItem::with_id(app, "show", "Tampilkan", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Sembunyikan", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Keluar", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_item, &hide_item, &separator, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            let main_window = app.get_webview_window("main").unwrap();
            let window_clone = main_window.clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    window_clone.hide().unwrap();
                    api.prevent_close();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_note, load_note, save_config, load_config])
        .run(tauri::generate_context!())
        .expect("error saat menjalankan aplikasi");
}
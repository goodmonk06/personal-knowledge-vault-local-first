// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;
mod crypto;

use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    db: Mutex<Option<db::Database>>,
}

#[tauri::command]
fn initialize_db(state: State<AppState>, password: String) -> Result<String, String> {
    let mut db_lock = state.db.lock().unwrap();

    match db::Database::new(&password) {
        Ok(database) => {
            *db_lock = Some(database);
            Ok("Database initialized successfully".to_string())
        }
        Err(e) => Err(format!("Failed to initialize database: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            db: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            initialize_db,
            commands::create_note,
            commands::get_note,
            commands::update_note,
            commands::delete_note,
            commands::list_notes,
            commands::search_notes,
            commands::add_tag_to_note,
            commands::remove_tag_from_note,
            commands::get_all_tags,
            commands::export_encrypted_archive,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

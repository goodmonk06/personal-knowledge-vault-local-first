use crate::db::{Note, Tag};
use crate::{crypto, AppState};
use tauri::State;

#[tauri::command]
pub fn create_note(
    state: State<AppState>,
    title: String,
    content: String,
) -> Result<i64, String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .create_note(&title, &content)
            .map_err(|e| format!("Failed to create note: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn get_note(state: State<AppState>, id: i64) -> Result<Note, String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .get_note(id)
            .map_err(|e| format!("Failed to get note: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn update_note(
    state: State<AppState>,
    id: i64,
    title: String,
    content: String,
) -> Result<(), String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .update_note(id, &title, &content)
            .map_err(|e| format!("Failed to update note: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn delete_note(state: State<AppState>, id: i64) -> Result<(), String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .delete_note(id)
            .map_err(|e| format!("Failed to delete note: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn list_notes(state: State<AppState>) -> Result<Vec<Note>, String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .list_notes()
            .map_err(|e| format!("Failed to list notes: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn search_notes(state: State<AppState>, query: String) -> Result<Vec<Note>, String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .search_notes(&query)
            .map_err(|e| format!("Failed to search notes: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn add_tag_to_note(
    state: State<AppState>,
    note_id: i64,
    tag_name: String,
) -> Result<(), String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .add_tag_to_note(note_id, &tag_name)
            .map_err(|e| format!("Failed to add tag to note: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn remove_tag_from_note(
    state: State<AppState>,
    note_id: i64,
    tag_name: String,
) -> Result<(), String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .remove_tag_from_note(note_id, &tag_name)
            .map_err(|e| format!("Failed to remove tag from note: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub fn get_all_tags(state: State<AppState>) -> Result<Vec<Tag>, String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => db
            .get_all_tags()
            .map_err(|e| format!("Failed to get all tags: {}", e)),
        None => Err("Database not initialized".to_string()),
    }
}

#[tauri::command]
pub async fn export_encrypted_archive(
    state: State<'_, AppState>,
    password: String,
    output_path: String,
) -> Result<String, String> {
    // This is a placeholder implementation
    // In a full implementation, this would:
    // 1. Export all notes from the database
    // 2. Encrypt the data using the provided password
    // 3. Create a compressed archive
    // 4. Save to the specified output path

    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => {
            let notes = db
                .list_notes()
                .map_err(|e| format!("Failed to list notes: {}", e))?;

            // Serialize notes to JSON
            let json_data = serde_json::to_string(&notes)
                .map_err(|e| format!("Failed to serialize notes: {}", e))?;

            // Encrypt the data
            let encrypted_data = crypto::encrypt(&json_data, &password)
                .map_err(|e| format!("Failed to encrypt data: {}", e))?;

            // Write to file
            std::fs::write(&output_path, encrypted_data)
                .map_err(|e| format!("Failed to write file: {}", e))?;

            Ok(format!(
                "Successfully exported {} notes to {}",
                notes.len(),
                output_path
            ))
        }
        None => Err("Database not initialized".to_string()),
    }
}

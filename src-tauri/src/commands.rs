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

#[tauri::command]
pub fn seed_database(state: State<AppState>) -> Result<String, String> {
    let db_lock = state.db.lock().unwrap();

    match &*db_lock {
        Some(db) => {
            // Check if database already has notes
            let existing_notes = db
                .list_notes()
                .map_err(|e| format!("Failed to check existing notes: {}", e))?;

            if !existing_notes.is_empty() {
                return Ok("Database already contains notes. Skipping seed.".to_string());
            }

            // Create welcome note
            let welcome_id = db
                .create_note(
                    "Welcome to Personal Knowledge Vault",
                    r#"This is your personal, local-first knowledge management system.

## Features

- **Create Notes**: Click "+ 新しいノート" to create a new note
- **Tag Notes**: Organize your notes with tags for easy categorization
- **Search**: Use the search box to find notes by title or content
- **Export**: Create encrypted backups to external storage

## Getting Started

1. Create your first note by clicking the button in the sidebar
2. Add tags to organize your notes
3. Use the search feature to quickly find information
4. Export your data regularly for backup

This app stores all data locally on your device. No cloud services are involved, giving you complete control over your information."#,
                )
                .map_err(|e| format!("Failed to create welcome note: {}", e))?;

            db.add_tag_to_note(welcome_id, "important")
                .map_err(|e| format!("Failed to add tag: {}", e))?;

            // Create example notes
            let example1_id = db
                .create_note(
                    "Project Ideas",
                    r#"## New Feature Ideas

- Markdown editor support
- File attachments
- Multiple vaults
- Mobile companion app
- Cloud sync (optional)

Remember to keep notes concise and well-organized with tags!"#,
                )
                .map_err(|e| format!("Failed to create example note 1: {}", e))?;

            db.add_tag_to_note(example1_id, "ideas")
                .map_err(|e| format!("Failed to add tag: {}", e))?;
            db.add_tag_to_note(example1_id, "work")
                .map_err(|e| format!("Failed to add tag: {}", e))?;

            let example2_id = db
                .create_note(
                    "Meeting Notes Template",
                    r#"## Meeting: [Topic]
**Date**: [Date]
**Attendees**: [Names]

### Agenda
1. 
2. 
3. 

### Discussion Points
- 

### Action Items
- [ ] 
- [ ] 

### Next Steps
- "#,
                )
                .map_err(|e| format!("Failed to create example note 2: {}", e))?;

            db.add_tag_to_note(example2_id, "work")
                .map_err(|e| format!("Failed to add tag: {}", e))?;

            let example3_id = db
                .create_note(
                    "Learning Resources",
                    r#"## Topics to Study

### Programming
- Rust advanced patterns
- React performance optimization
- Database design

### Personal Development
- Time management techniques
- Note-taking systems
- Knowledge organization

Use tags to categorize learning materials by topic or priority."#,
                )
                .map_err(|e| format!("Failed to create example note 3: {}", e))?;

            db.add_tag_to_note(example3_id, "personal")
                .map_err(|e| format!("Failed to add tag: {}", e))?;

            Ok("Successfully seeded database with 4 example notes and tags!".to_string())
        }
        None => Err("Database not initialized".to_string()),
    }
}

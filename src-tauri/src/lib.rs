mod crypto;
mod db;
mod models;

use crate::db::Database;
use crate::models::{CreateNoteRequest, Note, NoteWithTags, UpdateNoteRequest};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};

/// アプリケーション状態
pub struct AppState {
    db: Arc<Mutex<Database>>,
}

/// データベースパスを取得
fn get_db_path(app_handle: &AppHandle) -> PathBuf {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");

    app_data_dir.join("knowledge_vault.db")
}

/// ノートを作成
#[tauri::command]
fn create_note(
    request: CreateNoteRequest,
    state: State<AppState>,
) -> Result<Note, String> {
    let note = Note::new(request.title, request.content);
    let db = state.db.lock().unwrap();

    db.create_note(&note, &request.tags)
        .map_err(|e| e.to_string())?;

    Ok(note)
}

/// ノートを更新
#[tauri::command]
fn update_note(
    request: UpdateNoteRequest,
    state: State<AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();

    let mut note = Note::new(request.title, request.content);
    note.id = request.id;
    note.updated_at = chrono::Utc::now();

    db.update_note(&note, &request.tags)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// ノートを削除
#[tauri::command]
fn delete_note(note_id: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_note(&note_id).map_err(|e| e.to_string())?;
    Ok(())
}

/// 全ノートを取得
#[tauri::command]
fn get_all_notes(state: State<AppState>) -> Result<Vec<NoteWithTags>, String> {
    let db = state.db.lock().unwrap();
    db.get_all_notes().map_err(|e| e.to_string())
}

/// ノートを検索
#[tauri::command]
fn search_notes(query: String, state: State<AppState>) -> Result<Vec<NoteWithTags>, String> {
    let db = state.db.lock().unwrap();
    db.search_notes(&query).map_err(|e| e.to_string())
}

/// 全タグを取得
#[tauri::command]
fn get_all_tags(state: State<AppState>) -> Result<Vec<String>, String> {
    let db = state.db.lock().unwrap();
    db.get_all_tags().map_err(|e| e.to_string())
}

/// データベースをエクスポート（暗号化アーカイブとして）
#[tauri::command]
fn export_archive(
    output_path: String,
    password: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    use std::fs::File;
    use std::io::Write;
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let db_path = get_db_path(&app_handle);

    // 一時的なZIPファイルを作成
    let temp_zip_path = app_handle
        .path()
        .temp_dir()
        .expect("Failed to get temp directory")
        .join("export.zip");

    {
        let file = File::create(&temp_zip_path).map_err(|e| e.to_string())?;
        let mut zip = ZipWriter::new(file);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // データベースファイルをZIPに追加
        zip.start_file("knowledge_vault.db", options)
            .map_err(|e| e.to_string())?;
        let db_data = fs::read(&db_path).map_err(|e| e.to_string())?;
        zip.write_all(&db_data).map_err(|e| e.to_string())?;

        // メタデータを追加
        let metadata = serde_json::json!({
            "version": "0.1.0",
            "exported_at": chrono::Utc::now().to_rfc3339(),
        });
        zip.start_file("metadata.json", options)
            .map_err(|e| e.to_string())?;
        zip.write_all(metadata.to_string().as_bytes())
            .map_err(|e| e.to_string())?;

        zip.finish().map_err(|e| e.to_string())?;
    }

    // ZIPファイルを暗号化
    let zip_data = fs::read(&temp_zip_path).map_err(|e| e.to_string())?;
    let encrypted = crypto::encrypt_data(&zip_data, &password).map_err(|e| e.to_string())?;
    fs::write(&output_path, encrypted).map_err(|e| e.to_string())?;

    // 一時ファイルを削除
    fs::remove_file(&temp_zip_path).ok();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let db_path = get_db_path(app.handle());
            let db = Database::new(db_path).expect("Failed to initialize database");

            app.manage(AppState {
                db: Arc::new(Mutex::new(db)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_note,
            update_note,
            delete_note,
            get_all_notes,
            search_notes,
            get_all_tags,
            export_archive,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

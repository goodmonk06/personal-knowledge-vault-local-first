use crate::models::{Note, NoteWithTags, Tag};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// データベースマネージャー
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// データベースを初期化
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)
            .context("Failed to open database")?;

        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        db.initialize_schema()?;
        Ok(db)
    }

    /// スキーマを初期化
    fn initialize_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // ノートテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // タグテーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                name TEXT UNIQUE NOT NULL
            )",
            [],
        )?;

        // ノート-タグ関連テーブル
        conn.execute(
            "CREATE TABLE IF NOT EXISTS note_tags (
                note_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (note_id, tag_id),
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // 全文検索用インデックス
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_notes_title ON notes(title)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name)",
            [],
        )?;

        Ok(())
    }

    /// ノートを作成
    pub fn create_note(&self, note: &Note, tag_names: &[String]) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &note.id,
                &note.title,
                &note.content,
                note.created_at.to_rfc3339(),
                note.updated_at.to_rfc3339(),
            ],
        )?;

        // タグを追加
        for tag_name in tag_names {
            let tag_id = self.get_or_create_tag_internal(&conn, tag_name)?;
            conn.execute(
                "INSERT INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
                params![&note.id, &tag_id],
            )?;
        }

        Ok(())
    }

    /// ノートを更新
    pub fn update_note(&self, note: &Note, tag_names: &[String]) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            params![
                &note.title,
                &note.content,
                note.updated_at.to_rfc3339(),
                &note.id,
            ],
        )?;

        // 既存のタグを削除
        conn.execute(
            "DELETE FROM note_tags WHERE note_id = ?1",
            params![&note.id],
        )?;

        // 新しいタグを追加
        for tag_name in tag_names {
            let tag_id = self.get_or_create_tag_internal(&conn, tag_name)?;
            conn.execute(
                "INSERT INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
                params![&note.id, &tag_id],
            )?;
        }

        Ok(())
    }

    /// ノートを削除
    pub fn delete_note(&self, note_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notes WHERE id = ?1", params![note_id])?;
        Ok(())
    }

    /// 全ノートを取得
    pub fn get_all_notes(&self) -> Result<Vec<NoteWithTags>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get::<_, String>(3)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse::<DateTime<Utc>>().unwrap(),
            })
        })?;

        let mut result = Vec::new();
        for note in notes {
            let note = note?;
            let tags = self.get_note_tags_internal(&conn, &note.id)?;
            result.push(NoteWithTags { note, tags });
        }

        Ok(result)
    }

    /// ノートを検索
    pub fn search_notes(&self, query: &str) -> Result<Vec<NoteWithTags>> {
        let conn = self.conn.lock().unwrap();
        let search_pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes
             WHERE title LIKE ?1 OR content LIKE ?1
             ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map(params![search_pattern], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get::<_, String>(3)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse::<DateTime<Utc>>().unwrap(),
            })
        })?;

        let mut result = Vec::new();
        for note in notes {
            let note = note?;
            let tags = self.get_note_tags_internal(&conn, &note.id)?;
            result.push(NoteWithTags { note, tags });
        }

        Ok(result)
    }

    /// タグを取得または作成（内部メソッド）
    fn get_or_create_tag_internal(&self, conn: &Connection, tag_name: &str) -> Result<String> {
        // 既存のタグを検索
        let mut stmt = conn.prepare("SELECT id FROM tags WHERE name = ?1")?;
        let mut rows = stmt.query(params![tag_name])?;

        if let Some(row) = rows.next()? {
            Ok(row.get(0)?)
        } else {
            // 新規タグを作成
            let tag = Tag::new(tag_name.to_string());
            conn.execute(
                "INSERT INTO tags (id, name) VALUES (?1, ?2)",
                params![&tag.id, &tag.name],
            )?;
            Ok(tag.id)
        }
    }

    /// ノートのタグを取得（内部メソッド）
    fn get_note_tags_internal(&self, conn: &Connection, note_id: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT t.name FROM tags t
             INNER JOIN note_tags nt ON t.id = nt.tag_id
             WHERE nt.note_id = ?1"
        )?;

        let tags = stmt.query_map(params![note_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        Ok(tags)
    }

    /// 全タグを取得
    pub fn get_all_tags(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name FROM tags ORDER BY name")?;

        let tags = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        Ok(tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> Database {
        let temp_file = NamedTempFile::new().unwrap();
        Database::new(temp_file.path().to_path_buf()).unwrap()
    }

    #[test]
    fn test_create_and_get_note() {
        let db = create_test_db();
        let note = Note::new("Test Note".to_string(), "Test content".to_string());
        let tags = vec!["test".to_string(), "example".to_string()];

        // Create note
        db.create_note(&note, &tags).unwrap();

        // Get all notes
        let notes = db.get_all_notes().unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].note.title, "Test Note");
        assert_eq!(notes[0].note.content, "Test content");
        assert_eq!(notes[0].tags.len(), 2);
        assert!(notes[0].tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_update_note() {
        let db = create_test_db();
        let note = Note::new("Original".to_string(), "Original content".to_string());

        db.create_note(&note, &vec!["tag1".to_string()]).unwrap();

        // Update note
        let mut updated_note = note.clone();
        updated_note.title = "Updated".to_string();
        updated_note.content = "Updated content".to_string();
        updated_note.updated_at = chrono::Utc::now();

        db.update_note(&updated_note, &vec!["tag2".to_string()]).unwrap();

        // Verify update
        let notes = db.get_all_notes().unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].note.title, "Updated");
        assert_eq!(notes[0].note.content, "Updated content");
        assert_eq!(notes[0].tags, vec!["tag2".to_string()]);
    }

    #[test]
    fn test_delete_note() {
        let db = create_test_db();
        let note = Note::new("To Delete".to_string(), "Content".to_string());

        db.create_note(&note, &vec![]).unwrap();
        assert_eq!(db.get_all_notes().unwrap().len(), 1);

        db.delete_note(&note.id).unwrap();
        assert_eq!(db.get_all_notes().unwrap().len(), 0);
    }

    #[test]
    fn test_search_notes() {
        let db = create_test_db();

        let note1 = Note::new("Rust Programming".to_string(), "Learning Rust".to_string());
        let note2 = Note::new("JavaScript Guide".to_string(), "JavaScript basics".to_string());
        let note3 = Note::new("Rust Async".to_string(), "Async programming".to_string());

        db.create_note(&note1, &vec![]).unwrap();
        db.create_note(&note2, &vec![]).unwrap();
        db.create_note(&note3, &vec![]).unwrap();

        // Search for "Rust"
        let results = db.search_notes("Rust").unwrap();
        assert_eq!(results.len(), 2);

        // Search for "JavaScript"
        let results = db.search_notes("JavaScript").unwrap();
        assert_eq!(results.len(), 1);

        // Search for non-existent term
        let results = db.search_notes("Python").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_all_tags() {
        let db = create_test_db();

        let note1 = Note::new("Note 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Note 2".to_string(), "Content 2".to_string());

        db.create_note(&note1, &vec!["rust".to_string(), "programming".to_string()]).unwrap();
        db.create_note(&note2, &vec!["javascript".to_string(), "programming".to_string()]).unwrap();

        let tags = db.get_all_tags().unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags, vec!["javascript", "programming", "rust"]);
    }

    #[test]
    fn test_multiple_notes_with_same_tags() {
        let db = create_test_db();

        let note1 = Note::new("Note 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Note 2".to_string(), "Content 2".to_string());

        db.create_note(&note1, &vec!["shared".to_string()]).unwrap();
        db.create_note(&note2, &vec!["shared".to_string()]).unwrap();

        let tags = db.get_all_tags().unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "shared");
    }
}

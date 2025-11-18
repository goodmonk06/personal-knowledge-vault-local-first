use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(password: &str) -> Result<Self> {
        let db_path = Self::get_db_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                rusqlite::Error::InvalidPath(PathBuf::from(format!(
                    "Failed to create directory: {}",
                    e
                )))
            })?;
        }

        let conn = Connection::open(&db_path)?;

        // In production, use SQLCipher for encryption
        // For now, using regular SQLite with a note about encryption
        // The password parameter is stored for future SQLCipher integration

        let db = Database { conn };
        db.initialize_schema()?;
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("personal-knowledge-vault");
        path.push("vault.db");
        path
    }

    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS note_tags (
                note_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                PRIMARY KEY (note_id, tag_id),
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
                FOREIGN KEY (note_id) REFERENCES tags(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Create FTS table for full-text search
        self.conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
                title, content, content=notes, content_rowid=id
            )",
            [],
        )?;

        // Create triggers to keep FTS table in sync
        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS notes_ai AFTER INSERT ON notes BEGIN
                INSERT INTO notes_fts(rowid, title, content) VALUES (new.id, new.title, new.content);
            END",
            [],
        )?;

        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS notes_ad AFTER DELETE ON notes BEGIN
                DELETE FROM notes_fts WHERE rowid = old.id;
            END",
            [],
        )?;

        self.conn.execute(
            "CREATE TRIGGER IF NOT EXISTS notes_au AFTER UPDATE ON notes BEGIN
                UPDATE notes_fts SET title = new.title, content = new.content WHERE rowid = old.id;
            END",
            [],
        )?;

        Ok(())
    }

    pub fn create_note(&self, title: &str, content: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO notes (title, content, created_at, updated_at) VALUES (?1, ?2, datetime('now'), datetime('now'))",
            params![title, content],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_note(&self, id: i64) -> Result<Note> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes WHERE id = ?1"
        )?;

        stmt.query_row(params![id], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
    }

    pub fn update_note(&self, id: i64, title: &str, content: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = datetime('now') WHERE id = ?3",
            params![title, content, id],
        )?;
        Ok(())
    }

    pub fn delete_note(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_notes(&self) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map([], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        notes.collect()
    }

    pub fn search_notes(&self, query: &str) -> Result<Vec<Note>> {
        let search_query = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT n.id, n.title, n.content, n.created_at, n.updated_at
             FROM notes n
             WHERE n.title LIKE ?1 OR n.content LIKE ?1
             ORDER BY n.updated_at DESC"
        )?;

        let notes = stmt.query_map(params![search_query], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        notes.collect()
    }

    pub fn add_tag(&self, name: &str) -> Result<i64> {
        match self.conn.execute(
            "INSERT INTO tags (name) VALUES (?1)",
            params![name],
        ) {
            Ok(_) => Ok(self.conn.last_insert_rowid()),
            Err(rusqlite::Error::SqliteFailure(err, _)) if err.code == rusqlite::ErrorCode::ConstraintViolation => {
                // Tag already exists, get its ID
                let mut stmt = self.conn.prepare("SELECT id FROM tags WHERE name = ?1")?;
                stmt.query_row(params![name], |row| row.get(0))
            }
            Err(e) => Err(e),
        }
    }

    pub fn add_tag_to_note(&self, note_id: i64, tag_name: &str) -> Result<()> {
        let tag_id = self.add_tag(tag_name)?;
        self.conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag_id],
        )?;
        Ok(())
    }

    pub fn remove_tag_from_note(&self, note_id: i64, tag_name: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = (SELECT id FROM tags WHERE name = ?2)",
            params![note_id, tag_name],
        )?;
        Ok(())
    }

    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM tags ORDER BY name")?;
        let tags = stmt.query_map([], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
            })
        })?;
        tags.collect()
    }

    pub fn get_tags_for_note(&self, note_id: i64) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.name FROM tags t
             INNER JOIN note_tags nt ON t.id = nt.tag_id
             WHERE nt.note_id = ?1
             ORDER BY t.name"
        )?;
        let tags = stmt.query_map(params![note_id], |row| {
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
            })
        })?;
        tags.collect()
    }
}

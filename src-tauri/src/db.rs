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
// Phase 3: Enhanced domain models

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notebook {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteVersion {
    pub id: Option<i64>,
    pub note_id: i64,
    pub title: String,
    pub content: String,
    pub version_number: i32,
    pub created_at: String,
    pub created_by: String,
    pub change_summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteLink {
    pub id: Option<i64>,
    pub source_note_id: i64,
    pub target_note_id: i64,
    pub link_type: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub title_template: String,
    pub content_template: String,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: Option<i64>,
    pub note_id: i64,
    pub filename: String,
    pub filepath: String,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub query_rules: String, // JSON
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultStats {
    pub total_notes: i64,
    pub total_notebooks: i64,
    pub total_tags: i64,
    pub total_versions: i64,
    pub total_attachments: i64,
    pub vault_size_bytes: i64,
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

        // Apply Phase 3 schema migrations
        self.apply_phase3_migrations()?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_db() -> Database {
        // Use in-memory database for tests
        let conn = Connection::open_in_memory().unwrap();
        let db = Database { conn };
        db.initialize_schema().unwrap();
        db
    }

    #[test]
    fn test_create_and_get_note() {
        let db = create_test_db();
        
        let id = db.create_note("Test Note", "Test Content").unwrap();
        assert!(id > 0);

        let note = db.get_note(id).unwrap();
        assert_eq!(note.title, "Test Note");
        assert_eq!(note.content, "Test Content");
        assert_eq!(note.id, Some(id));
    }

    #[test]
    fn test_update_note() {
        let db = create_test_db();
        
        let id = db.create_note("Original", "Original Content").unwrap();
        db.update_note(id, "Updated", "Updated Content").unwrap();

        let note = db.get_note(id).unwrap();
        assert_eq!(note.title, "Updated");
        assert_eq!(note.content, "Updated Content");
    }

    #[test]
    fn test_delete_note() {
        let db = create_test_db();
        
        let id = db.create_note("To Delete", "Content").unwrap();
        db.delete_note(id).unwrap();

        let result = db.get_note(id);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_notes() {
        let db = create_test_db();
        
        db.create_note("Note 1", "Content 1").unwrap();
        db.create_note("Note 2", "Content 2").unwrap();
        db.create_note("Note 3", "Content 3").unwrap();

        let notes = db.list_notes().unwrap();
        assert_eq!(notes.len(), 3);
    }

    #[test]
    fn test_search_notes() {
        let db = create_test_db();
        
        db.create_note("Meeting Notes", "Discussed project timeline").unwrap();
        db.create_note("Shopping List", "Buy groceries").unwrap();
        db.create_note("Project Ideas", "Brainstormed new features").unwrap();

        let results = db.search_notes("project").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_tags() {
        let db = create_test_db();
        
        let note_id = db.create_note("Tagged Note", "Content").unwrap();
        
        db.add_tag_to_note(note_id, "important").unwrap();
        db.add_tag_to_note(note_id, "work").unwrap();

        let tags = db.get_tags_for_note(note_id).unwrap();
        assert_eq!(tags.len(), 2);

        db.remove_tag_from_note(note_id, "work").unwrap();
        let tags = db.get_tags_for_note(note_id).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "important");
    }

    #[test]
    fn test_get_all_tags() {
        let db = create_test_db();
        
        let note1 = db.create_note("Note 1", "Content").unwrap();
        let note2 = db.create_note("Note 2", "Content").unwrap();

        db.add_tag_to_note(note1, "important").unwrap();
        db.add_tag_to_note(note2, "work").unwrap();
        db.add_tag_to_note(note1, "personal").unwrap();

        let tags = db.get_all_tags().unwrap();
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn test_duplicate_tag() {
        let db = create_test_db();
        
        let note_id = db.create_note("Note", "Content").unwrap();
        
        db.add_tag_to_note(note_id, "duplicate").unwrap();
        db.add_tag_to_note(note_id, "duplicate").unwrap();

        let tags = db.get_tags_for_note(note_id).unwrap();
        assert_eq!(tags.len(), 1);
    }
}

    // Phase 3: Apply schema migrations for enhanced features
    fn apply_phase3_migrations(&self) -> Result<()> {
        // Helper to check if column exists
        let column_exists = |table: &str, column: &str| -> bool {
            let query = format!("PRAGMA table_info({})", table);
            if let Ok(mut stmt) = self.conn.prepare(&query) {
                let columns: Vec<String> = stmt
                    .query_map([], |row| row.get::<_, String>(1))
                    .unwrap()
                    .filter_map(|r| r.ok())
                    .collect();
                columns.contains(&column.to_string())
            } else {
                false
            }
        };

        // Create notebooks table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notebooks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                parent_id INTEGER,
                color TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (parent_id) REFERENCES notebooks(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Add new columns to notes table if they don't exist
        if !column_exists("notes", "notebook_id") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN notebook_id INTEGER REFERENCES notebooks(id)", [])?;
        }
        if !column_exists("notes", "status") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN status TEXT DEFAULT 'active'", [])?;
        }
        if !column_exists("notes", "priority") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN priority INTEGER DEFAULT 0", [])?;
        }
        if !column_exists("notes", "is_pinned") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN is_pinned INTEGER DEFAULT 0", [])?;
        }
        if !column_exists("notes", "is_favorite") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN is_favorite INTEGER DEFAULT 0", [])?;
        }
        if !column_exists("notes", "word_count") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN word_count INTEGER DEFAULT 0", [])?;
        }
        if !column_exists("notes", "metadata") {
            self.conn.execute("ALTER TABLE notes ADD COLUMN metadata TEXT", [])?;
        }

        // Note versions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS note_versions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                version_number INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                created_by TEXT DEFAULT 'user',
                change_summary TEXT,
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Note links table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS note_links (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_note_id INTEGER NOT NULL,
                target_note_id INTEGER NOT NULL,
                link_type TEXT DEFAULT 'reference',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (source_note_id) REFERENCES notes(id) ON DELETE CASCADE,
                FOREIGN KEY (target_note_id) REFERENCES notes(id) ON DELETE CASCADE,
                UNIQUE(source_note_id, target_note_id)
            )",
            [],
        )?;

        // Templates table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                title_template TEXT NOT NULL,
                content_template TEXT NOT NULL,
                category TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Attachments table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS attachments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL,
                filename TEXT NOT NULL,
                filepath TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                mime_type TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // Collections table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                query_rules TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Activity log table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS activity_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action_type TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                entity_id INTEGER,
                details TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Preferences table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS preferences (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Create indexes
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_notebook ON notes(notebook_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_status ON notes(status)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_notes_pinned ON notes(is_pinned)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_note_versions_note_id ON note_versions(note_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_note_links_source ON note_links(source_note_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_note_links_target ON note_links(target_note_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_attachments_note ON attachments(note_id)", [])?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_activity_log_entity ON activity_log(entity_type, entity_id)", [])?;

        Ok(())
    }

    // Notebook operations
    pub fn create_notebook(&self, name: &str, description: Option<&str>, parent_id: Option<i64>, color: Option<&str>) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO notebooks (name, description, parent_id, color, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, datetime('now'), datetime('now'))",
            params![name, description, parent_id, color],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_notebook(&self, id: i64) -> Result<Notebook> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, parent_id, color, created_at, updated_at FROM notebooks WHERE id = ?1"
        )?;
        stmt.query_row(params![id], |row| {
            Ok(Notebook {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                color: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
    }

    pub fn list_notebooks(&self) -> Result<Vec<Notebook>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, parent_id, color, created_at, updated_at FROM notebooks ORDER BY name"
        )?;
        let notebooks = stmt.query_map([], |row| {
            Ok(Notebook {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                parent_id: row.get(3)?,
                color: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        notebooks.collect()
    }

    pub fn update_notebook(&self, id: i64, name: &str, description: Option<&str>, color: Option<&str>) -> Result<()> {
        self.conn.execute(
            "UPDATE notebooks SET name = ?1, description = ?2, color = ?3, updated_at = datetime('now') WHERE id = ?4",
            params![name, description, color, id],
        )?;
        Ok(())
    }

    pub fn delete_notebook(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM notebooks WHERE id = ?1", params![id])?;
        Ok(())
    }

    // Get vault statistics
    pub fn get_vault_stats(&self) -> Result<VaultStats> {
        let total_notes: i64 = self.conn.query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))?;
        let total_notebooks: i64 = self.conn.query_row("SELECT COUNT(*) FROM notebooks", [], |row| row.get(0))?;
        let total_tags: i64 = self.conn.query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0))?;
        let total_versions: i64 = self.conn.query_row("SELECT COUNT(*) FROM note_versions", [], |row| row.get(0))?;
        let total_attachments: i64 = self.conn.query_row("SELECT COUNT(*) FROM attachments", [], |row| row.get(0))?;
        
        // Calculate database file size
        let db_path = Self::get_db_path();
        let vault_size_bytes = std::fs::metadata(db_path).map(|m| m.len() as i64).unwrap_or(0);

        Ok(VaultStats {
            total_notes,
            total_notebooks,
            total_tags,
            total_versions,
            total_attachments,
            vault_size_bytes,
        })
    }

    // Create a new version when updating a note
    pub fn create_note_version(&self, note_id: i64, title: &str, content: &str, version_number: i32, change_summary: Option<&str>) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO note_versions (note_id, title, content, version_number, created_at, change_summary) 
             VALUES (?1, ?2, ?3, ?4, datetime('now'), ?5)",
            params![note_id, title, content, version_number, change_summary],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_note_versions(&self, note_id: i64) -> Result<Vec<NoteVersion>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, note_id, title, content, version_number, created_at, created_by, change_summary 
             FROM note_versions WHERE note_id = ?1 ORDER BY version_number DESC"
        )?;
        let versions = stmt.query_map(params![note_id], |row| {
            Ok(NoteVersion {
                id: Some(row.get(0)?),
                note_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                version_number: row.get(4)?,
                created_at: row.get(5)?,
                created_by: row.get(6)?,
                change_summary: row.get(7)?,
            })
        })?;
        versions.collect()
    }

    // Create note link (bidirectional)
    pub fn create_note_link(&self, source_id: i64, target_id: i64, link_type: Option<&str>) -> Result<i64> {
        let link_type = link_type.unwrap_or("reference");
        self.conn.execute(
            "INSERT INTO note_links (source_note_id, target_note_id, link_type, created_at) 
             VALUES (?1, ?2, ?3, datetime('now'))",
            params![source_id, target_id, link_type],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_linked_notes(&self, note_id: i64) -> Result<Vec<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT n.id, n.title, n.content, n.created_at, n.updated_at 
             FROM notes n
             INNER JOIN note_links l ON (l.source_note_id = ?1 AND l.target_note_id = n.id) 
                                     OR (l.target_note_id = ?1 AND l.source_note_id = n.id)
             ORDER BY n.updated_at DESC"
        )?;
        let notes = stmt.query_map(params![note_id], |row| {
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
}

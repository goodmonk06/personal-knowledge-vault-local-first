use crate::migrations;
use crate::models::{
    Note, NoteLink, NoteLinkInfo, Notebook, NotebookWithCount, NoteWithLinks, NoteWithTags, Tag,
    Template,
};
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
        migrations::run_migrations(&conn)?;
        Ok(())
    }

    /// ノートを作成
    pub fn create_note(&self, note: &Note, tag_names: &[String]) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO notes (id, title, content, notebook_id, template_id, is_pinned, color, metadata, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                &note.id,
                &note.title,
                &note.content,
                &note.notebook_id,
                &note.template_id,
                note.is_pinned as i32,
                &note.color,
                &note.metadata,
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
            "UPDATE notes SET title = ?1, content = ?2, notebook_id = ?3, is_pinned = ?4, color = ?5, updated_at = ?6 WHERE id = ?7",
            params![
                &note.title,
                &note.content,
                &note.notebook_id,
                note.is_pinned as i32,
                &note.color,
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
            "SELECT id, title, content, notebook_id, template_id, is_pinned, color, metadata, created_at, updated_at
             FROM notes ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                notebook_id: row.get(3)?,
                template_id: row.get(4)?,
                is_pinned: row.get::<_, i32>(5)? != 0,
                color: row.get(6)?,
                metadata: row.get(7)?,
                created_at: row.get::<_, String>(8)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_, String>(9)?.parse::<DateTime<Utc>>().unwrap(),
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
            "SELECT id, title, content, notebook_id, template_id, is_pinned, color, metadata, created_at, updated_at
             FROM notes WHERE title LIKE ?1 OR content LIKE ?1 ORDER BY updated_at DESC"
        )?;

        let notes = stmt.query_map(params![search_pattern], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                notebook_id: row.get(3)?,
                template_id: row.get(4)?,
                is_pinned: row.get::<_, i32>(5)? != 0,
                color: row.get(6)?,
                metadata: row.get(7)?,
                created_at: row.get::<_, String>(8)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_, String>(9)?.parse::<DateTime<Utc>>().unwrap(),
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

    // ==================== Notebook Operations ====================

    /// ノートブックを作成
    pub fn create_notebook(&self, notebook: &Notebook) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO notebooks (id, name, description, icon, color, parent_id, is_archived, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &notebook.id,
                &notebook.name,
                &notebook.description,
                &notebook.icon,
                &notebook.color,
                &notebook.parent_id,
                notebook.is_archived as i32,
                notebook.created_at.to_rfc3339(),
                notebook.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// ノートブックを更新
    pub fn update_notebook(&self, notebook: &Notebook) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notebooks SET name = ?1, description = ?2, icon = ?3, color = ?4, is_archived = ?5, updated_at = ?6
             WHERE id = ?7",
            params![
                &notebook.name,
                &notebook.description,
                &notebook.icon,
                &notebook.color,
                notebook.is_archived as i32,
                notebook.updated_at.to_rfc3339(),
                &notebook.id,
            ],
        )?;
        Ok(())
    }

    /// ノートブックを削除
    pub fn delete_notebook(&self, notebook_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notebooks WHERE id = ?1", params![notebook_id])?;
        Ok(())
    }

    /// 全ノートブックを取得
    pub fn get_all_notebooks(&self) -> Result<Vec<NotebookWithCount>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT n.id, n.name, n.description, n.icon, n.color, n.parent_id, n.is_archived, n.created_at, n.updated_at,
                    COALESCE(COUNT(notes.id), 0) as note_count
             FROM notebooks n
             LEFT JOIN notes ON notes.notebook_id = n.id
             GROUP BY n.id
             ORDER BY n.name"
        )?;

        let notebooks = stmt.query_map([], |row| {
            Ok(NotebookWithCount {
                notebook: Notebook {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    icon: row.get(3)?,
                    color: row.get(4)?,
                    parent_id: row.get(5)?,
                    is_archived: row.get::<_, i32>(6)? != 0,
                    created_at: row.get::<_, String>(7)?.parse::<DateTime<Utc>>().unwrap(),
                    updated_at: row.get::<_, String>(8)?.parse::<DateTime<Utc>>().unwrap(),
                },
                note_count: row.get(9)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(notebooks)
    }

    // ==================== Template Operations ====================

    /// テンプレートを作成
    pub fn create_template(&self, template: &Template, default_tags: &[String]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO templates (id, name, description, content, icon, is_system, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &template.id,
                &template.name,
                &template.description,
                &template.content,
                &template.icon,
                template.is_system as i32,
                template.created_at.to_rfc3339(),
                template.updated_at.to_rfc3339(),
            ],
        )?;

        // デフォルトタグを保存
        for tag_name in default_tags {
            conn.execute(
                "INSERT INTO template_default_tags (template_id, tag_name) VALUES (?1, ?2)",
                params![&template.id, tag_name],
            )?;
        }

        Ok(())
    }

    /// テンプレートを更新
    pub fn update_template(&self, template: &Template, default_tags: &[String]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE templates SET name = ?1, description = ?2, content = ?3, icon = ?4, updated_at = ?5
             WHERE id = ?6",
            params![
                &template.name,
                &template.description,
                &template.content,
                &template.icon,
                template.updated_at.to_rfc3339(),
                &template.id,
            ],
        )?;

        // デフォルトタグを更新
        conn.execute(
            "DELETE FROM template_default_tags WHERE template_id = ?1",
            params![&template.id],
        )?;

        for tag_name in default_tags {
            conn.execute(
                "INSERT INTO template_default_tags (template_id, tag_name) VALUES (?1, ?2)",
                params![&template.id, tag_name],
            )?;
        }

        Ok(())
    }

    /// テンプレートを削除
    pub fn delete_template(&self, template_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM templates WHERE id = ?1", params![template_id])?;
        Ok(())
    }

    /// 全テンプレートを取得
    pub fn get_all_templates(&self) -> Result<Vec<Template>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, content, icon, is_system, created_at, updated_at
             FROM templates ORDER BY name"
        )?;

        let templates = stmt.query_map([], |row| {
            let template_id: String = row.get(0)?;
            let default_tags = self.get_template_default_tags_internal(&conn, &template_id)?;

            Ok(Template {
                id: template_id,
                name: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                default_tags,
                icon: row.get(4)?,
                is_system: row.get::<_, i32>(5)? != 0,
                created_at: row.get::<_, String>(6)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_, String>(7)?.parse::<DateTime<Utc>>().unwrap(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(templates)
    }

    /// テンプレートのデフォルトタグを取得（内部メソッド）
    fn get_template_default_tags_internal(&self, conn: &Connection, template_id: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT tag_name FROM template_default_tags WHERE template_id = ?1"
        )?;

        let tags = stmt.query_map(params![template_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        Ok(tags)
    }

    // ==================== Note Link Operations ====================

    /// ノートリンクを作成
    pub fn create_note_link(&self, link: &NoteLink) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO note_links (id, source_note_id, target_note_id, link_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &link.id,
                &link.source_note_id,
                &link.target_note_id,
                &link.link_type,
                link.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// ノートリンクを削除
    pub fn delete_note_link(&self, link_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM note_links WHERE id = ?1", params![link_id])?;
        Ok(())
    }

    /// ノートのリンクを取得
    pub fn get_note_links(&self, note_id: &str) -> Result<NoteWithLinks> {
        let conn = self.conn.lock().unwrap();

        // ノート本体を取得
        let note: Note = conn.query_row(
            "SELECT id, title, content, notebook_id, template_id, is_pinned, color, metadata, created_at, updated_at
             FROM notes WHERE id = ?1",
            params![note_id],
            |row| Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                notebook_id: row.get(3)?,
                template_id: row.get(4)?,
                is_pinned: row.get::<_, i32>(5)? != 0,
                color: row.get(6)?,
                metadata: row.get(7)?,
                created_at: row.get::<_, String>(8)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_, String>(9)?.parse::<DateTime<Utc>>().unwrap(),
            })
        )?;

        // タグを取得
        let tags = self.get_note_tags_internal(&conn, note_id)?;

        // 前方リンク（このノートから他のノートへのリンク）
        let mut stmt = conn.prepare(
            "SELECT n.id, n.title, l.link_type
             FROM note_links l
             INNER JOIN notes n ON l.target_note_id = n.id
             WHERE l.source_note_id = ?1"
        )?;

        let forward_links = stmt.query_map(params![note_id], |row| {
            Ok(NoteLinkInfo {
                note_id: row.get(0)?,
                note_title: row.get(1)?,
                link_type: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        // バックリンク（他のノートからこのノートへのリンク）
        let mut stmt = conn.prepare(
            "SELECT n.id, n.title, l.link_type
             FROM note_links l
             INNER JOIN notes n ON l.source_note_id = n.id
             WHERE l.target_note_id = ?1"
        )?;

        let backlinks = stmt.query_map(params![note_id], |row| {
            Ok(NoteLinkInfo {
                note_id: row.get(0)?,
                note_title: row.get(1)?,
                link_type: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(NoteWithLinks {
            note,
            tags,
            forward_links,
            backlinks,
        })
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

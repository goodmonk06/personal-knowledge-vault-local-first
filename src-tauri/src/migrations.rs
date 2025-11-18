use anyhow::Result;
use rusqlite::Connection;

const SCHEMA_VERSION: i32 = 2;

/// マイグレーションを実行
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // バージョンテーブルの作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        )",
        [],
    )?;

    // 現在のバージョンを取得
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // 必要なマイグレーションを実行
    if current_version < 1 {
        migration_v1(conn)?;
        record_migration(conn, 1)?;
    }

    if current_version < 2 {
        migration_v2(conn)?;
        record_migration(conn, 2)?;
    }

    Ok(())
}

/// マイグレーション記録
fn record_migration(conn: &Connection, version: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO schema_version (version, applied_at) VALUES (?1, datetime('now'))",
        [version],
    )?;
    Ok(())
}

/// マイグレーション v1: 基本スキーマ
fn migration_v1(conn: &Connection) -> Result<()> {
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

    // インデックス
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

/// マイグレーション v2: Phase 3 拡張
fn migration_v2(conn: &Connection) -> Result<()> {
    // ノートテーブルに新しいカラムを追加
    let columns_to_add = vec![
        "ALTER TABLE notes ADD COLUMN notebook_id TEXT",
        "ALTER TABLE notes ADD COLUMN template_id TEXT",
        "ALTER TABLE notes ADD COLUMN is_pinned INTEGER DEFAULT 0",
        "ALTER TABLE notes ADD COLUMN color TEXT",
        "ALTER TABLE notes ADD COLUMN metadata TEXT",
    ];

    for sql in columns_to_add {
        // カラムが既に存在する場合はスキップ
        if let Err(_) = conn.execute(sql, []) {
            // エラーは無視（カラムが既に存在する場合）
        }
    }

    // タグテーブルに新しいカラムを追加
    let tag_columns = vec![
        "ALTER TABLE tags ADD COLUMN color TEXT",
        "ALTER TABLE tags ADD COLUMN description TEXT",
        "ALTER TABLE tags ADD COLUMN parent_tag_id TEXT",
    ];

    for sql in tag_columns {
        if let Err(_) = conn.execute(sql, []) {
            // エラーは無視
        }
    }

    // ノートブックテーブル
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notebooks (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            icon TEXT,
            color TEXT,
            parent_id TEXT,
            is_archived INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (parent_id) REFERENCES notebooks(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // テンプレートテーブル
    conn.execute(
        "CREATE TABLE IF NOT EXISTS templates (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            content TEXT NOT NULL,
            icon TEXT,
            is_system INTEGER DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // テンプレートのデフォルトタグ
    conn.execute(
        "CREATE TABLE IF NOT EXISTS template_default_tags (
            template_id TEXT NOT NULL,
            tag_name TEXT NOT NULL,
            PRIMARY KEY (template_id, tag_name),
            FOREIGN KEY (template_id) REFERENCES templates(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // ノート履歴テーブル
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note_history (
            id TEXT PRIMARY KEY,
            note_id TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            changed_at TEXT NOT NULL,
            change_type TEXT NOT NULL,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // ノートリンクテーブル
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note_links (
            id TEXT PRIMARY KEY,
            source_note_id TEXT NOT NULL,
            target_note_id TEXT NOT NULL,
            link_type TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (source_note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (target_note_id) REFERENCES notes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // 設定テーブル
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id TEXT PRIMARY KEY,
            key TEXT UNIQUE NOT NULL,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 新しいインデックス
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_notes_notebook ON notes(notebook_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_notes_pinned ON notes(is_pinned)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_history_note ON note_history(note_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_links_source ON note_links(source_note_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_note_links_target ON note_links(target_note_id)",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_migrations() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // マイグレーションを実行
        run_migrations(&conn).unwrap();

        // バージョンテーブルを確認
        let version: i32 = conn
            .query_row("SELECT MAX(version) FROM schema_version", [], |row| {
                row.get(0)
            })
            .unwrap();

        assert_eq!(version, SCHEMA_VERSION);

        // テーブルの存在を確認
        let table_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN
                ('notes', 'tags', 'notebooks', 'templates', 'note_history', 'note_links', 'settings')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(table_count, 7);
    }

    #[test]
    fn test_idempotent_migrations() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = Connection::open(temp_file.path()).unwrap();

        // 複数回実行しても問題ないことを確認
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();

        let version: i32 = conn
            .query_row("SELECT MAX(version) FROM schema_version", [], |row| {
                row.get(0)
            })
            .unwrap();

        assert_eq!(version, SCHEMA_VERSION);
    }
}

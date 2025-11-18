use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ノートモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub notebook_id: Option<String>,
    pub template_id: Option<String>,
    pub is_pinned: bool,
    pub color: Option<String>,
    pub metadata: Option<String>, // JSON string
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            notebook_id: None,
            template_id: None,
            is_pinned: false,
            color: None,
            metadata: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// タグモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub parent_tag_id: Option<String>,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            color: None,
            description: None,
            parent_tag_id: None,
        }
    }
}

/// ノートブックモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<String>,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Notebook {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            icon: None,
            color: None,
            parent_id: None,
            is_archived: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// テンプレートモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub default_tags: Vec<String>,
    pub icon: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Template {
    pub fn new(name: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            content,
            default_tags: Vec::new(),
            icon: None,
            is_system: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// ノート履歴モデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteHistory {
    pub id: String,
    pub note_id: String,
    pub title: String,
    pub content: String,
    pub changed_at: DateTime<Utc>,
    pub change_type: String, // "created", "updated", "restored"
}

/// ノートリンクモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteLink {
    pub id: String,
    pub source_note_id: String,
    pub target_note_id: String,
    pub link_type: String, // "reference", "related", "parent", "child"
    pub created_at: DateTime<Utc>,
}

impl NoteLink {
    pub fn new(source_note_id: String, target_note_id: String, link_type: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_note_id,
            target_note_id,
            link_type,
            created_at: Utc::now(),
        }
    }
}

/// 設定モデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub id: String,
    pub key: String,
    pub value: String, // JSON string
    pub updated_at: DateTime<Utc>,
}

/// ノート作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub notebook_id: Option<String>,
    pub template_id: Option<String>,
    pub color: Option<String>,
}

/// ノート更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub notebook_id: Option<String>,
    pub is_pinned: Option<bool>,
    pub color: Option<String>,
}

/// ノートブック作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateNotebookRequest {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<String>,
}

/// ノートブック更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateNotebookRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// テンプレート作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub default_tags: Vec<String>,
    pub icon: Option<String>,
}

/// テンプレート更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub default_tags: Vec<String>,
    pub icon: Option<String>,
}

/// ノートリンク作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateNoteLinkRequest {
    pub source_note_id: String,
    pub target_note_id: String,
    pub link_type: String,
}

/// 検索結果
#[derive(Debug, Serialize)]
pub struct NoteWithTags {
    #[serde(flatten)]
    pub note: Note,
    pub tags: Vec<String>,
}

/// ノートブック with カウント
#[derive(Debug, Serialize)]
pub struct NotebookWithCount {
    #[serde(flatten)]
    pub notebook: Notebook,
    pub note_count: i64,
}

/// ノート with リンク
#[derive(Debug, Serialize)]
pub struct NoteWithLinks {
    #[serde(flatten)]
    pub note: Note,
    pub tags: Vec<String>,
    pub backlinks: Vec<NoteLinkInfo>,
    pub forward_links: Vec<NoteLinkInfo>,
}

/// リンク情報
#[derive(Debug, Serialize)]
pub struct NoteLinkInfo {
    pub note_id: String,
    pub note_title: String,
    pub link_type: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ノートモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
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
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }
}

/// ノート作成リクエスト
#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// ノート更新リクエスト
#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// 検索結果
#[derive(Debug, Serialize)]
pub struct NoteWithTags {
    #[serde(flatten)]
    pub note: Note,
    pub tags: Vec<String>,
}

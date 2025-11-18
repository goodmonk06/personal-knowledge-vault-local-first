// ==================== Core Entities ====================

export interface Note {
  id: string;
  title: string;
  content: string;
  notebook_id: string | null;
  template_id: string | null;
  is_pinned: boolean;
  color: string | null;
  metadata: string | null; // JSON string
  created_at: string;
  updated_at: string;
}

export interface Tag {
  id: string;
  name: string;
  color: string | null;
  description: string | null;
  parent_tag_id: string | null;
}

export interface Notebook {
  id: string;
  name: string;
  description: string | null;
  icon: string | null;
  color: string | null;
  parent_id: string | null;
  is_archived: boolean;
  created_at: string;
  updated_at: string;
}

export interface Template {
  id: string;
  name: string;
  description: string | null;
  content: string;
  default_tags: string[];
  icon: string | null;
  is_system: boolean;
  created_at: string;
  updated_at: string;
}

export interface NoteHistory {
  id: string;
  note_id: string;
  title: string;
  content: string;
  changed_at: string;
  change_type: string; // "created" | "updated" | "restored"
}

export interface NoteLink {
  id: string;
  source_note_id: string;
  target_note_id: string;
  link_type: string; // "reference" | "related" | "parent" | "child"
  created_at: string;
}

export interface Settings {
  id: string;
  key: string;
  value: string; // JSON string
  updated_at: string;
}

// ==================== Composite Types ====================

export interface NoteWithTags extends Note {
  tags: string[];
}

export interface NotebookWithCount extends Notebook {
  note_count: number;
}

export interface NoteLinkInfo {
  note_id: string;
  note_title: string;
  link_type: string;
}

export interface NoteWithLinks extends Note {
  tags: string[];
  backlinks: NoteLinkInfo[];
  forward_links: NoteLinkInfo[];
}

// ==================== Request Types ====================

export interface CreateNoteRequest {
  title: string;
  content: string;
  tags: string[];
  notebook_id?: string | null;
  template_id?: string | null;
  color?: string | null;
}

export interface UpdateNoteRequest {
  id: string;
  title: string;
  content: string;
  tags: string[];
  notebook_id?: string | null;
  is_pinned?: boolean;
  color?: string | null;
}

export interface CreateNotebookRequest {
  name: string;
  description?: string | null;
  icon?: string | null;
  color?: string | null;
  parent_id?: string | null;
}

export interface UpdateNotebookRequest {
  id: string;
  name: string;
  description?: string | null;
  icon?: string | null;
  color?: string | null;
}

export interface CreateTemplateRequest {
  name: string;
  description?: string | null;
  content: string;
  default_tags: string[];
  icon?: string | null;
}

export interface UpdateTemplateRequest {
  id: string;
  name: string;
  description?: string | null;
  content: string;
  default_tags: string[];
  icon?: string | null;
}

export interface CreateNoteLinkRequest {
  source_note_id: string;
  target_note_id: string;
  link_type: string;
}

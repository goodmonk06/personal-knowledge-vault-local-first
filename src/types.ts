export interface Note {
  id: string;
  title: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface NoteWithTags extends Note {
  tags: string[];
}

export interface CreateNoteRequest {
  title: string;
  content: string;
  tags: string[];
}

export interface UpdateNoteRequest {
  id: string;
  title: string;
  content: string;
  tags: string[];
}

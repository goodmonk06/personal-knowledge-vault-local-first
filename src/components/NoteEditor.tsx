import { useState, useEffect } from 'react';
import type { Note, Tag } from '../types';
import { api } from '../api';

interface NoteEditorProps {
  note: Note | null;
  onSave: (note: Note) => void;
  onDelete: (noteId: number) => void;
}

export default function NoteEditor({ note, onSave, onDelete }: NoteEditorProps) {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [tags, setTags] = useState<Tag[]>([]);
  const [newTag, setNewTag] = useState('');

  useEffect(() => {
    if (note) {
      setTitle(note.title);
      setContent(note.content);
      loadTags();
    }
  }, [note]);

  const loadTags = async () => {
    try {
      const allTags = await api.getAllTags();
      setTags(allTags);
    } catch (error) {
      console.error('Failed to load tags:', error);
    }
  };

  const handleSave = () => {
    if (note) {
      onSave({ ...note, title, content });
    }
  };

  const handleDelete = () => {
    if (note?.id && confirm('このノートを削除してもよろしいですか?')) {
      onDelete(note.id);
    }
  };

  const handleAddTag = async () => {
    if (newTag.trim() && note?.id) {
      try {
        await api.addTagToNote(note.id, newTag.trim());
        setNewTag('');
        await loadTags();
      } catch (error) {
        console.error('Failed to add tag:', error);
      }
    }
  };

  const handleRemoveTag = async (tagName: string) => {
    if (note?.id) {
      try {
        await api.removeTagFromNote(note.id, tagName);
        await loadTags();
      } catch (error) {
        console.error('Failed to remove tag:', error);
      }
    }
  };

  if (!note) {
    return (
      <div className="main-content">
        <div className="empty-state">
          <h2>ノートが選択されていません</h2>
          <p>左側からノートを選択するか、新しいノートを作成してください</p>
        </div>
      </div>
    );
  }

  return (
    <div className="main-content">
      <div className="editor-header">
        <div></div>
        <div className="editor-actions">
          <button className="btn btn-primary" onClick={handleSave}>
            保存
          </button>
          <button className="btn btn-danger" onClick={handleDelete}>
            削除
          </button>
        </div>
      </div>
      <div className="editor-container">
        <input
          type="text"
          className="note-title-input"
          placeholder="ノートのタイトル"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onBlur={handleSave}
        />
        <textarea
          className="note-content-input"
          placeholder="ノートの内容を入力..."
          value={content}
          onChange={(e) => setContent(e.target.value)}
          onBlur={handleSave}
        />
        <div className="tags-section">
          <h3>タグ</h3>
          <div className="tags-input-container">
            <input
              type="text"
              className="tag-input"
              placeholder="タグを追加..."
              value={newTag}
              onChange={(e) => setNewTag(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleAddTag()}
            />
            <button className="btn btn-secondary" onClick={handleAddTag}>
              追加
            </button>
          </div>
          <div className="tags-list">
            {tags.map((tag) => (
              <div key={tag.id} className="tag">
                {tag.name}
                <span
                  className="tag-remove"
                  onClick={() => handleRemoveTag(tag.name)}
                >
                  ×
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

import type { Note } from '../types';

interface SidebarProps {
  notes: Note[];
  selectedNote: Note | null;
  searchQuery: string;
  onSearch: (query: string) => void;
  onCreateNote: () => void;
  onSelectNote: (note: Note) => void;
  onExport: () => void;
}

export default function Sidebar({
  notes,
  selectedNote,
  searchQuery,
  onSearch,
  onCreateNote,
  onSelectNote,
  onExport,
}: SidebarProps) {
  return (
    <div className="sidebar">
      <div className="sidebar-header">
        <h1>Knowledge Vault</h1>
        <input
          type="text"
          className="search-box"
          placeholder="ノートを検索..."
          value={searchQuery}
          onChange={(e) => onSearch(e.target.value)}
        />
        <button className="new-note-btn" onClick={onCreateNote}>
          + 新しいノート
        </button>
        <button
          className="new-note-btn"
          onClick={onExport}
          style={{ marginTop: '10px', backgroundColor: '#28a745' }}
        >
          アーカイブをエクスポート
        </button>
      </div>
      <div className="notes-list">
        {notes.length === 0 ? (
          <div style={{ padding: '20px', textAlign: 'center', color: '#666' }}>
            {searchQuery ? 'ノートが見つかりません' : 'ノートがありません'}
          </div>
        ) : (
          notes.map((note) => (
            <div
              key={note.id}
              className={`note-item ${
                selectedNote?.id === note.id ? 'active' : ''
              }`}
              onClick={() => onSelectNote(note)}
            >
              <h3>{note.title || '無題'}</h3>
              <p>{note.content.substring(0, 60)}...</p>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

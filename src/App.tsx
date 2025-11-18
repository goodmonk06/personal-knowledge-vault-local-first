import { useState, useEffect } from 'react';
import { api } from './api';
import type { Note } from './types';
import Login from './components/Login';
import NoteEditor from './components/NoteEditor';
import Sidebar from './components/Sidebar';

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [notes, setNotes] = useState<Note[]>([]);
  const [selectedNote, setSelectedNote] = useState<Note | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    if (isAuthenticated) {
      loadNotes();
    }
  }, [isAuthenticated]);

  const loadNotes = async () => {
    try {
      const allNotes = await api.listNotes();
      setNotes(allNotes);
    } catch (error) {
      console.error('Failed to load notes:', error);
      alert('ノートの読み込みに失敗しました');
    }
  };

  const handleLogin = async (password: string) => {
    try {
      await api.initializeDb(password);
      setIsAuthenticated(true);
    } catch (error) {
      console.error('Login failed:', error);
      throw error;
    }
  };

  const handleSearch = async (query: string) => {
    setSearchQuery(query);
    if (query.trim() === '') {
      loadNotes();
    } else {
      try {
        const results = await api.searchNotes(query);
        setNotes(results);
      } catch (error) {
        console.error('Search failed:', error);
      }
    }
  };

  const handleCreateNote = async () => {
    try {
      const id = await api.createNote('新しいノート', '');
      await loadNotes();
      const newNote = await api.getNote(id);
      setSelectedNote(newNote);
    } catch (error) {
      console.error('Failed to create note:', error);
      alert('ノートの作成に失敗しました');
    }
  };

  const handleSelectNote = (note: Note) => {
    setSelectedNote(note);
  };

  const handleSaveNote = async (note: Note) => {
    try {
      if (note.id) {
        await api.updateNote(note.id, note.title, note.content);
        await loadNotes();
      }
    } catch (error) {
      console.error('Failed to save note:', error);
      alert('ノートの保存に失敗しました');
    }
  };

  const handleDeleteNote = async (noteId: number) => {
    try {
      await api.deleteNote(noteId);
      await loadNotes();
      setSelectedNote(null);
    } catch (error) {
      console.error('Failed to delete note:', error);
      alert('ノートの削除に失敗しました');
    }
  };

  const handleExport = async () => {
    const password = prompt('アーカイブを暗号化するパスワードを入力してください：');
    if (!password) return;

    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const defaultPath = `knowledge-vault-backup-${timestamp}.enc`;

    try {
      const result = await api.exportEncryptedArchive(password, defaultPath);
      alert(result);
    } catch (error) {
      console.error('Export failed:', error);
      alert('エクスポートに失敗しました: ' + error);
    }
  };

  if (!isAuthenticated) {
    return <Login onLogin={handleLogin} />;
  }

  return (
    <div className="app">
      <Sidebar
        notes={notes}
        selectedNote={selectedNote}
        searchQuery={searchQuery}
        onSearch={handleSearch}
        onCreateNote={handleCreateNote}
        onSelectNote={handleSelectNote}
        onExport={handleExport}
      />
      <NoteEditor
        note={selectedNote}
        onSave={handleSaveNote}
        onDelete={handleDeleteNote}
      />
    </div>
  );
}

export default App;

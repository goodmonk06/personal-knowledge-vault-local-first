import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { NoteWithTags, CreateNoteRequest, UpdateNoteRequest } from "./types";
import "./App.css";

function App() {
  const [notes, setNotes] = useState<NoteWithTags[]>([]);
  const [selectedNote, setSelectedNote] = useState<NoteWithTags | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");

  // 編集フォームの状態
  const [editTitle, setEditTitle] = useState("");
  const [editContent, setEditContent] = useState("");
  const [editTags, setEditTags] = useState("");

  // 全ノートを読み込む
  const loadNotes = async () => {
    try {
      const result = await invoke<NoteWithTags[]>("get_all_notes");
      setNotes(result);
    } catch (error) {
      console.error("Failed to load notes:", error);
      alert("ノートの読み込みに失敗しました");
    }
  };

  // ノートを検索
  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      loadNotes();
      return;
    }

    try {
      const result = await invoke<NoteWithTags[]>("search_notes", {
        query: searchQuery,
      });
      setNotes(result);
    } catch (error) {
      console.error("Failed to search notes:", error);
      alert("検索に失敗しました");
    }
  };

  // 新規ノートを作成
  const handleCreate = async () => {
    if (!editTitle.trim()) {
      alert("タイトルを入力してください");
      return;
    }

    try {
      const request: CreateNoteRequest = {
        title: editTitle,
        content: editContent,
        tags: editTags.split(",").map((t) => t.trim()).filter((t) => t),
      };

      await invoke("create_note", { request });
      setEditTitle("");
      setEditContent("");
      setEditTags("");
      setIsEditing(false);
      loadNotes();
    } catch (error) {
      console.error("Failed to create note:", error);
      alert("ノートの作成に失敗しました");
    }
  };

  // ノートを更新
  const handleUpdate = async () => {
    if (!selectedNote) return;

    try {
      const request: UpdateNoteRequest = {
        id: selectedNote.id,
        title: editTitle,
        content: editContent,
        tags: editTags.split(",").map((t) => t.trim()).filter((t) => t),
      };

      await invoke("update_note", { request });
      setIsEditing(false);
      loadNotes();
    } catch (error) {
      console.error("Failed to update note:", error);
      alert("ノートの更新に失敗しました");
    }
  };

  // ノートを削除
  const handleDelete = async () => {
    if (!selectedNote || !confirm("本当に削除しますか？")) return;

    try {
      await invoke("delete_note", { noteId: selectedNote.id });
      setSelectedNote(null);
      loadNotes();
    } catch (error) {
      console.error("Failed to delete note:", error);
      alert("ノートの削除に失敗しました");
    }
  };

  // エクスポート
  const handleExport = async () => {
    const password = prompt("エクスポートアーカイブのパスワードを入力してください:");
    if (!password) return;

    try {
      const filePath = await save({
        filters: [{ name: "Encrypted Archive", extensions: ["enc"] }],
      });

      if (!filePath) return;

      await invoke("export_archive", { outputPath: filePath, password });
      alert("エクスポートが完了しました");
    } catch (error) {
      console.error("Failed to export:", error);
      alert("エクスポートに失敗しました");
    }
  };

  // ノートを選択
  const selectNote = (note: NoteWithTags) => {
    setSelectedNote(note);
    setEditTitle(note.title);
    setEditContent(note.content);
    setEditTags(note.tags.join(", "));
    setIsEditing(false);
  };

  // 新規作成モード
  const startCreate = () => {
    setSelectedNote(null);
    setEditTitle("");
    setEditContent("");
    setEditTags("");
    setIsEditing(true);
  };

  useEffect(() => {
    loadNotes();
  }, []);

  return (
    <div className="container">
      <header>
        <h1>Knowledge Vault</h1>
        <div className="header-actions">
          <input
            type="text"
            placeholder="検索..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyPress={(e) => e.key === "Enter" && handleSearch()}
          />
          <button onClick={handleSearch}>検索</button>
          <button onClick={handleExport}>エクスポート</button>
        </div>
      </header>

      <div className="main-content">
        <aside className="sidebar">
          <button onClick={startCreate} className="new-note-btn">
            + 新規ノート
          </button>
          <div className="note-list">
            {notes.map((note) => (
              <div
                key={note.id}
                className={`note-item ${selectedNote?.id === note.id ? "active" : ""}`}
                onClick={() => selectNote(note)}
              >
                <h3>{note.title}</h3>
                <p className="note-preview">
                  {note.content.substring(0, 50)}
                  {note.content.length > 50 ? "..." : ""}
                </p>
                <div className="note-tags">
                  {note.tags.map((tag) => (
                    <span key={tag} className="tag">
                      {tag}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </aside>

        <main className="editor">
          {selectedNote || isEditing ? (
            <>
              <div className="editor-header">
                {isEditing ? (
                  <>
                    {selectedNote ? (
                      <>
                        <button onClick={handleUpdate}>保存</button>
                        <button onClick={() => setIsEditing(false)}>
                          キャンセル
                        </button>
                      </>
                    ) : (
                      <>
                        <button onClick={handleCreate}>作成</button>
                        <button onClick={() => setIsEditing(false)}>
                          キャンセル
                        </button>
                      </>
                    )}
                  </>
                ) : (
                  <>
                    <button onClick={() => setIsEditing(true)}>編集</button>
                    <button onClick={handleDelete}>削除</button>
                  </>
                )}
              </div>

              {isEditing ? (
                <div className="edit-form">
                  <input
                    type="text"
                    placeholder="タイトル"
                    value={editTitle}
                    onChange={(e) => setEditTitle(e.target.value)}
                    className="title-input"
                  />
                  <textarea
                    placeholder="内容"
                    value={editContent}
                    onChange={(e) => setEditContent(e.target.value)}
                    className="content-input"
                  />
                  <input
                    type="text"
                    placeholder="タグ (カンマ区切り)"
                    value={editTags}
                    onChange={(e) => setEditTags(e.target.value)}
                    className="tags-input"
                  />
                </div>
              ) : (
                selectedNote && (
                  <div className="note-view">
                    <h2>{selectedNote.title}</h2>
                    <div className="note-meta">
                      作成: {new Date(selectedNote.created_at).toLocaleString()}
                      <br />
                      更新: {new Date(selectedNote.updated_at).toLocaleString()}
                    </div>
                    <div className="note-tags">
                      {selectedNote.tags.map((tag) => (
                        <span key={tag} className="tag">
                          {tag}
                        </span>
                      ))}
                    </div>
                    <div className="note-content">
                      {selectedNote.content.split("\n").map((line, i) => (
                        <p key={i}>{line}</p>
                      ))}
                    </div>
                  </div>
                )
              )}
            </>
          ) : (
            <div className="empty-state">
              <p>ノートを選択するか、新規作成してください</p>
            </div>
          )}
        </main>
      </div>
    </div>
  );
}

export default App;

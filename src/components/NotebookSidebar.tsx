import { useState, useEffect } from "react";
import { NotebookWithCount, CreateNotebookRequest, UpdateNotebookRequest } from "../types";
import {
  getAllNotebooks,
  createNotebook,
  updateNotebook,
  deleteNotebook,
} from "../api";
import "./NotebookSidebar.css";

interface NotebookSidebarProps {
  selectedNotebookId: string | null;
  onNotebookSelect: (notebookId: string | null) => void;
}

export default function NotebookSidebar({
  selectedNotebookId,
  onNotebookSelect,
}: NotebookSidebarProps) {
  const [notebooks, setNotebooks] = useState<NotebookWithCount[]>([]);
  const [isCreating, setIsCreating] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    icon: "",
    color: "",
  });

  useEffect(() => {
    loadNotebooks();
  }, []);

  const loadNotebooks = async () => {
    try {
      const result = await getAllNotebooks();
      setNotebooks(result);
    } catch (error) {
      console.error("Failed to load notebooks:", error);
      alert("ノートブックの読み込みに失敗しました");
    }
  };

  const handleCreate = async () => {
    if (!formData.name.trim()) {
      alert("ノートブック名を入力してください");
      return;
    }

    try {
      const request: CreateNotebookRequest = {
        name: formData.name,
        description: formData.description || null,
        icon: formData.icon || null,
        color: formData.color || null,
      };
      await createNotebook(request);
      resetForm();
      loadNotebooks();
    } catch (error) {
      console.error("Failed to create notebook:", error);
      alert("ノートブックの作成に失敗しました");
    }
  };

  const handleUpdate = async () => {
    if (!editingId || !formData.name.trim()) return;

    try {
      const request: UpdateNotebookRequest = {
        id: editingId,
        name: formData.name,
        description: formData.description || null,
        icon: formData.icon || null,
        color: formData.color || null,
      };
      await updateNotebook(request);
      resetForm();
      loadNotebooks();
    } catch (error) {
      console.error("Failed to update notebook:", error);
      alert("ノートブックの更新に失敗しました");
    }
  };

  const handleDelete = async (notebookId: string) => {
    if (!confirm("このノートブックを削除しますか？")) return;

    try {
      await deleteNotebook(notebookId);
      if (selectedNotebookId === notebookId) {
        onNotebookSelect(null);
      }
      loadNotebooks();
    } catch (error) {
      console.error("Failed to delete notebook:", error);
      alert("ノートブックの削除に失敗しました");
    }
  };

  const startEdit = (notebook: NotebookWithCount) => {
    setEditingId(notebook.id);
    setFormData({
      name: notebook.name,
      description: notebook.description || "",
      icon: notebook.icon || "",
      color: notebook.color || "",
    });
    setIsCreating(false);
  };

  const resetForm = () => {
    setFormData({ name: "", description: "", icon: "", color: "" });
    setIsCreating(false);
    setEditingId(null);
  };

  return (
    <div className="notebook-sidebar">
      <div className="notebook-header">
        <h3>ノートブック</h3>
        <button
          className="btn-icon"
          onClick={() => {
            resetForm();
            setIsCreating(true);
          }}
          title="新規ノートブック"
        >
          +
        </button>
      </div>

      {(isCreating || editingId) && (
        <div className="notebook-form">
          <input
            type="text"
            placeholder="アイコン (絵文字)"
            value={formData.icon}
            onChange={(e) => setFormData({ ...formData, icon: e.target.value })}
            maxLength={2}
            className="input-icon"
          />
          <input
            type="text"
            placeholder="ノートブック名"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            className="input-name"
          />
          <input
            type="text"
            placeholder="説明"
            value={formData.description}
            onChange={(e) => setFormData({ ...formData, description: e.target.value })}
            className="input-description"
          />
          <input
            type="color"
            value={formData.color || "#6366f1"}
            onChange={(e) => setFormData({ ...formData, color: e.target.value })}
            className="input-color"
          />
          <div className="form-actions">
            {editingId ? (
              <button onClick={handleUpdate}>更新</button>
            ) : (
              <button onClick={handleCreate}>作成</button>
            )}
            <button onClick={resetForm}>キャンセル</button>
          </div>
        </div>
      )}

      <div className="notebook-list">
        <div
          className={`notebook-item ${selectedNotebookId === null ? "active" : ""}`}
          onClick={() => onNotebookSelect(null)}
        >
          <span className="notebook-icon">📝</span>
          <span className="notebook-name">すべてのノート</span>
        </div>

        {notebooks.map((notebook) => (
          <div
            key={notebook.id}
            className={`notebook-item ${selectedNotebookId === notebook.id ? "active" : ""}`}
            onClick={() => onNotebookSelect(notebook.id)}
            style={{ borderLeft: notebook.color ? `3px solid ${notebook.color}` : undefined }}
          >
            <span className="notebook-icon">{notebook.icon || "📁"}</span>
            <div className="notebook-info">
              <span className="notebook-name">{notebook.name}</span>
              <span className="notebook-count">({notebook.note_count})</span>
            </div>
            <div className="notebook-actions">
              <button
                className="btn-icon-small"
                onClick={(e) => {
                  e.stopPropagation();
                  startEdit(notebook);
                }}
                title="編集"
              >
                ✏️
              </button>
              <button
                className="btn-icon-small"
                onClick={(e) => {
                  e.stopPropagation();
                  handleDelete(notebook.id);
                }}
                title="削除"
              >
                🗑️
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

import { useState, useEffect } from "react";
import { Template, CreateTemplateRequest, UpdateTemplateRequest } from "../types";
import {
  getAllTemplates,
  createTemplate,
  updateTemplate,
  deleteTemplate,
} from "../api";
import "./TemplateSelector.css";

interface TemplateSelectorProps {
  isOpen: boolean;
  onClose: () => void;
  onSelect: (template: Template) => void;
  allowManagement?: boolean;
}

export default function TemplateSelector({
  isOpen,
  onClose,
  onSelect,
  allowManagement = false,
}: TemplateSelectorProps) {
  const [templates, setTemplates] = useState<Template[]>([]);
  const [isCreating, setIsCreating] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    content: "",
    icon: "",
    defaultTags: "",
  });

  useEffect(() => {
    if (isOpen) {
      loadTemplates();
    }
  }, [isOpen]);

  const loadTemplates = async () => {
    try {
      const result = await getAllTemplates();
      setTemplates(result);
    } catch (error) {
      console.error("Failed to load templates:", error);
      alert("テンプレートの読み込みに失敗しました");
    }
  };

  const handleCreate = async () => {
    if (!formData.name.trim() || !formData.content.trim()) {
      alert("名前と内容を入力してください");
      return;
    }

    try {
      const request: CreateTemplateRequest = {
        name: formData.name,
        description: formData.description || null,
        content: formData.content,
        default_tags: formData.defaultTags
          .split(",")
          .map((t) => t.trim())
          .filter((t) => t),
        icon: formData.icon || null,
      };
      await createTemplate(request);
      resetForm();
      loadTemplates();
    } catch (error) {
      console.error("Failed to create template:", error);
      alert("テンプレートの作成に失敗しました");
    }
  };

  const handleUpdate = async () => {
    if (!editingId || !formData.name.trim() || !formData.content.trim()) return;

    try {
      const request: UpdateTemplateRequest = {
        id: editingId,
        name: formData.name,
        description: formData.description || null,
        content: formData.content,
        default_tags: formData.defaultTags
          .split(",")
          .map((t) => t.trim())
          .filter((t) => t),
        icon: formData.icon || null,
      };
      await updateTemplate(request);
      resetForm();
      loadTemplates();
    } catch (error) {
      console.error("Failed to update template:", error);
      alert("テンプレートの更新に失敗しました");
    }
  };

  const handleDelete = async (templateId: string) => {
    if (!confirm("このテンプレートを削除しますか？")) return;

    try {
      await deleteTemplate(templateId);
      loadTemplates();
    } catch (error) {
      console.error("Failed to delete template:", error);
      alert("テンプレートの削除に失敗しました");
    }
  };

  const startEdit = (template: Template) => {
    setEditingId(template.id);
    setFormData({
      name: template.name,
      description: template.description || "",
      content: template.content,
      icon: template.icon || "",
      defaultTags: template.default_tags.join(", "),
    });
    setIsCreating(false);
  };

  const resetForm = () => {
    setFormData({
      name: "",
      description: "",
      content: "",
      icon: "",
      defaultTags: "",
    });
    setIsCreating(false);
    setEditingId(null);
  };

  if (!isOpen) return null;

  return (
    <div className="template-modal-overlay" onClick={onClose}>
      <div className="template-modal" onClick={(e) => e.stopPropagation()}>
        <div className="template-modal-header">
          <h2>テンプレート選択</h2>
          {allowManagement && (
            <button
              className="btn-icon"
              onClick={() => {
                resetForm();
                setIsCreating(true);
              }}
              title="新規テンプレート"
            >
              +
            </button>
          )}
          <button className="btn-close" onClick={onClose}>
            ✕
          </button>
        </div>

        {(isCreating || editingId) && allowManagement && (
          <div className="template-form">
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
              placeholder="テンプレート名"
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
            <textarea
              placeholder="テンプレート内容"
              value={formData.content}
              onChange={(e) => setFormData({ ...formData, content: e.target.value })}
              className="input-content"
              rows={6}
            />
            <input
              type="text"
              placeholder="デフォルトタグ (カンマ区切り)"
              value={formData.defaultTags}
              onChange={(e) => setFormData({ ...formData, defaultTags: e.target.value })}
              className="input-tags"
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

        <div className="template-list">
          {templates.length === 0 ? (
            <div className="empty-state">
              <p>テンプレートがありません</p>
            </div>
          ) : (
            templates.map((template) => (
              <div
                key={template.id}
                className="template-item"
                onClick={() => {
                  onSelect(template);
                  onClose();
                }}
              >
                <div className="template-item-header">
                  <span className="template-icon">{template.icon || "📄"}</span>
                  <h3>{template.name}</h3>
                  {template.is_system && (
                    <span className="badge-system">システム</span>
                  )}
                </div>
                {template.description && (
                  <p className="template-description">{template.description}</p>
                )}
                <div className="template-preview">
                  {template.content.substring(0, 100)}
                  {template.content.length > 100 ? "..." : ""}
                </div>
                {template.default_tags.length > 0 && (
                  <div className="template-tags">
                    {template.default_tags.map((tag) => (
                      <span key={tag} className="tag">
                        {tag}
                      </span>
                    ))}
                  </div>
                )}
                {allowManagement && !template.is_system && (
                  <div className="template-actions">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        startEdit(template);
                      }}
                    >
                      編集
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDelete(template.id);
                      }}
                    >
                      削除
                    </button>
                  </div>
                )}
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}

import { useState, useEffect } from "react";
import { NoteWithTags, NoteWithLinks, CreateNoteLinkRequest } from "../types";
import { getNoteLinks, createNoteLink, deleteNoteLink } from "../api";
import "./NoteLinkPanel.css";

interface NoteLinkPanelProps {
  currentNoteId: string | null;
  allNotes: NoteWithTags[];
  onNavigateToNote: (noteId: string) => void;
  onLinksChanged?: () => void;
}

type LinkType = "reference" | "related" | "parent" | "child";

const LINK_TYPES: { value: LinkType; label: string; emoji: string }[] = [
  { value: "reference", label: "参照", emoji: "🔗" },
  { value: "related", label: "関連", emoji: "🔀" },
  { value: "parent", label: "親", emoji: "⬆️" },
  { value: "child", label: "子", emoji: "⬇️" },
];

export default function NoteLinkPanel({
  currentNoteId,
  allNotes,
  onNavigateToNote,
  onLinksChanged,
}: NoteLinkPanelProps) {
  const [noteWithLinks, setNoteWithLinks] = useState<NoteWithLinks | null>(null);
  const [isCreatingLink, setIsCreatingLink] = useState(false);
  const [selectedTargetId, setSelectedTargetId] = useState<string>("");
  const [selectedLinkType, setSelectedLinkType] = useState<LinkType>("reference");

  useEffect(() => {
    if (currentNoteId) {
      loadNoteLinks();
    } else {
      setNoteWithLinks(null);
    }
  }, [currentNoteId]);

  const loadNoteLinks = async () => {
    if (!currentNoteId) return;

    try {
      const result = await getNoteLinks(currentNoteId);
      setNoteWithLinks(result);
    } catch (error) {
      console.error("Failed to load note links:", error);
    }
  };

  const handleCreateLink = async () => {
    if (!currentNoteId || !selectedTargetId) {
      alert("リンク先のノートを選択してください");
      return;
    }

    if (currentNoteId === selectedTargetId) {
      alert("同じノートにはリンクできません");
      return;
    }

    try {
      const request: CreateNoteLinkRequest = {
        source_note_id: currentNoteId,
        target_note_id: selectedTargetId,
        link_type: selectedLinkType,
      };
      await createNoteLink(request);
      setIsCreatingLink(false);
      setSelectedTargetId("");
      loadNoteLinks();
      onLinksChanged?.();
    } catch (error) {
      console.error("Failed to create link:", error);
      alert("リンクの作成に失敗しました");
    }
  };

  const handleDeleteLink = async (linkId: string) => {
    if (!confirm("このリンクを削除しますか？")) return;

    try {
      await deleteNoteLink(linkId);
      loadNoteLinks();
      onLinksChanged?.();
    } catch (error) {
      console.error("Failed to delete link:", error);
      alert("リンクの削除に失敗しました");
    }
  };

  const getAvailableNotes = () => {
    if (!currentNoteId) return [];
    return allNotes.filter((note) => note.id !== currentNoteId);
  };

  const getLinkTypeLabel = (linkType: string) => {
    const type = LINK_TYPES.find((t) => t.value === linkType);
    return type ? `${type.emoji} ${type.label}` : linkType;
  };

  if (!currentNoteId || !noteWithLinks) {
    return (
      <div className="note-link-panel">
        <div className="empty-state">
          <p>ノートを選択してリンクを表示</p>
        </div>
      </div>
    );
  }

  return (
    <div className="note-link-panel">
      <div className="link-panel-header">
        <h3>ノートリンク</h3>
        <button
          className="btn-icon"
          onClick={() => setIsCreatingLink(!isCreatingLink)}
          title="新規リンク"
        >
          {isCreatingLink ? "−" : "+"}
        </button>
      </div>

      {isCreatingLink && (
        <div className="link-form">
          <select
            value={selectedTargetId}
            onChange={(e) => setSelectedTargetId(e.target.value)}
            className="select-note"
          >
            <option value="">リンク先のノートを選択...</option>
            {getAvailableNotes().map((note) => (
              <option key={note.id} value={note.id}>
                {note.title}
              </option>
            ))}
          </select>
          <select
            value={selectedLinkType}
            onChange={(e) => setSelectedLinkType(e.target.value as LinkType)}
            className="select-link-type"
          >
            {LINK_TYPES.map((type) => (
              <option key={type.value} value={type.value}>
                {type.emoji} {type.label}
              </option>
            ))}
          </select>
          <div className="form-actions">
            <button onClick={handleCreateLink}>作成</button>
            <button onClick={() => setIsCreatingLink(false)}>キャンセル</button>
          </div>
        </div>
      )}

      <div className="link-sections">
        {/* Forward Links */}
        <div className="link-section">
          <h4>
            リンク先 <span className="count">({noteWithLinks.forward_links.length})</span>
          </h4>
          {noteWithLinks.forward_links.length === 0 ? (
            <p className="empty-message">リンクがありません</p>
          ) : (
            <div className="link-list">
              {noteWithLinks.forward_links.map((link) => (
                <div
                  key={`${link.note_id}-${link.link_type}`}
                  className="link-item"
                  onClick={() => onNavigateToNote(link.note_id)}
                >
                  <span className="link-type">{getLinkTypeLabel(link.link_type)}</span>
                  <span className="link-title">{link.note_title}</span>
                  <button
                    className="btn-delete"
                    onClick={(e) => {
                      e.stopPropagation();
                      // Find the link ID by matching source and target
                      // Note: We need to track link IDs properly
                      // For now, this is a placeholder
                      alert("リンク削除機能は実装中です");
                    }}
                    title="削除"
                  >
                    ✕
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Backlinks */}
        <div className="link-section">
          <h4>
            被リンク <span className="count">({noteWithLinks.backlinks.length})</span>
          </h4>
          {noteWithLinks.backlinks.length === 0 ? (
            <p className="empty-message">被リンクがありません</p>
          ) : (
            <div className="link-list">
              {noteWithLinks.backlinks.map((link) => (
                <div
                  key={`${link.note_id}-${link.link_type}`}
                  className="link-item"
                  onClick={() => onNavigateToNote(link.note_id)}
                >
                  <span className="link-type">{getLinkTypeLabel(link.link_type)}</span>
                  <span className="link-title">{link.note_title}</span>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      <div className="link-stats">
        <div className="stat-item">
          <span className="stat-label">合計リンク</span>
          <span className="stat-value">
            {noteWithLinks.forward_links.length + noteWithLinks.backlinks.length}
          </span>
        </div>
      </div>
    </div>
  );
}

import { NoteWithTags } from "../types";

/**
 * ノートのフィルタリング
 */
export function filterNotesByTag(notes: NoteWithTags[], tag: string): NoteWithTags[] {
  if (!tag) return notes;
  return notes.filter((note) => note.tags.includes(tag));
}

/**
 * ノートのソート
 */
export function sortNotesByDate(
  notes: NoteWithTags[],
  order: "asc" | "desc" = "desc"
): NoteWithTags[] {
  return [...notes].sort((a, b) => {
    const dateA = new Date(a.updated_at).getTime();
    const dateB = new Date(b.updated_at).getTime();
    return order === "desc" ? dateB - dateA : dateA - dateB;
  });
}

/**
 * タグの抽出と集計
 */
export function extractUniqueTags(notes: NoteWithTags[]): string[] {
  const tagSet = new Set<string>();
  notes.forEach((note) => {
    note.tags.forEach((tag) => tagSet.add(tag));
  });
  return Array.from(tagSet).sort();
}

/**
 * ノートの検証
 */
export function validateNote(title: string, content: string): {
  isValid: boolean;
  errors: string[];
} {
  const errors: string[] = [];

  if (!title.trim()) {
    errors.push("Title is required");
  }

  if (title.length > 200) {
    errors.push("Title must be less than 200 characters");
  }

  if (content.length > 100000) {
    errors.push("Content must be less than 100000 characters");
  }

  return {
    isValid: errors.length === 0,
    errors,
  };
}

/**
 * タグのパース
 */
export function parseTags(tagString: string): string[] {
  return tagString
    .split(",")
    .map((tag) => tag.trim())
    .filter((tag) => tag.length > 0)
    .filter((tag, index, self) => self.indexOf(tag) === index); // Remove duplicates
}

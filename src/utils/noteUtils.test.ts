import { describe, it, expect } from "vitest";
import {
  filterNotesByTag,
  sortNotesByDate,
  extractUniqueTags,
  validateNote,
  parseTags,
} from "./noteUtils";
import { NoteWithTags } from "../types";

const mockNotes: NoteWithTags[] = [
  {
    id: "1",
    title: "First Note",
    content: "Content 1",
    created_at: "2024-01-01T00:00:00Z",
    updated_at: "2024-01-01T00:00:00Z",
    tags: ["work", "important"],
  },
  {
    id: "2",
    title: "Second Note",
    content: "Content 2",
    created_at: "2024-01-02T00:00:00Z",
    updated_at: "2024-01-02T00:00:00Z",
    tags: ["personal"],
  },
  {
    id: "3",
    title: "Third Note",
    content: "Content 3",
    created_at: "2024-01-03T00:00:00Z",
    updated_at: "2024-01-03T00:00:00Z",
    tags: ["work", "urgent"],
  },
];

describe("filterNotesByTag", () => {
  it("should return all notes when tag is empty", () => {
    const result = filterNotesByTag(mockNotes, "");
    expect(result).toEqual(mockNotes);
  });

  it("should filter notes by specific tag", () => {
    const result = filterNotesByTag(mockNotes, "work");
    expect(result).toHaveLength(2);
    expect(result[0].id).toBe("1");
    expect(result[1].id).toBe("3");
  });

  it("should return empty array when no notes match tag", () => {
    const result = filterNotesByTag(mockNotes, "nonexistent");
    expect(result).toHaveLength(0);
  });
});

describe("sortNotesByDate", () => {
  it("should sort notes by date descending by default", () => {
    const result = sortNotesByDate(mockNotes);
    expect(result[0].id).toBe("3");
    expect(result[1].id).toBe("2");
    expect(result[2].id).toBe("1");
  });

  it("should sort notes by date ascending", () => {
    const result = sortNotesByDate(mockNotes, "asc");
    expect(result[0].id).toBe("1");
    expect(result[1].id).toBe("2");
    expect(result[2].id).toBe("3");
  });

  it("should not mutate original array", () => {
    const original = [...mockNotes];
    sortNotesByDate(mockNotes);
    expect(mockNotes).toEqual(original);
  });
});

describe("extractUniqueTags", () => {
  it("should extract all unique tags sorted alphabetically", () => {
    const result = extractUniqueTags(mockNotes);
    expect(result).toEqual(["important", "personal", "urgent", "work"]);
  });

  it("should return empty array for notes with no tags", () => {
    const notesWithoutTags: NoteWithTags[] = [
      {
        id: "1",
        title: "Note",
        content: "Content",
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
        tags: [],
      },
    ];
    const result = extractUniqueTags(notesWithoutTags);
    expect(result).toEqual([]);
  });
});

describe("validateNote", () => {
  it("should validate correct note", () => {
    const result = validateNote("Valid Title", "Valid content");
    expect(result.isValid).toBe(true);
    expect(result.errors).toHaveLength(0);
  });

  it("should reject empty title", () => {
    const result = validateNote("", "Content");
    expect(result.isValid).toBe(false);
    expect(result.errors).toContain("Title is required");
  });

  it("should reject title longer than 200 characters", () => {
    const longTitle = "a".repeat(201);
    const result = validateNote(longTitle, "Content");
    expect(result.isValid).toBe(false);
    expect(result.errors).toContain("Title must be less than 200 characters");
  });

  it("should reject content longer than 100000 characters", () => {
    const longContent = "a".repeat(100001);
    const result = validateNote("Title", longContent);
    expect(result.isValid).toBe(false);
    expect(result.errors).toContain("Content must be less than 100000 characters");
  });
});

describe("parseTags", () => {
  it("should parse comma-separated tags", () => {
    const result = parseTags("work, personal, urgent");
    expect(result).toEqual(["work", "personal", "urgent"]);
  });

  it("should trim whitespace from tags", () => {
    const result = parseTags("  work  , personal,urgent  ");
    expect(result).toEqual(["work", "personal", "urgent"]);
  });

  it("should remove empty tags", () => {
    const result = parseTags("work, , personal, ,urgent");
    expect(result).toEqual(["work", "personal", "urgent"]);
  });

  it("should remove duplicate tags", () => {
    const result = parseTags("work, personal, work, urgent");
    expect(result).toEqual(["work", "personal", "urgent"]);
  });

  it("should handle empty string", () => {
    const result = parseTags("");
    expect(result).toEqual([]);
  });
});

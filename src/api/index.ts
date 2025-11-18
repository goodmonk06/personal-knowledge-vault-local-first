import { invoke } from "@tauri-apps/api/core";
import {
  Note,
  NoteWithTags,
  NoteWithLinks,
  Notebook,
  NotebookWithCount,
  Template,
  NoteLink,
  CreateNoteRequest,
  UpdateNoteRequest,
  CreateNotebookRequest,
  UpdateNotebookRequest,
  CreateTemplateRequest,
  UpdateTemplateRequest,
  CreateNoteLinkRequest,
} from "../types";

// ==================== Note Operations ====================

export async function createNote(request: CreateNoteRequest): Promise<Note> {
  return invoke<Note>("create_note", { request });
}

export async function updateNote(request: UpdateNoteRequest): Promise<void> {
  return invoke<void>("update_note", { request });
}

export async function deleteNote(noteId: string): Promise<void> {
  return invoke<void>("delete_note", { noteId });
}

export async function getAllNotes(): Promise<NoteWithTags[]> {
  return invoke<NoteWithTags[]>("get_all_notes");
}

export async function searchNotes(query: string): Promise<NoteWithTags[]> {
  return invoke<NoteWithTags[]>("search_notes", { query });
}

export async function getAllTags(): Promise<string[]> {
  return invoke<string[]>("get_all_tags");
}

export async function exportArchive(
  outputPath: string,
  password: string
): Promise<void> {
  return invoke<void>("export_archive", { outputPath, password });
}

// ==================== Notebook Operations ====================

export async function createNotebook(
  request: CreateNotebookRequest
): Promise<Notebook> {
  return invoke<Notebook>("create_notebook", { request });
}

export async function updateNotebook(
  request: UpdateNotebookRequest
): Promise<void> {
  return invoke<void>("update_notebook", { request });
}

export async function deleteNotebook(notebookId: string): Promise<void> {
  return invoke<void>("delete_notebook", { notebookId });
}

export async function getAllNotebooks(): Promise<NotebookWithCount[]> {
  return invoke<NotebookWithCount[]>("get_all_notebooks");
}

// ==================== Template Operations ====================

export async function createTemplate(
  request: CreateTemplateRequest
): Promise<Template> {
  return invoke<Template>("create_template", { request });
}

export async function updateTemplate(
  request: UpdateTemplateRequest
): Promise<void> {
  return invoke<void>("update_template", { request });
}

export async function deleteTemplate(templateId: string): Promise<void> {
  return invoke<void>("delete_template", { templateId });
}

export async function getAllTemplates(): Promise<Template[]> {
  return invoke<Template[]>("get_all_templates");
}

// ==================== Note Link Operations ====================

export async function createNoteLink(
  request: CreateNoteLinkRequest
): Promise<NoteLink> {
  return invoke<NoteLink>("create_note_link", { request });
}

export async function deleteNoteLink(linkId: string): Promise<void> {
  return invoke<void>("delete_note_link", { linkId });
}

export async function getNoteLinks(noteId: string): Promise<NoteWithLinks> {
  return invoke<NoteWithLinks>("get_note_links", { noteId });
}

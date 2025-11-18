import { invoke } from '@tauri-apps/api/core';
import type { Note, Tag } from './types';
import {
  CreateNoteSchema,
  UpdateNoteSchema,
  InitializeDbSchema,
  SearchQuerySchema,
  ExportSchema,
} from './schemas';

export const api = {
  async initializeDb(password: string): Promise<string> {
    const validated = InitializeDbSchema.parse({ password });
    return await invoke('initialize_db', { password: validated.password });
  },

  async createNote(title: string, content: string): Promise<number> {
    const validated = CreateNoteSchema.parse({ title, content });
    return await invoke('create_note', {
      title: validated.title,
      content: validated.content,
    });
  },

  async getNote(id: number): Promise<Note> {
    return await invoke('get_note', { id });
  },

  async updateNote(id: number, title: string, content: string): Promise<void> {
    const validated = UpdateNoteSchema.parse({ id, title, content });
    return await invoke('update_note', {
      id: validated.id,
      title: validated.title,
      content: validated.content,
    });
  },

  async deleteNote(id: number): Promise<void> {
    return await invoke('delete_note', { id });
  },

  async listNotes(): Promise<Note[]> {
    return await invoke('list_notes');
  },

  async searchNotes(query: string): Promise<Note[]> {
    const validated = SearchQuerySchema.parse({ query });
    return await invoke('search_notes', { query: validated.query });
  },

  async addTagToNote(noteId: number, tagName: string): Promise<void> {
    return await invoke('add_tag_to_note', { noteId, tagName });
  },

  async removeTagFromNote(noteId: number, tagName: string): Promise<void> {
    return await invoke('remove_tag_from_note', { noteId, tagName });
  },

  async getAllTags(): Promise<Tag[]> {
    return await invoke('get_all_tags');
  },

  async exportEncryptedArchive(password: string, outputPath: string): Promise<string> {
    const validated = ExportSchema.parse({ password, outputPath });
    return await invoke('export_encrypted_archive', {
      password: validated.password,
      outputPath: validated.outputPath,
    });
  },
};

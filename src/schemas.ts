import { z } from 'zod';

// Note schemas
export const NoteSchema = z.object({
  id: z.number().optional(),
  title: z.string().min(1, 'Title is required').max(500, 'Title too long'),
  content: z.string().max(100000, 'Content too long'),
  created_at: z.string(),
  updated_at: z.string(),
});

export const CreateNoteSchema = z.object({
  title: z.string().min(1, 'Title is required').max(500, 'Title too long'),
  content: z.string().max(100000, 'Content too long').default(''),
});

export const UpdateNoteSchema = z.object({
  id: z.number(),
  title: z.string().min(1, 'Title is required').max(500, 'Title too long'),
  content: z.string().max(100000, 'Content too long'),
});

// Tag schemas
export const TagSchema = z.object({
  id: z.number().optional(),
  name: z.string().min(1, 'Tag name is required').max(100, 'Tag name too long'),
});

export const CreateTagSchema = z.object({
  name: z.string().min(1, 'Tag name is required').max(100, 'Tag name too long'),
});

// Search schema
export const SearchQuerySchema = z.object({
  query: z.string().min(1, 'Search query is required').max(500, 'Search query too long'),
});

// Export schema
export const ExportSchema = z.object({
  password: z.string().min(8, 'Password must be at least 8 characters'),
  outputPath: z.string().min(1, 'Output path is required'),
});

// Initialize DB schema
export const InitializeDbSchema = z.object({
  password: z.string().min(8, 'Password must be at least 8 characters'),
});

// Type exports
export type Note = z.infer<typeof NoteSchema>;
export type CreateNoteInput = z.infer<typeof CreateNoteSchema>;
export type UpdateNoteInput = z.infer<typeof UpdateNoteSchema>;
export type Tag = z.infer<typeof TagSchema>;
export type CreateTagInput = z.infer<typeof CreateTagSchema>;
export type SearchQuery = z.infer<typeof SearchQuerySchema>;
export type ExportInput = z.infer<typeof ExportSchema>;
export type InitializeDbInput = z.infer<typeof InitializeDbSchema>;

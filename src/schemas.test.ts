import { describe, it, expect } from 'vitest';
import {
  CreateNoteSchema,
  UpdateNoteSchema,
  InitializeDbSchema,
  SearchQuerySchema,
  ExportSchema,
  CreateTagSchema,
} from './schemas';

describe('Validation Schemas', () => {
  describe('CreateNoteSchema', () => {
    it('should validate valid note creation', () => {
      const validData = {
        title: 'Test Note',
        content: 'This is test content',
      };

      const result = CreateNoteSchema.safeParse(validData);
      expect(result.success).toBe(true);
    });

    it('should reject empty title', () => {
      const invalidData = {
        title: '',
        content: 'Content',
      };

      const result = CreateNoteSchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });

    it('should reject title that is too long', () => {
      const invalidData = {
        title: 'a'.repeat(501),
        content: 'Content',
      };

      const result = CreateNoteSchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });

    it('should accept empty content with default', () => {
      const validData = {
        title: 'Test',
      };

      const result = CreateNoteSchema.parse(validData);
      expect(result.content).toBe('');
    });
  });

  describe('UpdateNoteSchema', () => {
    it('should validate valid note update', () => {
      const validData = {
        id: 1,
        title: 'Updated Note',
        content: 'Updated content',
      };

      const result = UpdateNoteSchema.safeParse(validData);
      expect(result.success).toBe(true);
    });

    it('should reject update without id', () => {
      const invalidData = {
        title: 'Updated Note',
        content: 'Updated content',
      };

      const result = UpdateNoteSchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });
  });

  describe('InitializeDbSchema', () => {
    it('should validate password of at least 8 characters', () => {
      const validData = {
        password: 'password123',
      };

      const result = InitializeDbSchema.safeParse(validData);
      expect(result.success).toBe(true);
    });

    it('should reject short password', () => {
      const invalidData = {
        password: 'short',
      };

      const result = InitializeDbSchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });
  });

  describe('SearchQuerySchema', () => {
    it('should validate search query', () => {
      const validData = {
        query: 'search term',
      };

      const result = SearchQuerySchema.safeParse(validData);
      expect(result.success).toBe(true);
    });

    it('should reject empty query', () => {
      const invalidData = {
        query: '',
      };

      const result = SearchQuerySchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });

    it('should reject query that is too long', () => {
      const invalidData = {
        query: 'a'.repeat(501),
      };

      const result = SearchQuerySchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });
  });

  describe('ExportSchema', () => {
    it('should validate export request', () => {
      const validData = {
        password: 'securepass',
        outputPath: '/path/to/backup.enc',
      };

      const result = ExportSchema.safeParse(validData);
      expect(result.success).toBe(true);
    });

    it('should reject export with short password', () => {
      const invalidData = {
        password: 'short',
        outputPath: '/path/to/backup.enc',
      };

      const result = ExportSchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });
  });

  describe('CreateTagSchema', () => {
    it('should validate tag creation', () => {
      const validData = {
        name: 'important',
      };

      const result = CreateTagSchema.safeParse(validData);
      expect(result.success).toBe(true);
    });

    it('should reject empty tag name', () => {
      const invalidData = {
        name: '',
      };

      const result = CreateTagSchema.safeParse(invalidData);
      expect(result.success).toBe(false);
    });
  });
});

/**
 * Seed script for Knowledge Vault
 *
 * This script creates sample notes to demonstrate the application's functionality.
 * Note: This is for development/demo purposes only.
 */

import { invoke } from "@tauri-apps/api/core";

interface CreateNoteRequest {
  title: string;
  content: string;
  tags: string[];
}

const sampleNotes: CreateNoteRequest[] = [
  {
    title: "Welcome to Knowledge Vault",
    content: `This is your local-first personal knowledge vault!

# Features
- Create and organize notes with tags
- Full-text search across all notes
- Export encrypted backups
- Complete data ownership and privacy

All your data stays on your device. No cloud sync, no tracking, just your knowledge.`,
    tags: ["welcome", "getting-started"],
  },
  {
    title: "Project Ideas",
    content: `Some project ideas for the future:

1. Personal website redesign
   - Modern UI with dark mode
   - Blog section
   - Portfolio showcase

2. Learn Rust programming
   - Work through "The Book"
   - Build CLI tools
   - Contribute to open source

3. Home automation system
   - Temperature monitoring
   - Smart lighting
   - Security cameras`,
    tags: ["projects", "ideas", "todo"],
  },
  {
    title: "Rust Best Practices",
    content: `Key Rust principles to follow:

## Ownership and Borrowing
- Use references when you don't need ownership
- Clone only when necessary
- Understand lifetime annotations

## Error Handling
- Use Result and Option types
- Avoid unwrap() in production code
- Use thiserror or anyhow for error handling

## Performance
- Profile before optimizing
- Use iterators for better performance
- Leverage zero-cost abstractions`,
    tags: ["rust", "programming", "best-practices"],
  },
  {
    title: "Weekly Goals - November 2025",
    content: `Goals for this week:

✓ Set up local development environment
✓ Install Knowledge Vault
□ Organize existing notes
□ Create backup routine
□ Write project documentation

Notes:
- Remember to export encrypted backups weekly
- Store backups on external drive
- Keep tags consistent`,
    tags: ["goals", "weekly", "personal"],
  },
  {
    title: "Book Notes: Atomic Habits",
    content: `Key takeaways from "Atomic Habits" by James Clear:

## The Four Laws of Behavior Change
1. Make it obvious
2. Make it attractive
3. Make it easy
4. Make it satisfying

## Implementation Ideas
- Habit stacking: After [current habit], I will [new habit]
- Environment design: Make good habits visible
- Two-minute rule: Scale habits down to 2 minutes
- Habit tracking: Mark each day you complete the habit

The book emphasizes that small changes compound over time. Focus on systems, not goals.`,
    tags: ["books", "notes", "habits", "personal-development"],
  },
  {
    title: "TypeScript Tips",
    content: `Useful TypeScript patterns:

## Type Guards
\`\`\`typescript
function isString(value: unknown): value is string {
  return typeof value === 'string';
}
\`\`\`

## Utility Types
- Partial<T>: Make all properties optional
- Required<T>: Make all properties required
- Pick<T, K>: Pick specific properties
- Omit<T, K>: Omit specific properties

## Generic Constraints
\`\`\`typescript
function getProperty<T, K extends keyof T>(obj: T, key: K) {
  return obj[key];
}
\`\`\``,
    tags: ["typescript", "programming", "web-development"],
  },
  {
    title: "Security Checklist",
    content: `Security practices to follow:

## Password Management
- Use unique passwords for each service
- Enable 2FA wherever possible
- Use a password manager (KeePassXC, 1Password)
- Never reuse passwords

## Data Backup
- Follow 3-2-1 rule: 3 copies, 2 different media, 1 offsite
- Encrypt sensitive backups
- Test restore procedures regularly
- Keep backup software updated

## Privacy
- Use VPN on public WiFi
- Review app permissions regularly
- Minimize data sharing
- Use privacy-focused alternatives when possible`,
    tags: ["security", "privacy", "best-practices"],
  },
  {
    title: "Healthy Recipes",
    content: `Quick and healthy meal ideas:

## Breakfast
- Overnight oats with berries and nuts
- Greek yogurt with granola and honey
- Avocado toast with poached egg

## Lunch
- Quinoa salad with roasted vegetables
- Grilled chicken wrap with hummus
- Lentil soup with whole grain bread

## Dinner
- Baked salmon with roasted Brussels sprouts
- Stir-fry with tofu and mixed vegetables
- Whole wheat pasta with marinara and vegetables

Meal prep on Sundays for the week ahead!`,
    tags: ["recipes", "health", "food", "personal"],
  },
];

async function seedDatabase() {
  console.log("🌱 Seeding database with sample notes...\n");

  for (let i = 0; i < sampleNotes.length; i++) {
    const note = sampleNotes[i];
    try {
      await invoke("create_note", { request: note });
      console.log(`✓ Created: "${note.title}" [${note.tags.join(", ")}]`);
    } catch (error) {
      console.error(`✗ Failed to create "${note.title}":`, error);
    }
  }

  console.log(`\n✅ Seeding complete! Created ${sampleNotes.length} sample notes.`);
  console.log("\nYou can now:");
  console.log("  - Browse notes in the sidebar");
  console.log("  - Search by keywords");
  console.log("  - Filter by tags");
  console.log("  - Export encrypted backup");
}

// Run if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  seedDatabase().catch(console.error);
}

export { seedDatabase, sampleNotes };

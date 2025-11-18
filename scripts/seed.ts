/**
 * Seed script for Knowledge Vault - Phase 3
 *
 * This script creates sample notebooks, templates, notes, and links
 * to demonstrate the application's full functionality.
 * Note: This is for development/demo purposes only.
 */

import { invoke } from "@tauri-apps/api/core";

interface CreateNotebookRequest {
  name: string;
  description?: string | null;
  icon?: string | null;
  color?: string | null;
  parent_id?: string | null;
}

interface CreateTemplateRequest {
  name: string;
  description?: string | null;
  content: string;
  default_tags: string[];
  icon?: string | null;
}

interface CreateNoteRequest {
  title: string;
  content: string;
  tags: string[];
  notebook_id?: string | null;
  template_id?: string | null;
  color?: string | null;
}

interface CreateNoteLinkRequest {
  source_note_id: string;
  target_note_id: string;
  link_type: string;
}

// Phase 3: Sample notebooks
const sampleNotebooks: CreateNotebookRequest[] = [
  {
    name: "Personal",
    description: "Personal notes, goals, and thoughts",
    icon: "👤",
    color: "#6366f1",
  },
  {
    name: "Work",
    description: "Work-related notes and project documentation",
    icon: "💼",
    color: "#10b981",
  },
  {
    name: "Learning",
    description: "Educational content, courses, and study notes",
    icon: "📚",
    color: "#f59e0b",
  },
  {
    name: "Recipes",
    description: "Food recipes and cooking tips",
    icon: "🍳",
    color: "#ef4444",
  },
];

// Phase 3: Sample templates
const sampleTemplates: CreateTemplateRequest[] = [
  {
    name: "Meeting Notes",
    description: "Template for meeting notes",
    icon: "📝",
    content: `# Meeting Notes

## Date:
## Attendees:
## Agenda:

### Discussion Points:
-

### Action Items:
- [ ]

### Next Steps:
`,
    default_tags: ["meeting", "work"],
  },
  {
    name: "Daily Journal",
    description: "Template for daily journaling",
    icon: "📔",
    content: `# Daily Journal - [Date]

## Morning Reflection
### How I'm feeling:

### Today's intentions:

## Evening Reflection
### Wins of the day:

### Lessons learned:

### Gratitude:
`,
    default_tags: ["journal", "personal"],
  },
  {
    name: "Book Summary",
    description: "Template for book notes and summaries",
    icon: "📖",
    content: `# Book Summary: [Title]

**Author:**
**Genre:**
**Rating:** /5

## Key Takeaways
1.
2.
3.

## Favorite Quotes
-

## My Thoughts
`,
    default_tags: ["books", "notes"],
  },
];

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
  console.log("🌱 Seeding Knowledge Vault database with Phase 3 data...\n");

  const createdNotebooks: any[] = [];
  const createdTemplates: any[] = [];
  const createdNotes: any[] = [];

  // Step 1: Create notebooks
  console.log("📁 Creating notebooks...");
  for (const notebook of sampleNotebooks) {
    try {
      const result = await invoke("create_notebook", { request: notebook });
      createdNotebooks.push(result);
      console.log(`✓ Created notebook: "${notebook.name}" ${notebook.icon}`);
    } catch (error) {
      console.error(`✗ Failed to create notebook "${notebook.name}":`, error);
    }
  }

  // Step 2: Create templates
  console.log("\n📄 Creating templates...");
  for (const template of sampleTemplates) {
    try {
      const result = await invoke("create_template", { request: template });
      createdTemplates.push(result);
      console.log(`✓ Created template: "${template.name}" ${template.icon}`);
    } catch (error) {
      console.error(`✗ Failed to create template "${template.name}":`, error);
    }
  }

  // Step 3: Assign notebooks to existing notes
  if (createdNotebooks.length >= 4) {
    sampleNotes[0].notebook_id = null; // Welcome note stays in root
    sampleNotes[1].notebook_id = createdNotebooks[1].id; // Projects → Work
    sampleNotes[2].notebook_id = createdNotebooks[2].id; // Rust → Learning
    sampleNotes[3].notebook_id = createdNotebooks[0].id; // Goals → Personal
    sampleNotes[4].notebook_id = createdNotebooks[2].id; // Book Notes → Learning
    sampleNotes[5].notebook_id = createdNotebooks[2].id; // TypeScript → Learning
    sampleNotes[6].notebook_id = createdNotebooks[1].id; // Security → Work
    sampleNotes[7].notebook_id = createdNotebooks[3].id; // Recipes → Recipes
  }

  // Step 4: Create notes
  console.log("\n📝 Creating notes...");
  for (const note of sampleNotes) {
    try {
      const result = await invoke("create_note", { request: note });
      createdNotes.push(result);
      console.log(`✓ Created note: "${note.title}" [${note.tags.join(", ")}]`);
    } catch (error) {
      console.error(`✗ Failed to create note "${note.title}":`, error);
    }
  }

  // Step 5: Create some note links for demonstration
  if (createdNotes.length >= 6) {
    console.log("\n🔗 Creating note links...");
    const links: CreateNoteLinkRequest[] = [
      {
        source_note_id: createdNotes[2].id, // Rust Best Practices
        target_note_id: createdNotes[5].id, // TypeScript Tips
        link_type: "related",
      },
      {
        source_note_id: createdNotes[4].id, // Book Notes
        target_note_id: createdNotes[3].id, // Weekly Goals
        link_type: "reference",
      },
    ];

    for (const link of links) {
      try {
        await invoke("create_note_link", { request: link });
        console.log(`✓ Created link: ${link.link_type}`);
      } catch (error) {
        console.error(`✗ Failed to create link:`, error);
      }
    }
  }

  // Summary
  console.log("\n✅ Seeding complete!");
  console.log(`  📁 ${createdNotebooks.length} notebooks`);
  console.log(`  📄 ${createdTemplates.length} templates`);
  console.log(`  📝 ${createdNotes.length} notes`);
  console.log(`  🔗 2 note links`);

  console.log("\n🎉 You can now:");
  console.log("  - Browse notebooks in the left sidebar");
  console.log("  - Filter notes by notebook");
  console.log("  - Create new notes from templates");
  console.log("  - Create links between related notes");
  console.log("  - Search across all content");
  console.log("  - Export encrypted backups");
}

// Run if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  seedDatabase().catch(console.error);
}

export { seedDatabase, sampleNotes };

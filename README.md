# Personal Knowledge Vault - Local First

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.91-orange)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB)](https://tauri.app/)

A privacy-first, local-only personal knowledge management system built with Tauri, React, and SQLite. Store your notes locally with encryption, no cloud required.

## 📋 Overview

Personal Knowledge Vault is a **local-first** desktop application for managing your personal notes and knowledge base. Unlike cloud-based solutions, all your data stays on your device, giving you complete control and privacy.

### ✨ Key Features

- **🔒 Privacy-First**: All data stored locally, never sent to cloud services
- **🔐 Encryption**: AES-256-GCM encrypted archives for secure backups
- **📝 Rich Note Management**: Create, edit, organize notes with tags
- **🔍 Full-Text Search**: Quickly find notes by title or content
- **💾 Portable Backups**: Export encrypted archives to external storage
- **🌙 Modern UI**: Clean, dark-mode interface built with React
- **⚡ Fast & Lightweight**: Built with Tauri for minimal resource usage

### 🎯 Perfect For

- Developers keeping technical notes and code snippets
- Researchers organizing study materials and references
- Anyone wanting complete control over their personal data
- Users preferring local-only storage over cloud services

## 🛠 Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Framework** | Tauri 2.x | Lightweight, secure desktop app framework |
| **Frontend** | React 18 + TypeScript | Modern, type-safe UI |
| **Backend** | Rust | High-performance, safe systems programming |
| **Database** | SQLite | Embedded, zero-config database |
| **Encryption** | AES-256-GCM | Military-grade encryption for exports |
| **Validation** | Zod | Runtime type validation |
| **Testing** | Vitest + Cargo Test | Comprehensive test coverage |
| **Build Tool** | Vite | Fast, modern build tooling |

## 🏗 Domain Model

### Core Entities

```
Note
├── id: number
├── title: string
├── content: string
├── created_at: timestamp
├── updated_at: timestamp
└── tags: Tag[]

Tag
├── id: number
└── name: string

ExportArchive
├── notes: Note[]
├── encryption: AES-256-GCM
└── password_protected: boolean
```

### Key Relationships

- Notes have many Tags (many-to-many)
- Tags can be assigned to multiple Notes
- Export Archives contain snapshots of all Notes at export time

## 🚀 Getting Started

### Prerequisites

- **Node.js** 18+
- **Rust** 1.70+
- **npm** or **pnpm**

#### Linux System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libgtk-3-dev librsvg2-dev libayatana-appindicator3-dev
```

**Fedora:**
```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget \
  file libappindicator-gtk3-devel librsvg2-devel
```

**Arch:**
```bash
sudo pacman -S webkit2gtk base-devel curl wget file openssl gtk3
```

### Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/personal-knowledge-vault-local-first.git
cd personal-knowledge-vault-local-first

# 2. Install dependencies
npm install

# 3. Start development server
npm run dev

# The app will launch automatically!
```

### Available Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server with hot reload |
| `npm run build` | Build production app |
| `npm test` | Run all tests |
| `npm run test:coverage` | Run tests with coverage report |
| `npm run lint` | Lint code |
| `npm run lint:fix` | Fix linting issues |
| `npm run format` | Format code with Prettier |
| `npm run type-check` | TypeScript type checking |
| `npm run seed` | Show seed data information |

### First Launch

1. **Launch the app**: Run `npm run dev`
2. **Set a password**: Enter a password (min. 8 characters) to protect your database
3. **Get started**: The app will seed with example notes automatically

**Demo Credentials** (for testing):
```
Password: demo123456
```

⚠️ **Important**: Store your password securely. Without it, you cannot access your data.

## 📘 Example Flow: Complete Note Lifecycle

This example demonstrates the complete vertical slice from creation to export:

### 1. Create a Note

```typescript
// Via UI: Click "+ 新しいノート" button
// Internally calls:
await api.createNote("My First Note", "This is the content");
// Returns: note_id
```

### 2. Add Tags

```typescript
// Via UI: Add tags in the note editor
await api.addTagToNote(noteId, "important");
await api.addTagToNote(noteId, "work");
```

### 3. Search Notes

```typescript
// Via UI: Type in search box
const results = await api.searchNotes("important");
// Returns: Array of matching notes
```

### 4. Update Note

```typescript
// Via UI: Edit and auto-save
await api.updateNote(noteId, "Updated Title", "New content");
```

### 5. Export Archive

```typescript
// Via UI: Click "アーカイブをエクスポート"
const result = await api.exportEncryptedArchive(
  "secure-password-123",
  "/backups/vault-2025-01-15.enc"
);
// Creates encrypted backup file
```

### End-to-End Flow Diagram

```
┌─────────────┐
│ Create Note │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Add Tags   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│Search & Edit│
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Export    │
│  to USB/HDD │
└─────────────┘
```

### Testing the Flow

```bash
# Run frontend tests
npm test

# Run backend tests
cargo test --manifest-path=src-tauri/Cargo.toml

# All tests should pass ✓
```

## 🔒 Security Considerations

### Current Implementation

- **Export Encryption**: AES-256-GCM with 12-byte nonce
- **Password Hashing**: Simple hash (⚠️ experimental)
- **Database**: SQLite without encryption (future: SQLCipher)

### Production Recommendations

For production use, consider upgrading:

1. **Password Derivation**: Implement Argon2 or PBKDF2
2. **Database Encryption**: Integrate SQLCipher for at-rest encryption
3. **Key Management**: Use OS-level keychain/credential management
4. **Backup Encryption**: Add additional encryption layers

**Note**: This is an experimental project. For sensitive data, always maintain multiple independent backups.

## 🗺 Roadmap & Future Extensions

### Phase 3: Enhanced Features
- [ ] **Markdown Editor**: Rich text editing with live preview
- [ ] **File Attachments**: Embed images, PDFs, and documents
- [ ] **Import Archives**: Restore from encrypted backups
- [ ] **Multi-Vault**: Manage multiple separate knowledge bases

### Phase 4: Advanced Capabilities
- [ ] **SQLCipher Integration**: True database-level encryption
- [ ] **Sync Protocol**: Optional P2P sync between devices
- [ ] **Plugin System**: Extensible architecture for custom features
- [ ] **Mobile Companion**: iOS/Android viewer app

### Phase 5: Ecosystem
- [ ] **CLI Tools**: Command-line utilities for automation
- [ ] **API Server**: Optional local REST API
- [ ] **Browser Extension**: Quick capture from web pages
- [ ] **Desktop Search Integration**: OS-level search providers

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to these goals!

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Development setup guide
- Code style guidelines
- Testing requirements
- Pull request process

Quick start for contributors:

```bash
git clone <your-fork>
npm install
npm test          # Ensure tests pass
npm run lint:fix  # Fix any linting issues
npm run dev       # Start developing!
```

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Rust-powered desktop framework
- [React](https://react.dev/) - UI library
- [Zod](https://zod.dev/) - TypeScript-first validation
- [Vite](https://vitejs.dev/) - Next-generation build tool

## 📞 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/yourusername/personal-knowledge-vault-local-first/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/yourusername/personal-knowledge-vault-local-first/discussions)
- 📖 **Documentation**: See docs in this repository

---

**Disclaimer**: This is an experimental project. Always maintain multiple backups of important data. The developers are not responsible for data loss.

**Privacy Commitment**: This application never transmits your data to external servers. All processing happens locally on your device.

# Contributing to Personal Knowledge Vault

Thank you for your interest in contributing to Personal Knowledge Vault!

## Development Setup

### Prerequisites

- Node.js 18+
- Rust 1.70+
- npm or pnpm

### System Dependencies (Linux)

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libgtk-3-dev librsvg2-dev libayatana-appindicator3-dev
```

**Fedora:**
```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel \
  curl wget file libappindicator-gtk3-devel librsvg2-devel
```

**Arch:**
```bash
sudo pacman -S webkit2gtk base-devel curl wget file openssl gtk3
```

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/personal-knowledge-vault-local-first.git
cd personal-knowledge-vault-local-first
```

2. Install dependencies:
```bash
npm install
```

3. Start development server:
```bash
npm run dev
```

4. Run tests:
```bash
npm test                # Frontend tests
cargo test --manifest-path=src-tauri/Cargo.toml  # Backend tests
```

## Project Structure

```
personal-knowledge-vault-local-first/
├── src/                    # React frontend
│   ├── components/        # UI components
│   ├── schemas.ts         # Zod validation schemas
│   ├── api.ts             # Tauri API wrapper
│   └── App.tsx            # Main app component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── db.rs          # Database layer
│   │   ├── commands.rs    # Tauri commands
│   │   └── crypto.rs      # Encryption utilities
│   └── Cargo.toml         # Rust dependencies
├── scripts/               # Build and seed scripts
└── tests/                 # Test files
```

## Development Workflow

### Running Tests

```bash
# Frontend tests with Vitest
npm test

# Run tests in watch mode
npm test -- --watch

# Run tests with coverage
npm run test:coverage

# Backend Rust tests
cargo test --manifest-path=src-tauri/Cargo.toml
```

### Linting and Formatting

```bash
# Lint code
npm run lint

# Fix linting issues
npm run lint:fix

# Format code with Prettier
npm run format

# Type check
npm run type-check
```

### Building

```bash
# Build frontend
npm run build

# Build Tauri app (includes frontend build)
npm run tauri:build
```

## Code Style

- **TypeScript**: Follow ESLint rules, use functional components with hooks
- **Rust**: Follow `rustfmt` and `clippy` guidelines
- **Commits**: Use conventional commit format: `feat:`, `fix:`, `docs:`, etc.

## Testing Guidelines

- Write tests for all new features
- Maintain or improve code coverage
- Test both happy path and error cases
- Use descriptive test names

### Example Test

```typescript
describe('CreateNoteSchema', () => {
  it('should validate valid note creation', () => {
    const validData = { title: 'Test', content: 'Content' };
    const result = CreateNoteSchema.safeParse(validData);
    expect(result.success).toBe(true);
  });
});
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `npm test`
6. Lint your code: `npm run lint:fix`
7. Commit with conventional commits
8. Push to your fork
9. Open a Pull Request

## Reporting Issues

When reporting issues, please include:

- OS and version
- Node.js and Rust versions
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if applicable

## Feature Requests

We welcome feature requests! Please:

- Check if the feature already exists
- Describe the use case clearly
- Explain why it would benefit users
- Consider implementation complexity

## Questions?

Feel free to open an issue for questions or join discussions!

Thank you for contributing! 🎉

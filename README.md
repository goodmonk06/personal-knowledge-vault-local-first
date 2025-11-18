# Knowledge Vault - ローカルファースト個人ナレッジボルト

プライバシーとセキュリティを重視した、ローカルファーストの個人ナレッジ管理アプリケーションです。

## Overview（概要）

Knowledge Vaultは、完全にローカルで動作するデスクトップアプリケーションです。すべてのデータはあなたのデバイス上のSQLiteデータベースに保存され、外部のクラウドサービスには送信されません。タグベースの整理、全文検索、そして暗号化されたバックアップ機能により、あなたの知識を安全に管理できます。

### 主な特徴

- **完全ローカル保存**: すべてのデータはローカルのSQLiteデータベースに保存
- **暗号化バックアップ**: AES-256-GCMで暗号化されたアーカイブをエクスポート
- **タグ管理**: ノートにタグを付けて簡単に整理
- **全文検索**: タイトルと本文から素早く検索
- **軽量**: Tauriベースで高速起動・低メモリ使用量
- **クロスプラットフォーム**: Windows、macOS、Linux対応
- **プライバシー第一**: データは第三者に共有されません

## Tech Stack（技術スタック）

### Backend
- **Tauri 2.x**: Rustベースのデスクトップアプリフレームワーク
- **SQLite**: 組み込みデータベース（rusqliteクレート）
- **AES-256-GCM**: データ暗号化
- **Argon2**: パスワードベースの鍵導出

### Frontend
- **React 19**: UIフレームワーク
- **TypeScript**: 型安全な開発
- **Vite**: 高速なビルドツール
- **CSS3**: カスタムスタイリング（ダークモード対応）

### Testing
- **Vitest**: フロントエンドのユニットテスト
- **Rust標準テスト**: バックエンドのテスト
- **Testing Library**: Reactコンポーネントテスト

## Domain Model（ドメインモデル）

### 主要エンティティ

1. **Note（ノート）**
   - id: UUID
   - title: タイトル
   - content: 本文
   - created_at: 作成日時
   - updated_at: 更新日時

2. **Tag（タグ）**
   - id: UUID
   - name: タグ名

3. **NoteTag（ノート-タグ関連）**
   - note_id: ノートID
   - tag_id: タグID
   - 多対多の関連を表現

### データフロー

```
┌──────────────┐
│   React UI   │
└──────┬───────┘
       │ Tauri Commands
┌──────▼───────┐
│ Rust Backend │
└──────┬───────┘
       │ rusqlite
┌──────▼───────┐
│    SQLite    │
└──────────────┘
```

## Getting Started

### Requirements（前提条件）

- **Node.js**: 18.x 以上
- **Rust**: 1.70 以上
- **Tauri依存関係**: プラットフォーム固有の要件
  - Linux: `webkit2gtk-4.1`, `libgtk-3-dev`, `libayatana-appindicator3-dev`など
  - macOS: Xcode Command Line Tools
  - Windows: Microsoft C++ Build Tools

詳細は[Tauri Prerequisites](https://tauri.app/start/prerequisites/)を参照してください。

### Setup Steps（セットアップ手順）

1. **リポジトリのクローン**
   ```bash
   git clone https://github.com/yourusername/personal-knowledge-vault-local-first.git
   cd personal-knowledge-vault-local-first
   ```

2. **依存関係のインストール**
   ```bash
   npm install
   ```

3. **環境変数の設定**（オプション）
   ```bash
   cp .env.example .env
   ```

4. **開発サーバーの起動**
   ```bash
   npm run dev
   ```

   アプリケーションが起動し、デスクトップウィンドウが表示されます。

### Available Scripts（利用可能なスクリプト）

```bash
# 開発モードで起動（ホットリロード有効）
npm run dev

# プロダクションビルド
npm run build

# アプリをビルドしてインストーラーを生成
npm run start

# TypeScriptの型チェック
npm run type-check

# リンター実行
npm run lint

# コードフォーマット
npm run format

# テスト実行
npm test

# テストをウォッチモードで実行
npm run test:ui

# データベースにサンプルデータを投入（開発用）
npm run db:seed
```

### Testing（テスト）

**フロントエンドテスト:**
```bash
npm test
```

**バックエンドテスト:**
```bash
cd src-tauri
cargo test
```

**すべてのテストを実行:**
```bash
npm test && cd src-tauri && cargo test
```

## Example Flow（実装済みの垂直スライス）

### 1. ノートの作成 → 一覧表示 → 編集 → 削除

完全に動作するCRUD操作が実装されています：

**Create（作成）:**
1. 「+ 新規ノート」ボタンをクリック
2. タイトル、内容、タグ（カンマ区切り）を入力
3. 「作成」ボタンで保存

**Read（読み取り）:**
- サイドバーにすべてのノートが更新日時順に表示
- ノートをクリックすると詳細表示

**Update（更新）:**
1. ノートを選択
2. 「編集」ボタンをクリック
3. 内容を変更して「保存」

**Delete（削除）:**
1. ノートを選択
2. 「削除」ボタンをクリック
3. 確認ダイアログで承認

### 2. 検索機能

**使い方:**
1. ヘッダーの検索ボックスにキーワードを入力
2. Enterキーまたは「検索」ボタンをクリック
3. タイトルと本文から部分一致で検索結果を表示

**例:**
- "rust" → Rustに関するノートを表示
- 空欄で検索 → すべてのノートを表示

### 3. タグベースの整理

- ノート作成・編集時にタグを追加
- サイドバーで各ノートのタグを確認
- タグは重複せず管理される

### 4. 暗号化エクスポート

**バックアップの作成:**
1. 「エクスポート」ボタンをクリック
2. パスワードを入力（12文字以上推奨）
3. 保存先を選択（.encファイルとして保存）

**バックアップに含まれるもの:**
- SQLiteデータベース全体
- すべてのノートとタグ
- メタデータ（バージョン、エクスポート日時）

**セキュリティ:**
- AES-256-GCM暗号化
- Argon2によるパスワード導出
- パスワードなしでは復号不可

### Demo Data（デモデータ）

初回起動後、サンプルノートを追加できます：

```bash
npm run db:seed
```

8つのサンプルノートが作成され、以下をデモンストレーションします：
- 様々なタグの使い方
- マークダウン形式のコンテンツ
- 検索機能のテスト
- 現実的なユースケース

## Architecture & Project Structure

```
personal-knowledge-vault-local-first/
├── src/                        # フロントエンド (React)
│   ├── App.tsx                # メインアプリケーションコンポーネント
│   ├── App.css                # スタイリング
│   ├── types.ts               # TypeScript型定義
│   ├── utils/                 # ユーティリティ関数
│   │   ├── noteUtils.ts       # ノート関連のヘルパー
│   │   └── noteUtils.test.ts # ユニットテスト
│   └── test/                  # テストセットアップ
│       └── setup.ts
│
├── src-tauri/                  # バックエンド (Rust)
│   ├── src/
│   │   ├── lib.rs             # Tauriコマンド定義
│   │   ├── main.rs            # エントリーポイント
│   │   ├── db.rs              # データベースラッパー
│   │   ├── models.rs          # データモデル
│   │   └── crypto.rs          # 暗号化ユーティリティ
│   ├── Cargo.toml             # Rust依存関係
│   └── tauri.conf.json        # Tauri設定
│
├── scripts/                    # ユーティリティスクリプト
│   └── seed.ts                # データベースシード
│
├── tests/                      # E2Eテスト（将来拡張用）
├── .env.example               # 環境変数テンプレート
├── vitest.config.ts           # Vitest設定
├── .eslintrc.json             # ESLint設定
├── .prettierrc.json           # Prettier設定
└── package.json               # npm依存関係
```

## Use Case: 外部ストレージへのバックアップ

このアプリケーションは「**ローカルファースト**」設計を採用しています。データは外部クラウドサービスに送信されず、完全にあなたのデバイス上に保存されます。

### バックアップワークフロー

1. **日常の使用**
   - すべてのノートはローカルのSQLiteデータベースに保存
   - データベースは`~/.local/share/knowledge-vault/`（Linux）、`~/Library/Application Support/knowledge-vault/`（macOS）、`%APPDATA%/knowledge-vault/`（Windows）に配置
   - インターネット接続は不要

2. **暗号化アーカイブのエクスポート**
   - アプリ内の「エクスポート」ボタンをクリック
   - 強力なパスワードを設定（12文字以上推奨）
   - `.enc`ファイルとして保存

3. **外部ストレージへの保存**
   - エクスポートした`.enc`ファイルを以下に保存：
     - 外付けHDD/SSD
     - USBメモリ
     - NAS（Network Attached Storage）
     - プライベートクラウド（Nextcloud、Syncthing等）
   - ファイルは暗号化されているため、パスワードなしでは読み取り不可

4. **復元**（将来実装予定）
   - バックアップファイルをインポート
   - パスワードを入力して復号化
   - データベースを復元

### なぜローカルファーストなのか？

- **プライバシー**: あなたのデータは第三者に共有されません
- **所有権**: データは完全にあなたのものです
- **オフライン動作**: インターネット接続不要
- **セキュリティ**: 暗号化により物理的な保管も安全
- **ベンダーロックインなし**: データ形式はオープン（SQLite）

### セキュリティのベストプラクティス

- **強力なパスワード**: エクスポート時は12文字以上の複雑なパスワードを使用
- **複数バックアップ**: 3-2-1ルール（3コピー、2種類のメディア、1つはオフサイト）
- **定期的なエクスポート**: 週次または月次でバックアップ
- **物理セキュリティ**: バックアップメディアは安全な場所に保管

## Development Guidelines

### コーディング規約

- **TypeScript**: strict モードを使用
- **React**: 関数コンポーネントとHooksを使用
- **Rust**: clippy の警告に従う
- **命名**: 明確で説明的な名前を使用

### コミット前チェックリスト

```bash
# 型チェック
npm run type-check

# リンター
npm run lint

# テスト
npm test
cd src-tauri && cargo test

# フォーマット
npm run format
```

### テストカバレッジ

現在のテストカバレッジ：
- **フロントエンド**: ユーティリティ関数のユニットテスト
- **バックエンド**: データベース操作、暗号化機能のテスト
- **統合テスト**: Tauri Commands の動作確認

## Future Extensions（今後の拡張案）

### 短期（Phase 3）
- [ ] インポート機能（暗号化アーカイブからの復元）
- [ ] マークダウンプレビュー機能
- [ ] タグによるフィルタリングUI
- [ ] ノートのソート機能（作成日、更新日、タイトル）
- [ ] キーボードショートカット

### 中期
- [ ] 全文検索の強化（FTS5による高速検索）
- [ ] ノート間のリンク機能
- [ ] 添付ファイルのサポート
- [ ] テーマのカスタマイズ
- [ ] エクスポート形式の追加（Markdown、JSON）

### 長期
- [ ] プラグインシステム
- [ ] カスタムビューとフィルタ
- [ ] 暗号化同期（P2P、E2EE）
- [ ] モバイルアプリ版
- [ ] Webクリッパー拡張機能

## Troubleshooting

### アプリが起動しない
- Tauri依存関係がすべてインストールされているか確認
- `npm install` を再実行
- ターミナルのエラーメッセージを確認

### データベースエラー
- アプリデータディレクトリのパーミッションを確認
- データベースファイルが破損している場合は削除して再起動

### ビルドエラー
- Node.js と Rust のバージョンを確認
- `node_modules` と `target` ディレクトリを削除して再インストール

```bash
rm -rf node_modules src-tauri/target
npm install
```

## Contributing

コントリビューションを歓迎します！以下の手順でお願いします：

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. Pull Requestを作成

## License

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## Acknowledgments

- [Tauri](https://tauri.app/) - 軽量なデスクトップアプリフレームワーク
- [React](https://react.dev/) - UIライブラリ
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLiteバインディング
- [aes-gcm](https://github.com/RustCrypto/AEADs) - 暗号化ライブラリ

---

**Note**: このアプリケーションはローカルファーストを重視しています。クラウド同期機能は意図的に含まれていません。データの管理とバックアップはユーザーの責任で行ってください。

**Questions or Issues?** GitHubのIssuesで報告してください。

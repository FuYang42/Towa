# Towa - 開発環境クイックセットアップツール

> 🌏 [中文](README.zh-CN.md) | [English](README.en.md) | **日本語**

Towaは、新しいコンピュータやシステムで開発環境を素早くセットアップするための包括的なツールキットです。特に以下の用途に最適化されています：
- C/C++ 開発環境
- Rust 開発環境
- Cepton センサーデバッグ環境
- **Cepton Unified Firmware (UFB) 開発環境**

## 🎯 3ステップで始める

```bash
# 1️⃣ プロジェクトをクローン
git clone https://github.com/FuYang42/Towa.git
cd Towa

# 2️⃣ インストールスクリプトを実行
./scripts/setup.sh install    # Linux/macOS
# または
.\scripts\setup.bat install    # Windows

# 3️⃣ インストールを確認
./scripts/setup.sh check
```

✅ **これだけ！5分で環境構築完了**

📖 **詳細ガイド:** [スタートガイド →](GETTING_STARTED.md)

## プロジェクト構成

```
Towa/
├── src/
│   ├── c/          # C言語で書かれたシステムツール
│   └── rust/       # Rustで書かれた設定管理ツール
├── scripts/        # 自動インストールスクリプト
├── docs/           # ドキュメント
└── README.md
```

## 機能

### 1. 環境検出
- OSの種類とバージョンを自動検出
- インストール済みの開発ツールと依存関係をチェック
- 環境レポートを生成

### 2. 開発ツールのインストール
- **C/C++ ツールチェーン**: GCC, Clang, CMake, Make
- **Rust ツールチェーン**: rustup, cargo, rustfmt, clippy
- **バージョン管理**: Git
- **エディタ**: VS Code（オプション）

### 3. Cepton SDK 設定
- Cepton SDKの自動ダウンロードとインストール
- 環境変数の設定
- サンプルプログラムのコンパイル
- センサー接続テスト

### 4. Cepton UFB 開発環境
- **UFB依存関係のワンクリックインストール**: CMake, Ninja, Python 3, Rust
- **ARMツールチェーン設定ガイド**: LLVM+Clang Embedded Toolchain
- **Git設定の最適化**: シンボリックリンク、CRLF処理
- **Pre-commitフック**: コード品質チェック
- **詳細なドキュメント**: 完全なUFB環境セットアップガイド

## クイックスタート

### Windows

```bash
# 環境チェックを実行
.\scripts\setup.bat check

# すべてのツールを自動インストール
.\scripts\setup.bat install

# Cepton SDKのみインストール
.\scripts\setup.bat cepton

# UFB開発環境をセットアップ（ファームウェア開発推奨）
towa-cli ufb
```

### Linux/macOS

```bash
# 環境チェックを実行
./scripts/setup.sh check

# すべてのツールを自動インストール
./scripts/setup.sh install

# Cepton SDKのみインストール
./scripts/setup.sh cepton

# UFB開発環境をセットアップ（ファームウェア開発推奨）
towa-cli ufb
```

## UFBファームウェア開発クイックセットアップ

Ceptonファームウェア（Nova、Ultraなど）の開発が必要な場合は、以下の手順に従ってください：

```bash
# 1. 基本ツールをインストール
./scripts/setup.sh install

# 2. UFB環境をセットアップ
cd src/rust
cargo build --release
./target/release/towa-cli ufb

# 3. ARMツールチェーンのインストールプロンプトに従う

# 4. 環境を検証
./target/release/towa-cli check

# 5. 詳細なドキュメントを参照
# docs/CEPTON_UFB_SETUP.md を参照
```

## ツールの説明

### Cツール
- **env_checker**: システム設定を素早くスキャンする環境検出ツール
- **pkg_installer**: 統一されたインストールインターフェースを提供するパッケージマネージャーラッパー

### Rustツール
- **towa-cli**: インタラクティブな設定ウィザードを提供するメインCLIプログラム
- **config-manager**: 設定ファイルマネージャー

## プロジェクトのビルド

### Cツールのビルド

```bash
cd src/c
mkdir build && cd build
cmake ..
make
```

### Rustツールのビルド

```bash
cd src/rust
cargo build --release
```

## サポートされているプラットフォーム

- Windows 10/11
- Ubuntu 20.04+
- macOS 11+

## ドキュメント

- **[クイックスタートガイド](docs/QUICKSTART.md)** - 5分で始める
- **[ビルド手順](docs/BUILDING.md)** - 詳細なコンパイルとインストールガイド
- **[Cepton SDKセットアップ](docs/CEPTON_SETUP.md)** - CeptonセンサーSDK設定
- **[Cepton UFB環境セットアップ](docs/CEPTON_UFB_SETUP.md)** - 完全なファームウェア開発環境ガイド ⭐
- **[プロジェクト技術概要](PROJECT_OVERVIEW.md)** - プロジェクトアーキテクチャと技術詳細

## よく使うコマンド (towa-cli)

Rust CLIをビルドした後、以下のコマンドを使用できます：

```bash
# 環境チェック
towa-cli check

# すべてのツールをインストール
towa-cli install all

# Cツールチェーンのみインストール
towa-cli install c

# Rustツールチェーンのみインストール
towa-cli install rust

# Cepton SDKを設定
towa-cli cepton

# UFB開発環境をセットアップ
towa-cli ufb

# 設定ファイルを初期化
towa-cli init

# バージョンを表示
towa-cli version
```

## 貢献

IssueとPull Requestを歓迎します！

## ライセンス

MIT License

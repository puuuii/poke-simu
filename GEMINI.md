# Gemini コードアシスタントのコンテキスト

## プロジェクト概要

このプロジェクト `poke-simu` は、Rust で書かれたポケモンバトルシミュレーターです。開発の初期段階にあるようです。`data` ディレクトリには、能力、技、アイテム、ポケモンの世界の様々な側面を詳述した JSON ファイルが包括的に含まれています。

このプロジェクトは、標準的な Rust のバイナリアプリケーションとして構成されています。

**本アプリは最新のポケモンスカーレット・バイオレット（第 9 世代）に準拠したバトルのシミュレーション**を目的としており、それ以外の静的データは読み込まず、スカーレット・バイオレット版のデータのみを使用します。

## ビルドと実行

標準的な Rust プロジェクトとして、以下のコマンドを使用する必要があります：

- **ビルド:** `cargo build`
- **テスト:** `cargo test`

## 開発規約

- **まず GEMINI.md を読んでから動くこと**
- **`cargo run` は実行しないこと**
- 開発者からの問いかけへの返答の冒頭に必ず「まず GEMINI.md を読んでから動くこと」と述べること
- このプロジェクトは、標準的な Rust の規約に従います。
- 開発中に規約が発生した場合、本書に随時追記します。
- ディレクトリやファイルを追加・削除・修正した場合は、本書の「ディレクトリ構成」も修正します。
- **モジュール宣言:** Rust 2018 エディション以降の新しい書き方（`mod.rs`を使わない方法）を採用し、ディレクトリ名をモジュール名として扱います。各レイヤーのサブモジュールもディレクトリとして分割し、その中に具体的な実装ファイルを配置します。
- 今後一切の対話は日本語で行います。
- **テスト:** 実装時は極めてありふれた正常系のみの単体テストを設けること。

## ディレクトリ構成

```txt
├── src/
│   ├── main.rs          # アプリケーションのエントリーポイント、依存性の注入(DI)など
│   │
│   ├── domain.rs        # ドメイン層のトップレベルモジュール
│   ├── domain/          # ドメイン層: ビジネスの核となるルールとロジック
│   │   ├── model/       # エンティティ、値オブジェクト (例: Pokemon, Move, Battle)
│   │   │   ├── ability.rs
│   │   │   ├── active_pokemon.rs
│   │   │   ├── common.rs
│   │   │   ├── item.rs
│   │   │   ├── move.rs
│   │   │   ├── pokemon.rs
│   │   │   ├── pokemon_species.rs
│   │   │   ├── poke_type.rs
│   │   │   ├── stats.rs
│   │   │   └── status.rs
│   │   ├── repository/  # データ永続化のインターフェース(トレイト)定義
│   │   │   ├── ability_repository.rs
│   │   │   ├── item_repository.rs
│   │   │   ├── move_repository.rs
│   │   │   ├── pokemon_repository.rs
│   │   │   ├── pokemon_species_repository.rs
│   │   │   └── type_repository.rs
│   │   └── service/     # ドメインサービス（一モデルに載せるには範囲が大きい、状態を持たないロジックを定義）
│   │
│   ├── application.rs   # アプリケーション層のトップレベルモジュール
│   ├── application/     # アプリケーション層: ユースケースを実現
│   │   ├── usecase/     # 具体的なユースケース (例: バトル開始, ターン実行)
│   │   │   └── load_static_data.rs
│   │   └── dto/         # モデルから必要な情報だけを抽出する場合の入れ物
│   │       └── loaded_static_data.rs
│   │
│   ├── infrastructure.rs # インフラストラクチャ層のトップレベルモジュール
│   ├── infrastructure/  # インフラストラクチャ層: 外部システムとの連携
│   │   └── persistence/ # データ永続化の実装 (ファイル、DBなど)
│   │       ├── json_loader.rs
│   │       ├── file_ability_repository.rs
│   │       ├── file_item_repository.rs
│   │       ├── file_move_repository.rs
│   │       ├── file_pokemon_repository.rs
│   │       ├── file_pokemon_species_repository.rs
│   │       └── file_type_repository.rs
│   │
│   ├── interfaces.rs    # インターフェース層のトップレベルモジュール
│   └── interfaces/      # インターフェース層: ユーザーや外部システムとのI/F
│       └── cli/         # CLIの実装
```

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要
Rust/Bevyで開発する「Make 10」ゲーム。4つのランダム数字と四則演算を使って10を作る数学パズルゲーム。TDD（テスト駆動開発）アプローチで実装する。

## 開発コマンド

### ビルドとテスト
```bash
# プロジェクトビルド
cargo build

# テスト実行
cargo test

# 特定のテストのみ実行
cargo test <test_name>

# テストの詳細出力
cargo test -- --nocapture

# ゲーム実行
cargo run
```

### Bevyの最適化設定
- 開発時はdynamic_linkingを使用して高速コンパイル
- 本番ビルド時は`cargo build --release`

## アーキテクチャ

### 主要コンポーネント
1. **Game State Management** - ゲーム状態（Playing/StageClear/GameOver）の管理
2. **Number Generation** - ランダム4桁数字生成と解答可能性チェック
3. **Calculation Engine** - 数式パーサーと検証システム
4. **UI Systems** - Bevyによる数字表示、操作ボタン、ポップアップ

### TDD実装方針
- 各機能はテストファーストで実装
  - 1. テストが通らないことを確認
  - 2. 最小限のコードでテストを通す
  - 3. リファクタリングしてコード品質を向上
- ユニットテスト：ゲームロジック、数字生成、計算エンジン
- 統合テスト：UI相互作用、完全なゲームフロー

### ファイル構成（予定）
```
src/
├── main.rs           # Bevyアプリケーションエントリーポイント
├── game/
│   ├── mod.rs        # ゲーム関連モジュール
│   ├── state.rs      # ゲーム状態管理
│   ├── numbers.rs    # 数字生成ロジック
│   └── calculator.rs # 計算エンジン
├── ui/
│   ├── mod.rs        # UI関連モジュール
│   ├── components.rs # UIコンポーネント
│   └── systems.rs    # UIシステム
└── tests/
    ├── game_logic.rs # ゲームロジックテスト
    └── integration.rs # 統合テスト
```

## 言語とコミュニケーション
- このプロジェクトは日本語でのコミュニケーションを前提とする
- コード内コメントは日本語で記述
- 変数名や関数名は英語、但しドキュメントや説明は日本語
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Rust フルスタック Cloudflare Workers SPA。Backend (Axum) と Frontend (Leptos CSR) を共に Rust → WebAssembly でコンパイルし、Cloudflare 上で動作させる。

## 開発コマンド

```bash
# Nix dev shell に入る（必須。全ツールはここで提供される）
nix develop

# 全サービス一括起動（frontend watch + wrangler dev）
process-compose up

# 個別起動
trunk watch --dist dist          # frontend（frontend/ ディレクトリで実行）
wrangler dev                     # worker + アセット配信

# ビルド
worker-build --release worker    # worker の WASM ビルド → worker/build/
trunk build --release            # frontend の WASM ビルド → frontend/dist/

# チェック・リント
cargo check --workspace --target wasm32-unknown-unknown
cargo clippy --workspace --target wasm32-unknown-unknown
```

## アーキテクチャ

- **Cargo workspace**: `worker/`（backend）と `frontend/`（frontend）の2クレート
- **Worker (`worker/src/lib.rs`)**: `#[event(fetch)]` エントリポイント → Axum Router でルーティング → Worker Response に変換。`/api/*` パスを処理
- **Frontend (`frontend/src/main.rs`)**: Leptos CSR。`trunk` で WASM + JS + HTML にバンドルされ `frontend/dist/` に出力
- **ルーティング分離**: `wrangler.jsonc` の `run_worker_first: ["/api/*"]` で API リクエストのみ Worker に振り分け、それ以外は `frontend/dist/` の静的アセットを SPA モードで配信
- **ビルドターゲット**: 両クレートとも `wasm32-unknown-unknown`。release プロファイルは `opt-level = "s"` + LTO + strip でサイズ最適化

## 開発ツールチェーン

すべて `flake.nix` で宣言的に管理:
- **fenix**: Rust stable + `wasm32-unknown-unknown` ターゲット
- **worker-build**: Worker の Rust → WASM コンパイル
- **trunk**: Frontend の WASM バンドル
- **wrangler**: Cloudflare Workers CLI
- **process-compose**: 開発プロセスオーケストレーション

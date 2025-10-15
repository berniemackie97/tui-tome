# Tome — TUI Reader/Editor (Rust)

## Problem
A terminal-first e-book reader/editor with **highlights, underlines, bookmarks, notes, and tabs** that never mutates the source file. Support **.txt** and **.md** first; keep adapters extensible (PDF later).

## Architecture (short)
- **crates/app**: TUI shell (ratatui + crossterm), screens, keymap.
- **crates/core**: domain types (DocumentId, Range, Anchor) and algorithms.
- **crates/storage** (later): SQLite for annotations/search.
- **crates/adapters** (later): `txt`, `md`, `pdf` turn bytes → virtual lines.

Anchors store *context* (`before/target/after`) so selections survive edits. Adapters only render; annotations persist in SQLite keyed by `DocumentId`.

## Quickstart
```bash
cargo run -p tui-tome   # launch TUI (q/Esc to quit)
cargo test --workspace  # run core tests
cargo clippy --workspace --all-targets -- -D warnings
Milestones

Workspace + CI ✅

Domain + anchors ✅

Adapters: txt/md → open/scroll/status

Annotations UI + SQLite persistence

Search + export (JSON)

PDF adapter (text extraction), perf polish

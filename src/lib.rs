//! cp-library: 競技プログラミング用 Rust ライブラリ（cargo で常時コンパイル検査）。
//!
//! 開発時は `use cplib::ds::segtree::*;` のように参照し、提出時に
//! `tools/expand.py main.rs > submit.rs` で依存ごと 1 ファイルへ展開する。
//! 方針は AGENTS.md を参照。
pub mod math;
pub mod ds;
pub mod graph;
pub mod string;
pub mod geometry;
pub mod algo;
pub mod misc;
pub mod dp;

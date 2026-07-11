//! cp-library: 競技プログラミング用 Rust ライブラリ（cargo で常時コンパイル検査）。
//!
//! 使い方（開発時）: 解答 crate から `use cplib::ds::segtree::*;` のように参照。
//! 提出時: `tools/expand.py main.rs > submit.rs` で依存ごと 1 ファイルに展開。
pub mod math;
pub mod ds;

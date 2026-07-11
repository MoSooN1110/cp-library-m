# cp-library

競技プログラミング用の Rust ライブラリ集（コピペ利用前提）。

- 大半は Zed スニペット（`lib_*` / `mlib_*`）から抽出したものです。テンプレ側の
  `MOD` / `read_vec` などの基本定義に依存するファイルがあります。
- **`modint.rs`** は MOD 内蔵・組合せ(`Comb`: nCr/nPr/nHr)内包で**単体動作**（検証済み）。
- 一覧は [`INDEX.md`](./INDEX.md) を参照。

## ディレクトリ

```
.
├── INDEX.md              カテゴリ別索引
├── modint.rs             ★正準・単体動作（MOD内蔵＋組合せ）
├── *.rs                  抽出ライブラリ（ds / graph / math / string / geometry / dp …）
├── _archive_oldstyle/    旧 CamelCase 実装（SEG / UFT / LCA など）
└── _archive_modint/      modint.rs に統合済みの旧 modint / 組合せ
```

## ライセンス

個人用ライブラリ。自由に利用可。

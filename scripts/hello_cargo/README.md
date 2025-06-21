# Hello, world!
1. プロジェクトディレクトリの作成
```
cargo new hello_cargo
```
2. ビルドと実行（`cd hello_cargo`後）
```
cargo run
```
- `cargo build`でビルドだけを行うことも可能
- `cargo check`で実行ファイルのビルドは行わずにコンパイル可能かチェック
- `cargo build --release`でリリース用の最適化されたコンパイルが実行可能（コンパイル所要時間↑実行速度↑）

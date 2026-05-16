# 🦀Rust 50 Steps🦀

50のミニアプリを作りながらRustを学びます。

## 進め方

1. プロジェクトのフォルダを開く
    `level1/01_fizzbuzz` → `level1/02_guessing_game` → ... → `level2/11_cli_todo` → ...
2. READMEを開く
    そのプロジェクトで使うキーワードやメソッドなどが書かれています。そのプロジェクトの要件定義も書かれています。
3. コードを書く
    要件定義に基づき、`src/main.rs`に実装していきます。
4. 実行
   `level1/01_fizzbuzz`であれば、`cargo run -p fizzbuzz`のようにして実行します。
5. チェックを付ける
    `PROGRESS.md`に進捗をチェックします。該当する箇所の`- [ ]`を`- [x]`とします。

## よく使う Cargo コマンド集

- `cargo check`: 実行せずコンパイルに通るかのみ高速チェックする
- `cargo fmt`: コードを標準的なスタイルに自動整形する
- `cargo clippy`: 「もっとRustらしく綺麗に書けるよ」とアドバイスをくれる

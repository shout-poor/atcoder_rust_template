これは何
====

Rust で AtCoder に参加する際のテンプレートプロジェクトです。個人用です。

手順
====

- [oj](https://github.com/online-judge-tools/oj) をインストールしておく
- `cargo generate --git https://github.com/shout-poor/atcoder_rust_template --branch main` で初期プロジェクトを構成する
- `oj d (AtCoder の問題のURL)` でテストケースをダウンロードする
- コードを書く
- `oj test -c "cargo run"` でテストする

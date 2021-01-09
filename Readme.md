# Hello, Wasmer

Wasmer という Web Assembly の処理系が 1.0.0 リリースされた。
ちょっと触ってみる。

## Wasmer を始める

### Wasmer のインストール

[参照: Wasmer の GitHub](https://github.com/wasmerio/wasmer#quickstart)

```sh
$ curl https://get.wasmer.io -sSfL | sh
```

`~/.bashrc`が更新されるので、現在のシェルに適用

```sh
$ source .bashrc
$ wasmer --version
wasmer 1.0.0
```

次に Rust をインストールする。

### Rust のインストール

[参照: TRPL 日本語版](https://doc.rust-jp.rs/book-ja/ch01-01-installation.html)

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source ~/.cargo/env
$ cargo --version
cargo 1.49.0 (d00d64df9 2020-12-05)
```

### Rust のプロジェクトの作成

[参照: TRPL 日本語版](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)

```sh
$ cargo new hello_wasmer_by_rust --bin
$ cd hello_wasmer_by_rust
```

### Wasmer の API のインストール

[参照: Wasmer 公式](https://docs.wasmer.io/integrations/rust/setup)

Cargo.toml を編集。

```toml
[dependencies]
# The Wasmer API
wasmer = "1.0"
```

### ソースコード

[参照: zenn/Wasmer で遊ぶ](https://zenn.dev/helloyuki/scraps/7c972a48d8d600)

src/main.rs に、参考 URL のソースをコピペ。

### 実行

```sh
$ cargo build
$ cargo run
Results of `add_one`: 2
```

### TODO

まだ Wasm も Wasmer も（Rust も）よくわかっていない。

# wio-examples-template

Wio Terminal でデバイスを試すための組込み Rust プロジェクトです。

## 事前準備

#### 1. hf2-cli / cargo-hf2

v0.3.1 以上

##### linux

libusb が必要です。debian 系であれば次のコマンドでインストールします。

```
$ sudo apt install libusb-1.0-0-dev
```

sudo なしで実行する場合には、udev のルールを設定します。`/etc/udev/rules.d/99-seeed-boards.rules` を次の内容で作成します。

```
ATTRS{idVendor}=="2886", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="2886", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="2886", MODE="0666"
```

udev ルールをリロードします。

```
$ sudo udevadm control --reload-rules
```

##### macOS

##### install

```
$ cargo install cargo-hf2
$ cargo install hf2-cli
```

[`hf2-rs`]: https://github.com/jacobrosenthal/hf2-rs/

#### 2. cargo-generate

```
$ cargo install cargo-generate
```

[`cargo-generate`]: https://crates.io/crates/cargo-generate

## ビルド & 実行

cargo-generate でプロジェクトテンプレートを初期化します。

```
$ cargo generate \
    --git https://github.com/tomoyuki-nakabayashi/wio-examples-template.git \
    --name wio_examples
```

examples ディレクトリ下にあるソースコードの TODO 部分を書籍を参考にしながら実装します。

Wio Terminal をブートローダーモードに切り替えたあと、次のコマンドを実行します。

```
$ cargo hf2 --example <サンプル名>
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

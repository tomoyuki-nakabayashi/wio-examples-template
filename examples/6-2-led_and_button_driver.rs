//! 6-2 LEDとボタン/GPIOのサンプルコードです。
//! ボタン1 (一番右のボタン) を押している間、ユーザーLEDが点灯します。
//! LEDドライバとボタンドライバを導入したバージョンです。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-2-led_and_button_driver
//! ```

#![no_std]
#![no_main]
#![allow(dead_code)] // 使用しないメソッドでコンパイラが警告を出さないようにする

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::*; // GPIOの構造体やトレイトをインポートする
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut pins = wio::Pins::new(peripherals.PORT);

    // TODO: ボタン1を押している間、LEDが点灯するコードを書く

    loop {}
}

// Wio Terminalのボタン1ドライバ
// TODO: Button1 を実装する

// Wio TerminalのユーザーLEDドライバ
// TODO: Led を実装する

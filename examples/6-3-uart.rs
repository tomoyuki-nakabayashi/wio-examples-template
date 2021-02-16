//! 8-3 シリアル入出力/UARTのサンプルコードです。
//! ホストPCのシリアルターミナルに
//! ```text
//! hello world
//! this is UART example!
//! ```
//! と出力します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example serial
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use wio::hal::clock::GenericClockController;
use wio::pac::Peripherals;
use wio::prelude::*;
use wio::{entry, Pins, Sets};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // TODO: クロックを初期化します

    // TODO: UARTドライバオブジェクトを初期化します

    // TODO: 「hello world」と出力します

    // TODO: 「this is UART example!」と出力します

    loop {}
}

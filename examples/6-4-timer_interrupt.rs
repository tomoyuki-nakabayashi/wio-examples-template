//! 8-4 タイマ/割り込みのサンプルコードです。
//! 割り込みでLチカしながら、ホストPCのシリアルターミナルに入力した内容をそのまま出力します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example timer_interrupt
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use cortex_m::peripheral::NVIC;
use wio::hal::clock::GenericClockController;
use wio::hal::hal::serial::*;
use wio::hal::timer::TimerCounter;
use wio::pac::{interrupt, Peripherals, TC3};
use wio::prelude::*;
use wio::{entry, Pins, Sets};
use wio_examples::Led;

// TODO: main() 関数と割り込みハンドラとで共有するリソースです


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化します
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    // UARTドライバオブジェクトを初期化します
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // TODO: 2 MHz のクロックを取得します。

    // TODO: TC3 へのクロックを 2 MHz にします

    // TODO: TC3 ドライバオブジェクトを初期化します

    // TODO: 割り込みコントローラで TC3 の割り込み通知を有効化します

    // TODO: 1 秒のカウントを開始し、TC3 が割り込みを発生するようにします

    // TODO: 割り込みハンドラと共有するリソースを格納します

    loop {
        // TODO: シリアルターミナルにechoし続けます

    }
}

// TODO: TC3 の割り込みハンドラを実装します

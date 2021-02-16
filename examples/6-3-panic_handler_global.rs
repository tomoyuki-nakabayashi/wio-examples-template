//! 8-3 シリアル入出力/UARTのサンプルコードです。
//! グローバル変数を使ったパニックハンドラ実装です。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example global_panic_handler
//! ```

#![no_std]
#![no_main]

use wio_terminal as wio;

use core::fmt::Write;
use core::panic::PanicInfo;
use wio::hal::clock::GenericClockController;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::pac::Peripherals;
use wio::prelude::*;
use wio::{entry, Pins, Sets};

// TODO: グローバル変数を初期化します


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // TODO: グローバル変数に格納されているNoneをSomeで上書きします

    // TODO: わざとNoneをunwrap()してパニックを発生させます

    loop {}
}

// TODO: パニックハンドラを実装します


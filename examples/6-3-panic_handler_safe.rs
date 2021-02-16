//! 8-3 シリアル入出力/UARTのサンプルコードです。
//! MutexとRefCellを使って安全にグローバル変数を共有するパニックハンドラ実装です。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example safe_panic_handler
//! ```

#![no_std]
#![no_main]

use wio_terminal as wio;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;
use core::panic::PanicInfo;
use cortex_m::interrupt::{self, Mutex};
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

    // TODO: グローバル変数に格納されているNoneを安全にSomeで上書きします

    // TODO: わざとNoneをunwrap()してパニックを発生させます

    let none: Option<usize> = None;
    none.unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: 安全にグローバル変数を使ってメッセージを出力します

    loop {}
}

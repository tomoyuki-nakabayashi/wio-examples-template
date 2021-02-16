//! 6-3 シリアル入出力/UARTのサンプルコードです。
//! グローバル変数を使ったパニックハンドラ実装です。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-3-panic_handler_global
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

// 絶対に初期化しないといけないので、いったんNoneで初期化する
static mut UART: Option<
    UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>,
> = None;

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

    // TODO: グローバル変数に格納されているNoneをSomeで上書きする

    // TODO: わざとNoneをunwrap()してパニックを発生させる

    loop {}
}

// TODO: パニックハンドラを実装する


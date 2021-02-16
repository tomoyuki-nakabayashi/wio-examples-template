//! 7-4 Wio TerminalのLCDにHello World!のサンプルコードです。
//! Wio Terminal の LCD に `Hello Wio Terminal!` と表示します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 7-4-hello_lcd
//! ```

#![no_std]
#![no_main]

use embedded_graphics as eg;
use panic_halt as _;
use wio_terminal as wio;

use eg::{fonts::*, pixelcolor::*, prelude::*, primitives::*, style::*};
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut sets = Pins::new(peripherals.PORT).split();

    // TODO: ディスプレイドライバを初期化する

    // TODO: LCDを黒色で塗りつぶす

    // TODO: 画面情報に「Hello Wio Terminal!」と表示する

    loop {}
}

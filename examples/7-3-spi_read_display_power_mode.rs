//! 7-3 LCD/SPIのサンプルコードです。
//! SPIでILI9341からDisplay Power Modeを取得します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 7-3-spi_read_display_power_mode
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::gpio::*;
use wio::hal::sercom::*;
use wio::hal::hal::spi;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

#[entry]
fn main() -> ! {
    // 1. ドライバの初期化
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

    let mut sets = wio::Pins::new(peripherals.PORT).split();
    // UARTドライバオブジェクトを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // TODO: SPIドライバオブジェクトを初期化する

    // TODO: その他の制御信号を出力に設定する

    // TODO: 2. ILI9341のハードウェアリセット

    // TODO: 3. Read Display Power Modeコマンド（0x0A）の送信

    // TODO: 4. データ出力の読み込み

    // TODO: 5. 読み込んだデータをシリアルに出力

    loop {}
}

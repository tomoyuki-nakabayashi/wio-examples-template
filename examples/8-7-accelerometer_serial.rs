//! 8-7 加速度センサ/I2Cのサンプルコードです。
//! 1秒ごとに加速度センサから値を読み出します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example accelerometer_serial
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use accelerometer::{vector::F32x3, Accelerometer};
use core::fmt::Write;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    // クロックを初期化します
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut sets = wio::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // UARTドライバオブジェクトを初期化します
    let mut serial = sets.uart.init(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // TODO: 加速度センサドライバオブジェクトを初期化します
    let mut accel = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // TODO: デバイスIDを取得します。0x33 が格納されています。

    // TODO: 1秒毎に加速度センサから読み取った値をシリアルに出力します
    loop {

    }
}

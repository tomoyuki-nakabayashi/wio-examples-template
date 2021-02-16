//! 8-5 ブザー/PWMのサンプルコードです。
//! ドレミファソラシドと1秒ずつ鳴ります。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example buzzer
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::pwm::Channel;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins};

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

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut sets = Pins::new(peripherals.PORT).split();
    // TODO: ブザー (PWM) ドライバオブジェクトを初期化します

    //           ド   レ    ミ   ファ  ソ   ラ   シ    ド
    let freqs = [261, 294, 329, 349, 392, 440, 494, 523];
    loop {
        for freq in freqs.iter() {
            // TODO: 周期 (周波数) を設定します

            // TODO: デューティ比を 50% に設定します

            // TODO: 1秒鳴らして止めます

        }
    }
}

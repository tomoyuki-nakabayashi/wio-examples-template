//! 8-4 タイマ/割り込みのサンプルコードです。
//! 1秒間隔でLEDが点滅します
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example busy_wait
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};
use wio_examples::Led;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    // LEDドライバオブジェクトを初期化します
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut led = Led::new(sets.user_led, &mut sets.port);

    // TODO: Delay構造体オブジェクトを取得します

    loop {
        // TODO: Lチカのコードを書きます
        
    }
}

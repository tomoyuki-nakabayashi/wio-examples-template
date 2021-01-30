#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;

use accelerometer::Accelerometer;
use embedded_graphics::{
    fonts::{Font12x16, Text},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::Rectangle,
    style::{PrimitiveStyleBuilder, TextStyle},
};
use heapless::consts::*;
use heapless::String;
use micromath::F32Ext;
use wio::entry;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;

#[no_mangle]
fn fminf(a: f32, b: f32) -> f32 {
    match a.partial_cmp(&b) {
        None => a,
        Some(core::cmp::Ordering::Less) => a,
        Some(core::cmp::Ordering::Equal) => a,
        Some(core::cmp::Ordering::Greater) => b,
    }
}
#[no_mangle]
fn fmaxf(a: f32, b: f32) -> f32 {
    match a.partial_cmp(&b) {
        None => a,
        Some(core::cmp::Ordering::Less) => b,
        Some(core::cmp::Ordering::Equal) => b,
        Some(core::cmp::Ordering::Greater) => a,
    }
}

fn draw_rectangle<
    TargetColor: RgbColor,
    Target: DrawTarget<TargetColor>,
    Color: Into<TargetColor>,
>(
    target: &mut Target,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    color: Color,
) {
    if width <= 0 || height <= 0 {
        return;
    }

    let _ = Rectangle::new(
        Point::new(left, top),
        Point::new(left + width - 1, top + height - 1),
    )
    .into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_width(0)
            .fill_color(color.into())
            .build(),
    )
    .draw(target);
}

fn draw_text<
    TargetColor: RgbColor,
    Target: DrawTarget<TargetColor>,
    Color: Into<TargetColor>,
    StyleFont: Font + Copy,
>(
    target: &mut Target,
    text: &str,
    x: i32,
    y: i32,
    font: StyleFont,
    color: Color,
) {
    let _ = Text::new(text, Point::new(x, y))
        .into_styled(TextStyle::new(font, color.into()))
        .draw(target);
}

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
    let mut sets = wio::Pins::new(peripherals.PORT).split();
    let mut led = sets.user_led.into_push_pull_output(&mut sets.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // 加速度センサドライバオブジェクトを初期化します
    let mut accel = sets.accelerometer.init(
        &mut clocks,
        peripherals.SERCOM4,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    let accel_id = accel.get_device_id().unwrap();

    // ディスプレイドライバを初期化します
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            58.mhz(),
            &mut delay,
        )
        .unwrap();

    // 画面を黒でクリア
    draw_rectangle(&mut display, 0, 0, 320, 240, Rgb565::BLACK);

    // 画面に加速度センサのIDを16進数で表示
    let mut text_buffer = String::<U128>::new();
    write!(&mut text_buffer, "{:X}", accel_id).unwrap();
    draw_text(&mut display, &text_buffer, 0, 0, Font12x16, Rgb565::WHITE);

    loop {
        // 加速度を取得
        let data = accel.accel_norm().unwrap();

        // XYZ軸の各加速度を画面に表示
        text_buffer.clear();
        write!(
            &mut text_buffer,
            "{:.2}, {:.2}, {:.2}",
            data.x, data.y, data.z
        )
        .unwrap();
        // 加速度の数値の範囲のみクリアしてフォーマットした文字を描画
        draw_rectangle(&mut display, 0, 20, 320, 20, Rgb565::BLACK);
        draw_text(&mut display, &text_buffer, 0, 20, Font12x16, Rgb565::WHITE);

        // XYZ軸の加速度の絶対値をそれぞれRGBにマップ
        let red = (data.x * 255f32).abs().max(0f32).min(255f32) as u8;
        let green = (data.y * 255f32).abs().max(0f32).min(255f32) as u8;
        let blue = (data.z * 255f32).abs().max(0f32).min(255f32) as u8;

        // RGBに対応する色の箱を描画
        draw_rectangle(&mut display, 72 * 0, 40, 20, 20, Rgb888::new(red, 0, 0));
        draw_rectangle(&mut display, 72 * 1, 40, 20, 20, Rgb888::new(0, green, 0));
        draw_rectangle(&mut display, 72 * 2, 40, 20, 20, Rgb888::new(0, 0, blue));
        // RGBを混ぜた色の箱を描画
        draw_rectangle(
            &mut display,
            72 * 3,
            40,
            20,
            20,
            Rgb888::new(red, green, blue),
        );
        // 次の処理まで500[ms]まつ
        delay.delay_ms(500u16);
        led.toggle();
    }
}

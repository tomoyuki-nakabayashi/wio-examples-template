//! 8-2 マイク音声の信号処理をする
//! マイクから入力した音声をフーリエ変換してパワースペクトラムを表示します
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 8-2-mic_fft --features app --release
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use core::fmt::Write;
use cortex_m::peripheral::NVIC;
use heapless::consts::*;
use heapless::Vec;
use micromath::F32Ext;
use wio::entry;
use wio::hal::adc::{FreeRunning, InterruptAdc};
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::time::Hertz;
use wio::pac::{interrupt, CorePeripherals, Peripherals, ADC1};
use wio::prelude::*;
use wio::Pins;

use eg::{egrectangle, pixelcolor::Rgb565, primitive_style};
use eg::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics as eg;

// main() 関数とADCの割り込みハンドラで共有するリソース
struct Ctx {
    adc: InterruptAdc<ADC1, FreeRunning>,
    buffers: [SamplingBuffer; 2], // ADC結果のバッファ2面分
    // 現在ADC結果取り込み先のバッファへの参照
    sampling_buffer: Option<&'static mut SamplingBuffer>,
    // 現在信号処理中のバッファへの参照
    processing_buffer: Option<&'static mut SamplingBuffer>,
}

static mut CTX: Option<Ctx> = None;

const AVERAGING_FACTOR: u32 = 4; // 平均化フィルタのサンプル点数
const FFT_POINTS: usize = 256; // FFTをするサンプル点数
const ADC_SAMPLING_RATE: f32 = 83333.0; // ADCのサンプリングレート
#[allow(dead_code)]
// 平均化フィルタ後のサンプリングレート
const SAMPLING_RATE: f32 = ADC_SAMPLING_RATE / AVERAGING_FACTOR as f32;
const AMPLITUDE: f32 = 4096.0; // サンプル値の最大振幅

type SamplingBuffer = heapless::Vec<f32, U256>; //サンプリングバッファの型

// f32::max,f32::minが
// プラットフォームのライブラリとしてfmaxf,fminfがあることを前提としているが、
// 現在の環境にはfmaxf,fminfがないので、最低限のものを実装しておく
// Cから呼び出せる形式でなければならないので、`#[no_mangle]`を付ける
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

    let mut sets = Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // TODO: フリーランニングモードでADCを動かすようにInterruptAdc型を構築する

    // デバッグ用UARTを初期化する
    let mut serial = sets.uart.init(
        &mut clocks,
        Hertz(115200u32),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 画面を初期化する
    let (mut display, _backlight) = sets
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM7,
            &mut peripherals.MCLK,
            &mut sets.port,
            60.mhz(),
            &mut delay,
        )
        .unwrap();

    // TODO: 共有リソースを初期化する

    // ADC変換完了割り込み(RESRDY)を有効にしてサンプリングを開始する
    writeln!(&mut serial, "start").unwrap();

    unsafe { NVIC::unmask(interrupt::ADC1_RESRDY); }

    let button_restart =
        sets.buttons.button1.into_floating_input(&mut sets.port);
    let button_stop =
        sets.buttons.button2.into_floating_input(&mut sets.port);

    // FFTの窓関数としてHann窓を使うので係数を計算しておく
    // 振幅の正規化用に最大振幅で割っておく
    let mut hann_factor = [0f32; FFT_POINTS];
    for i in 0..FFT_POINTS {
        use core::f32::consts::PI;
        hann_factor[i] = 0.5f32
            * (1f32 - (PI * 2.0f32 * i as f32 / FFT_POINTS as f32).cos())
            / AMPLITUDE;
    }
    let hann_factor = hann_factor;

    // 画面のスペクトラム表示領域の内容を消す
    const SCREEN_WIDTH: i32 = 320;
    const SCREEN_HEIGHT: i32 = 240;
    fn clear_screen<T: embedded_graphics::DrawTarget<Rgb565>>(
        display: &mut T,
    ) -> Result<(), T::Error> {
        egrectangle!(
            top_left = (0, 0),
            bottom_right = (SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
            style = primitive_style!(fill_color = Rgb565::BLACK)
        )
        .draw(display)
    };
    clear_screen(&mut display).unwrap();

    const BAR_WIDTH: i32 = 2;
    const REAL_POINTS: usize = FFT_POINTS / 2;
    const NUMBER_OF_BARS: usize = REAL_POINTS;
    const DRAW_AREA_WIDTH: i32 =
        BAR_WIDTH * (NUMBER_OF_BARS as i32 + 1);
    let mut prev_bar_position = [0u8; NUMBER_OF_BARS as usize];
    let mut stop_req = false;
    let mut stop_ack = false;
    loop {
        // TODO: `processing_buffer`が埋まっていれば、FFTを実行しスペクトラムを描画する
        //       停止ボタンが押された場合は、棒グラフを表示する
    }
}

#[interrupt]
fn ADC1_RESRDY() {
    // TODO: データをサンプリングし、`sampling_buffer`を埋める。
    //       `sampling_buffer`がいっぱいになったら`processing_buffer`と入れ替える。
}

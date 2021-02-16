//! 10-2 マイク音声の信号処理をする
//! マイクから入力した音声をフーリエ変換してパワースペクトルを表示します
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 10-2-mic_fft --features app --release
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
use wio::hal::gpio::{Output, Pb8, PushPull};
use wio::hal::time::Hertz;
use wio::pac::{interrupt, CorePeripherals, Peripherals, ADC1};
use wio::prelude::*;
use wio::Pins;

use eg::prelude::*;
use eg::{egline, egrectangle, pixelcolor::Rgb565, primitive_style};
use embedded_graphics as eg;

// main() 関数とADCの割り込みハンドラで共有するリソース
struct Ctx {
    adc: InterruptAdc<ADC1, FreeRunning>,
    buffers: [SamplingBuffer; 2], // ADC結果のバッファ (サンプリングバッファ) 2面分
    sampling_buffer: Option<&'static mut SamplingBuffer>, // 現在ADC結果取り込み先のバッファへの参照
    processing_buffer: Option<&'static mut SamplingBuffer>, // 現在信号処理中のバッファへの参照
    average: f32,                 // 平均値
    average_count: u32,           // 平均値計算時のサンプル数カウント
    debug_pin: Pb8<Output<PushPull>>, // デバッグ出力用ピン (信号処理時間計測用)
}

static mut CTX: Option<Ctx> = None;

const AVERAGING_FACTOR: u32 = 2; // 平均化フィルタのサンプル点数
const FFT_POINTS: usize = 512; // FFTをするサンプル点数
const ADC_SAMPLING_RATE: f32 = 83333.0; // ADCのサンプリングレート (83.333[kHz])
const SAMPLING_RATE: f32 = ADC_SAMPLING_RATE / AVERAGING_FACTOR as f32; // 平均化フィルタ後のサンプリングレート
const AMPLITUDE: f32 = 4096.0; // サンプル値の最大振幅

type SamplingBuffer = Vec<f32, U512>; // サンプリングバッファの型 (f32 512点分)

// f32::max, f32::minがプラットフォームのライブラリとしてfmaxf, fminfがあることを前提としているが、
// 現在の環境には fmaxf, fminf が無いので、とりあえず最低限の物を実装しておく。
// Cから呼び出せる形式でなければならないので、`#[no_mangle]` をつける。
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

    // フリーランニングモードでADCを動かすようにInterruptAdc型を構築する
    let (mut microphone_adc, mut microphone_pin) = {
        let (adc, pin) = sets.microphone.init(
            peripherals.ADC1,
            &mut clocks,
            &mut peripherals.MCLK,
            &mut sets.port,
        );
        let interrupt_adc: InterruptAdc<_, FreeRunning> = InterruptAdc::from(adc);
        (interrupt_adc, pin)
    };

    // ADCの変換処理を開始する
    microphone_adc.start_conversion(&mut microphone_pin);

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

    // デバッグ信号出力用ピンを初期化する
    let debug_pin = sets.header_pins.a0_d0.into_push_pull_output(&mut sets.port);
    let mut debug_pin_2 = sets.header_pins.a1_d1.into_push_pull_output(&mut sets.port);

    // 共有リソースを初期化する
    unsafe {
        CTX = Some(Ctx {
            adc: microphone_adc,
            buffers: [Vec::new(), Vec::new()],
            sampling_buffer: None,
            processing_buffer: None,
            average: 0.0,
            average_count: 0,
            debug_pin,
        });
        // 2面分のサンプリングバッファを取り込み用と処理用にそれぞれ割り当てる
        let mut ctx = CTX.as_mut().unwrap();
        let (first, rest) = ctx.buffers.split_first_mut().unwrap();
        ctx.sampling_buffer = Some(first);
        ctx.processing_buffer = Some(&mut rest[0]);
    }

    // ADC変換完了割り込み(RESRDY)を有効にしてサンプリングを開始する
    writeln!(&mut serial, "start").unwrap();

    unsafe {
        NVIC::unmask(interrupt::ADC1_RESRDY);
    }

    // FFTの窓関数としてHann窓を使うので係数を計算しておく
    let hann_factor = {
        let mut factor = [0f32; FFT_POINTS];
        for i in 0..FFT_POINTS {
            factor[i] = 0.5f32
                * (1f32 - (core::f32::consts::PI * 2.0f32 * i as f32 / FFT_POINTS as f32).cos());
        }
        factor
    };

    loop {
        unsafe {
            let ctx = CTX.as_mut().unwrap();
            // 処理対象バッファにFFT点数分のサンプルデータが入っている？
            if ctx.processing_buffer.as_mut().unwrap().len()
                == ctx.processing_buffer.as_mut().unwrap().capacity()
            {
                debug_pin_2.set_high().unwrap();
                // Hann窓の係数を掛ける
                let processing_buffer = ctx.processing_buffer.as_mut().unwrap();
                for i in 0..FFT_POINTS {
                    processing_buffer[i] *= hann_factor[i];
                }
                // 実部のみの入力に対する512点FFTを実行する
                let result = microfft::real::rfft_512(processing_buffer.as_mut());

                // 画面のスペクトル表示領域の内容を消す
                let offset_top = 0;
                let offset_left = (320 - 256) / 2;
                let area_height = 240;
                egrectangle!(
                    top_left = (offset_left, 0),
                    bottom_right = (offset_left + 256, 240),
                    style = primitive_style!(fill_color = Rgb565::BLACK)
                )
                .draw(&mut display)
                .unwrap();

                // スペクトルを描画する
                let _resolution = SAMPLING_RATE / FFT_POINTS as f32;
                let mut prev_point = None; // 直前の周波数の描画位置
                for (index, spectrum) in result.iter().enumerate() {
                    // パワーの計算
                    let power = spectrum.norm_sqr()
                        / (FFT_POINTS * FFT_POINTS) as f32
                        / (AMPLITUDE * AMPLITUDE);
                    // 対数にする
                    let relative_power = if power <= 0.0 {
                        core::f32::NEG_INFINITY
                    } else {
                        power.log10() * 10.0
                    };
                    // 値からY座標を計算
                    let height = -((relative_power * 2.0)
                        .round()
                        .max(-area_height as f32)
                        .min(0.0) as i32);
                    let end = (offset_left + index as i32, offset_top + height);
                    let start = prev_point.unwrap_or(end);

                    egline!(
                        start = start,
                        end = end,
                        style = primitive_style!(stroke_color = Rgb565::WHITE, stroke_width = 1)
                    )
                    .draw(&mut display)
                    .unwrap();
                    prev_point = Some(end);
                }

                // 処理が終わったので処理用バッファをクリアする
                processing_buffer.clear();
                debug_pin_2.set_low().unwrap();
            }
        }
    }
}

#[interrupt]
fn ADC1_RESRDY() {
    unsafe {
        let mut ctx = CTX.as_mut().unwrap();
        ctx.debug_pin.set_high().unwrap();
        if let Some(sample) = ctx.adc.service_interrupt_ready() {
            // サンプルデータがあれば平均値計算のために積算する
            ctx.average += sample as f32;
            ctx.average_count += 1;
            if ctx.average_count == AVERAGING_FACTOR {
                // 平均値計算回数分のサンプルデータを積算した
                let sampling_buffer = ctx.sampling_buffer.as_mut().unwrap();
                if sampling_buffer.len() == sampling_buffer.capacity() {
                    // サンプリングバッファがいっぱいなので、処理用バッファが空、つまり処理が終わっているなら入れ替える
                    if ctx.processing_buffer.as_mut().unwrap().len() == 0 {
                        core::mem::swap(&mut ctx.processing_buffer, &mut ctx.sampling_buffer);
                    }
                } else {
                    // サンプリングバッファに平均値を追加する
                    let _ = sampling_buffer.push(ctx.average / (AVERAGING_FACTOR as f32));
                }
                // 積算カウントを0に戻す
                ctx.average_count = 0;
                ctx.average = 0.0;
            }
        }
        ctx.debug_pin.set_low().unwrap();
    }
}

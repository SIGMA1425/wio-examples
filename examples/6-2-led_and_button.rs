//! 6-2 LEDとボタン/GPIOのサンプルコードです。
//! ボタン1 (一番右のボタン) を押している間、ユーザーLEDが点灯します。
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-2-led_and_button
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::pac::Peripherals;
use wio::prelude::*; // 主要な構造体やトレイトをインポートする

#[entry]
fn main() -> ! {
    //PACのPeripheralsオブジェクトの取得
    let peripherals = Peripherals::take().unwrap();
    //PORTの所有権を消費して，GPIOピンがまとめられたオブジェクトを初期化する
    //内部でsplitメソッドを呼び出している
    let mut pins = wio::Pins::new(peripherals.PORT);

    //ユーザLEDのGPIOピンを出力に設定
    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);
    //ボタン１を入力状態に設定
    let button1 = pins.button1.into_floating_input(&mut pins.port);

    loop {
        if button1.is_low().unwrap(){
            //ボタンが押されていればユーザLEDを点灯
            led.set_high().unwrap();
        }
        else{
            //ボタンが押されていなければユーザLEDを消灯
            led.set_low().unwrap();
        }
    }
}

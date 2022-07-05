use std::{thread::sleep, time::Duration};
use device_query::Keycode;
use toca::record::KeyboardRecorder;

/// 顶层模块下, 供子模块调用
fn set_timeout<T>(mut callback: T, ms: u64)
    where T: FnMut() -> () {
    sleep(Duration::from_millis(ms));
    callback();
}

#[allow(unused)]
fn main() {
    let mut recorder = KeyboardRecorder::new();

    println!("record start.");
    recorder.do_record(Keycode::Escape);
    println!("record stop.");

    for ev in recorder.get_record() {
        println!("[{}ms]: {} {}", ev.timestamp, (if ev.press { "Press" } else { "Release" }), ev.code);
    }
}

mod dq_record;
mod toca;

use enigo::Key;
use toca::{Action, Toca};
use dq_record::KeyboardRecorder;

use std::{thread::sleep, time::Duration};
use device_query::Keycode;

/// 顶层模块下, 供子模块调用
fn set_timeout<T>(mut callback: T, ms: u64)
    where T: FnMut() -> () {
    sleep(Duration::from_millis(ms));
    callback();
}

#[allow(unused)]
fn main() {
    let mut recorder = KeyboardRecorder::new();

    // let stop_code = Keycode::Escape;
    println!("record start.");
    recorder.start_record(Keycode::Escape);
    println!("record stop.");

    for ev in recorder.get_record() {
        println!("[{}ms]: {}", ev.timestamp, ev.code);
    }
}

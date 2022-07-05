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

    // let stop_code = Keycode::Escape;
    println!("record start.");
    let action = recorder.do_record(Keycode::Escape);
    println!("record stop. total time is : {}", action.till);

    for ev in action.evs {
        println!("[{}ms]: {}", ev.timestamp, ev.code);
    }
}

use std::{thread::sleep, time::Duration};
use device_query::Keycode;
use toca::record::KeyboardRecorder;

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

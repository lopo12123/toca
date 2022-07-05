extern crate toca;

use toca::record::KeyboardRecorder;
use toca::Keycode;

fn main() {
    let mut recorder = KeyboardRecorder::new();

    println!("record start. (press any key to record, press ESC to stop.)");
    let action = recorder.do_record(Keycode::Escape);
    println!("record stop. duration: {}ms", action.till);

    for ev in action.evs {
        println!("[{}ms]: {} {}", ev.timestamp, (if ev.press { "Press" } else { "Release" }), ev.code);
    }
}
extern crate toca;

use toca::{record::KeyboardRecorder, Keycode};

fn main() {
    let mut recorder = KeyboardRecorder::new();

    println!("record start. (press any key to record, press ESC to stop.)");
    let action = recorder.do_record(Keycode::Escape);
    println!("record stop. duration: {}ms", action.till);

    // to_string
    println!("in json:\n{:?}\n", action.to_string().unwrap());

    // directly use
    for ev in action.evs {
        println!("[{}ms]: {} {}", ev.timestamp, (if ev.press { "Press" } else { "Release" }), ev.code);
    }
}
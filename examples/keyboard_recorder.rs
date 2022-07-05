extern crate toca;

use toca::record::KeyboardRecorder;
use toca::Keycode;

fn main() {
    let mut recorder = KeyboardRecorder::new();

    println!("record start. (press any key to record, press ESC to stop.)");
    recorder.do_record(Keycode::Escape);
    println!("record stop.");

    for ev in recorder.get_record() {
        println!("[{}ms]: {} {}", ev.timestamp, (if ev.press { "Press" } else { "Release" }), ev.code);
    }
}
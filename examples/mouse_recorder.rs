extern crate toca;

use toca::{record::MouseRecorder, Keycode};

fn main() {
    let mut recorder = MouseRecorder::new();

    println!("record start. (press ESC to stop.)");
    let action = recorder.do_record(Keycode::Escape);
    println!("record stop. duration: {}ms", action.till);

    for ev in action.evs {
        println!("[{}ms]: {:?} at {:?}", ev.timestamp, ev.ev_name, ev.position);
    }
}
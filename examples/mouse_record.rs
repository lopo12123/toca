extern crate toca;

use toca::{record::MouseRecorder, Keycode};

fn main() {
    let mut recorder = MouseRecorder::new();

    println!("record start. (press ESC to stop.)");
    let action = recorder.do_record(Keycode::Escape);
    println!("record stop. duration: {}ms", action.till);

    // to_string
    println!("in json:\n{:?}\n", action.to_string().unwrap());

    // directly use
    for ev in action.evs {
        println!("[{}ms]: {:?} at {:?}", ev.timestamp, ev.ev_name, ev.position);
    }
}
extern crate toca;

use device_query::Keycode;
use toca::{KeyboardAction, KeyboardEv, KeyboardPlayer, set_timeout};

fn main() {
    // mock action
    let mock_action = KeyboardAction {
        evs: vec![
            KeyboardEv {
                code: Keycode::H,
                press: true,
                timestamp: 500,
            },
            KeyboardEv {
                code: Keycode::E,
                press: true,
                timestamp: 1500,
            },
            KeyboardEv {
                code: Keycode::L,
                press: true,
                timestamp: 2000,
            },
            KeyboardEv {
                code: Keycode::L,
                press: true,
                timestamp: 2500,
            },
            KeyboardEv {
                code: Keycode::O,
                press: true,
                timestamp: 3000,
            },
            KeyboardEv {
                code: Keycode::Space,
                press: true,
                timestamp: 3500,
            },
            KeyboardEv {
                code: Keycode::W,
                press: true,
                timestamp: 4000,
            },
            KeyboardEv {
                code: Keycode::O,
                press: true,
                timestamp: 4500,
            },
            KeyboardEv {
                code: Keycode::R,
                press: true,
                timestamp: 5000,
            },
            KeyboardEv {
                code: Keycode::L,
                press: true,
                timestamp: 5500,
            },
            KeyboardEv {
                code: Keycode::D,
                press: true,
                timestamp: 6000,
            },
            KeyboardEv {
                code: Keycode::LShift,
                press: true,
                timestamp: 6500,
            },
            KeyboardEv {
                code: Keycode::Numpad1,
                press: true,
                timestamp: 6600,
            },
            KeyboardEv {
                code: Keycode::LShift,
                press: false,
                timestamp: 6700,
            },
        ],
        till: 6700,
    };

    // simulate
    let mut player = KeyboardPlayer::new();
    match player.load(mock_action) {
        Ok(_) => {
            set_timeout(|| {
                match player.do_play() {
                    Ok(_) => {
                        println!("done.")
                    }
                    Err(_) => {
                        println!("failed.")
                    }
                }
            }, 3_000)
        }
        Err(_) => println!("error when load actions.")
    }

    // auto print: hello world!
}
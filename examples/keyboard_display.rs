extern crate toca;

use device_query::Keycode;
use toca::{KeyboardAction, KeyboardEv, KeyboardPlayer, set_timeout};

fn from_string() {
    // mock action
    let mock_action = KeyboardAction::from_string("{\"evs\":[{\"code\":\"KeyH\",\"press\":true,\"timestamp\":2065},{\"code\":\"KeyH\",\"press\":false,\"timestamp\":2144},{\"code\":\"KeyE\",\"press\":true,\"timestamp\":2391},{\"code\":\"KeyE\",\"press\":false,\"timestamp\":2484},{\"code\":\"KeyL\",\"press\":true,\"timestamp\":2620},{\"code\":\"KeyL\",\"press\":false,\"timestamp\":2700},{\"code\":\"KeyL\",\"press\":true,\"timestamp\":2753},{\"code\":\"KeyL\",\"press\":false,\"timestamp\":2846},{\"code\":\"KeyO\",\"press\":true,\"timestamp\":3109},{\"code\":\"KeyO\",\"press\":false,\"timestamp\":3194},{\"code\":\"Space\",\"press\":true,\"timestamp\":3501},{\"code\":\"Space\",\"press\":false,\"timestamp\":3612},{\"code\":\"KeyW\",\"press\":true,\"timestamp\":3797},{\"code\":\"KeyW\",\"press\":false,\"timestamp\":3878},{\"code\":\"KeyO\",\"press\":true,\"timestamp\":3997},{\"code\":\"KeyO\",\"press\":false,\"timestamp\":4075},{\"code\":\"KeyR\",\"press\":true,\"timestamp\":4123},{\"code\":\"KeyR\",\"press\":false,\"timestamp\":4223},{\"code\":\"KeyL\",\"press\":true,\"timestamp\":4483},{\"code\":\"KeyL\",\"press\":false,\"timestamp\":4569},{\"code\":\"KeyD\",\"press\":true,\"timestamp\":4684},{\"code\":\"KeyD\",\"press\":false,\"timestamp\":4787},{\"code\":\"ShiftRight\",\"press\":true,\"timestamp\":5004},{\"code\":\"Digit1\",\"press\":true,\"timestamp\":5146},{\"code\":\"Digit1\",\"press\":false,\"timestamp\":5235},{\"code\":\"ShiftRight\",\"press\":false,\"timestamp\":5313}],\"till\":6668}").unwrap();

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

fn from_struct() {
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

fn main() {
    // from_string();
    from_struct();
}
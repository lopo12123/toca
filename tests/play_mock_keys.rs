use device_query::Keycode;
use enigo::{KeyboardControllable};
use toca::display::KeyboardPlayer;
use toca::record::{KeyboardAction, KeyboardEv};
use toca::set_timeout;

#[test]
fn play_mock_keys() {
    let mock_key_action = KeyboardAction {
        evs: vec![
            KeyboardEv {
                code: Keycode::LShift,
                press: true,
                timestamp: 1_000,
            },
            KeyboardEv {
                code: Keycode::B,
                press: true,
                timestamp: 2_000,
            },
            KeyboardEv {
                code: Keycode::LShift,
                press: false,
                timestamp: 3_000,
            },
        ],
        till: 3_000,
    };
    let mut player = KeyboardPlayer::new();

    match player.load(mock_key_action) {
        Ok(_) => {
            set_timeout(|| {
                match player.do_play() {
                    Ok(_) => {
                        println!("success.")
                    }
                    Err(_) => {
                        println!("failed.")
                    }
                }
            }, 3000);
        }
        Err(_) => {
            println!("error when load actions.");
        }
    };
}
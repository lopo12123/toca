use device_query::Keycode;
use enigo::{Key, KeyboardControllable};
use toca::display::KeyboardPlayer;
use toca::record::{KeyboardAction, KeyboardEv};
use toca::set_timeout;

#[test]
fn play_mock_keys() {
    let mock_key_action = KeyboardAction {
        evs: vec![
            KeyboardEv {
                code: Keycode::A,
                press: true,
                timestamp: 1_000,
            },
            KeyboardEv {
                code: Keycode::B,
                press: true,
                timestamp: 2_000,
            },
            KeyboardEv {
                code: Keycode::Comma,
                press: true,
                timestamp: 3_000,
            },
        ],
        till: 3_000,
    };
    let mut player = KeyboardPlayer::new();
    player.load(mock_key_action);

    set_timeout(|| {
        player.do_play();
    }, 3000);
}
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

#[test]
fn enigo_test() {
    let mut en = enigo::Enigo::new();
    set_timeout(|| {
        en.key_click(Key::Layout('0'));
        en.key_click(Key::Layout('2'));
        en.key_click(Key::Layout('4'));
        en.key_click(Key::Layout('l'));
        en.key_click(Key::Layout('o'));
        en.key_click(Key::Layout('`'));
        en.key_click(Key::Layout('-'));
        en.key_click(Key::Layout('='));
        en.key_click(Key::Layout('/'));
        en.key_click(Key::Layout('\\'));
        en.key_click(Key::Layout(';'));
        en.key_click(Key::Layout('\''));
        en.key_click(Key::Layout('*'));
    }, 3000);
}
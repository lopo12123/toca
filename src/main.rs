use device_query::Keycode;
use enigo::{Key, KeyboardControllable};
use toca::display::KeyboardPlayer;
use toca::record::{KeyboardAction, KeyboardEv};
use toca::set_timeout;

#[allow(unused)]
fn main() {
    println!("nothing here, see examples.");

    // let mock_key_action = KeyboardAction {
    //     evs: vec![KeyboardEv {
    //         code: Keycode::A,
    //         press: true,
    //         timestamp: 4_000,
    //     }],
    //     till: 4_0000,
    // };
    // let mut player = KeyboardPlayer::new();
    // player.load(mock_key_action);
    // player.do_play();

    let mut en = enigo::Enigo::new();
    set_timeout(|| {
        en.key_click(Key::Layout('A'));
    }, 3000);
}

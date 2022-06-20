mod toca;

use enigo::Key;
use toca::{Action, Toca};

#[allow(unused)]
fn main() {
    let mut ins = Toca::new();

    ins.add_actions(&mut vec![
        Action::MouseMoveAbsolute { delay: 2000, target: [2000, 1300] },
        Action::MouseLeft { delay: 100 },
        Action::KeyClick { delay: 100, key: Key::Layout('h') },
        Action::KeyClick { delay: 100, key: Key::Layout('e') },
        Action::KeyClick { delay: 100, key: Key::Layout('l') },
        Action::KeyClick { delay: 100, key: Key::Layout('l') },
        Action::KeyClick { delay: 100, key: Key::Layout('o') },
        Action::KeyClick { delay: 100, key: Key::Return },
        Action::KeyClick { delay: 100, key: Key::Return },
    ]);

    ins.play_actions();
    println!("done!");
}

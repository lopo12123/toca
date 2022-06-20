mod toca;

use enigo::Key;
use toca::{Action, Toca};

#[allow(unused)]
fn main() {
    let mut ins = Toca::new();

    ins.add_actions(&mut vec![
        Action::MouseMove { delay: 2000, target: [2000, 1300] },
        Action::MouseLeft { delay: 100 },
        Action::KeyClick { delay: 100, key: Key::Layout('h') },
        Action::KeyClick { delay: 100, key: Key::Layout('e') },
        Action::KeyClick { delay: 100, key: Key::Layout('l') },
        Action::KeyClick { delay: 100, key: Key::Layout('l') },
        Action::KeyClick { delay: 100, key: Key::Layout('o') },
    ]);

    ins.play_actions();
    println!("done!");
}

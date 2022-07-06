use enigo::KeyboardControllable;
use toca::set_timeout;

#[allow(unused)]
fn main() {
    println!("nothing here, see examples.");

    set_timeout(|| {
        let mut en = enigo::Enigo::new();
        en.key_click(enigo::Key::Layout('['));
        en.key_click(enigo::Key::Layout(']'));
        en.key_click(enigo::Key::Layout('-'));
        en.key_click(enigo::Key::Layout('='));
        en.key_click(enigo::Key::Return);
    }, 4_000);
}

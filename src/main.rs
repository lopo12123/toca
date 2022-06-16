mod toca;

use toca::Toca;
use crate::toca::Action;

fn main() {
    let mut ins = Toca::new();
    ins.play_single(&Action::MouseMove { delay: 2000, target: [200, 200] });
    println!("Hello, world!");
}

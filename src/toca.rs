use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Key, MouseControllable};

// region misc
/// function like `setTimeout` in JavaScript.
fn set_timeout(mut callback: impl FnMut() -> (), timeout: u64) {
    sleep(Duration::from_millis(timeout));
    callback();
}
// endregion

/// type of action to play in pipeline
pub enum Action {
    /// mouse move
    MouseMove { delay: u64, target: [i32; 2] },
    /// mouse left-click
    MouseLeft { delay: u64, target: [i32; 2] },
    /// mouse right-click
    MouseRight { delay: u64, target: [i32; 2] },
    /// key click
    KeyClick { delay: u64, key: Key },
    /// key down
    KeyDown { delay: u64, key: Key },
    /// Key up
    KeyUp { delay: u64 },
}

pub struct Toca {
    instance: Enigo,
    total_time: u64,
    actions: Vec<Action>,
}

impl Toca {
    /// constructor
    pub fn new() -> Toca {
        Toca {
            instance: Enigo::new(),
            total_time: 0,
            actions: vec![],
        }
    }

    pub fn get_total_time(&self) -> u64 {
        self.total_time
    }

    /// play a single action.
    pub fn play_single(&mut self, action: &Action) {
        match *action {
            Action::MouseMove { delay, target } => {
                set_timeout(|| {
                    self.instance.mouse_move_to(target[0], target[1]);
                }, delay);
            }

            _ => {
                println!("invalid action");
            }
        }
    }

    /// play all actions in action pool.
    pub fn play_actions(&mut self) {
        let mut p = 0;
        while p < self.actions.len() {
            self.play_single(&self.actions[p]);
            p += 1;
        }
    }
}
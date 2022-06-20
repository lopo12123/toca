use std::thread::sleep;
use std::time::Duration;
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};

// region misc
/// a function like `setTimeout` in JavaScript.
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
    MouseLeft { delay: u64 },
    /// mouse right-click
    MouseRight { delay: u64 },
    /// key click
    KeyClick { delay: u64, key: Key },
    /// key press
    KeyPress { delay: u64, key: Key, duration: u64 },
}

pub struct Toca {
    instance: Enigo,
    total_time: u64,
    actions: Vec<Action>,
}

#[allow(unused)]
impl Toca {
    // constructor.
    pub fn new() -> Toca {
        Toca {
            instance: Enigo::new(),
            total_time: 0,
            actions: vec![],
        }
    }

    // how many time it takes for whole actions to play.
    pub fn get_time_count(&self) -> u64 {
        self.total_time
    }

    // how many actions in the queue
    pub fn get_action_count(&self) -> usize {
        self.actions.len()
    }

    // add an action into the queue
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
    // add some actions into the queue
    pub fn add_actions(&mut self, actions: &mut Vec<Action>) {
        self.actions.append(actions);
    }

    // play all actions in action queue.
    pub fn play_actions(&mut self) {
        let mut p = 0;
        while p < self.actions.len() {
            match self.actions[p] {
                Action::MouseMove { delay, target } => {
                    set_timeout(|| {
                        self.instance.mouse_move_to(target[0], target[1]);
                    }, delay);
                }
                Action::MouseLeft { delay } => {
                    set_timeout(|| {
                        self.instance.mouse_click(MouseButton::Left);
                    }, delay);
                }
                Action::MouseRight { delay } => {
                    set_timeout(|| {
                        self.instance.mouse_click(MouseButton::Right);
                    }, delay);
                }
                Action::KeyClick { delay, key } => {
                    set_timeout(|| {
                        self.instance.key_click(key);
                    }, delay);
                }
                Action::KeyPress { delay, key, duration } => {
                    set_timeout(|| {
                        self.instance.key_down(key);

                        set_timeout(|| {
                            self.instance.key_up(key);
                        }, duration);
                    }, delay);
                }
                // _ => {
                //     panic!("invalid action");
                // }
            };

            p += 1;
        }
    }
}
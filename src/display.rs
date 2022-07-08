use std::sync::{Arc, Mutex};
use enigo::{Enigo, KeyboardControllable, MouseControllable};
use crate::{KeyboardMapper, MouseEv, KeyboardAction, KeyboardEv, utils::set_timeout, MouseAction, MouseMapper};

// region keyboard event player
pub struct KeyboardPlayer {
    /// simulator
    instance: Enigo,
    /// if is playing
    playing: Arc<Mutex<bool>>,
    /// duration of the action
    duration: u64,
    /// events in the action
    ev_queue: Vec<KeyboardEv>,
}

impl KeyboardPlayer {
    pub fn new() -> KeyboardPlayer {
        KeyboardPlayer {
            instance: Enigo::new(),
            playing: Arc::new(Mutex::new(false)),
            duration: 0,
            ev_queue: vec![],
        }
    }

    /// load an action record to play later.
    pub fn load(&mut self, action: KeyboardAction) -> Result<(), ()> {
        return if *self.playing.lock().unwrap() {
            Err(())
        } else {
            self.ev_queue = action.evs;
            self.duration = action.till;
            Ok(())
        };
    }

    /// auto-play keyboard event using simulator.
    pub fn do_play(&mut self) -> Result<(), ()> {
        return if *self.playing.lock().unwrap() {
            Err(())
        } else {
            *self.playing.lock().unwrap() = true;
            let mut last_act_time = 0;
            if self.ev_queue.len() > 0 && self.duration > 0 {
                for ev in self.ev_queue.iter() {
                    if ev.timestamp <= last_act_time {
                        match (KeyboardMapper::dq_to_enigo(ev.code), ev.press) {
                            (Some(key), is_press) => {
                                if is_press {
                                    self.instance.key_down(key);
                                } else {
                                    self.instance.key_up(key);
                                }
                            }
                            (None, _) => ()
                        }
                    } else {
                        set_timeout(|| {
                            match (KeyboardMapper::dq_to_enigo(ev.code), ev.press) {
                                (Some(key), is_press) => {
                                    if is_press {
                                        self.instance.key_down(key);
                                    } else {
                                        self.instance.key_up(key);
                                    }
                                }
                                (None, _) => ()
                            }
                        }, ev.timestamp - last_act_time);
                    }

                    last_act_time = ev.timestamp;
                }
            }
            *self.playing.lock().unwrap() = false;
            Ok(())
        };
    }

    pub fn get_record(&self) -> Vec<KeyboardEv> {
        self.ev_queue.clone()
    }

    pub fn get_duration(&self) -> u64 {
        self.duration
    }
}
// endregion

// region mouse event player
pub struct MousePlayer {
    /// simulator
    instance: Enigo,
    /// if is playing
    playing: Arc<Mutex<bool>>,
    /// duration of the action
    duration: u64,
    /// events in the action
    ev_queue: Vec<MouseEv>,
}

impl MousePlayer {
    pub fn new() -> MousePlayer {
        MousePlayer {
            instance: Enigo::new(),
            playing: Arc::new(Mutex::new(false)),
            duration: 0,
            ev_queue: vec![],
        }
    }

    /// load an action record to play later.
    pub fn load(&mut self, action: MouseAction) -> Result<(), ()> {
        return if *self.playing.lock().unwrap() {
            Err(())
        } else {
            self.ev_queue = action.evs;
            self.duration = action.till;
            Ok(())
        };
    }

    /// auto-play keyboard event using simulator.
    pub fn do_play(&mut self) -> Result<(), ()> {
        return if *self.playing.lock().unwrap() {
            Err(())
        } else {
            *self.playing.lock().unwrap() = true;
            let mut last_act_time = 0;
            if self.ev_queue.len() > 0 && self.duration > 0 {
                for ev in self.ev_queue.iter() {
                    if ev.timestamp <= last_act_time {
                        match MouseMapper::parse_ev_name(ev.ev_name) {
                            (enigo_button, true) => {
                                let (x, y) = ev.position;
                                self.instance.mouse_move_to(x, y);
                                self.instance.mouse_down(enigo_button);
                            }
                            (enigo_button, false) => {
                                let (x, y) = ev.position;
                                self.instance.mouse_move_to(x, y);
                                self.instance.mouse_up(enigo_button);
                            }
                        }
                    } else {
                        set_timeout(|| {
                            match MouseMapper::parse_ev_name(ev.ev_name) {
                                (enigo_button, true) => {
                                    let (x, y) = ev.position;
                                    self.instance.mouse_move_to(x, y);
                                    self.instance.mouse_down(enigo_button);
                                }
                                (enigo_button, false) => {
                                    let (x, y) = ev.position;
                                    self.instance.mouse_move_to(x, y);
                                    self.instance.mouse_up(enigo_button);
                                }
                            }
                        }, ev.timestamp - last_act_time);
                    }

                    last_act_time = ev.timestamp;
                }
            }
            *self.playing.lock().unwrap() = false;
            Ok(())
        };
    }
}
// endregion

// region unit test 此处使用了覆盖率测试, 确保所有的映射都是有效的
#[cfg(test)]
mod test {
    use super::*;
    use device_query::Keycode;
    use enigo::Key;
    use crate::MouseEventName;

    /// **pass** 0-9 a-z
    #[test]
    fn enigo_key_layout() {
        let mut en = enigo::Enigo::new();
        set_timeout(|| {
            en.key_click(Key::Layout('0'));
            en.key_click(Key::Layout('1'));
            en.key_click(Key::Layout('2'));
            en.key_click(Key::Layout('3'));
            en.key_click(Key::Layout('4'));
            en.key_click(Key::Layout('5'));
            en.key_click(Key::Layout('6'));
            en.key_click(Key::Layout('7'));
            en.key_click(Key::Layout('8'));
            en.key_click(Key::Layout('9'));
            en.key_click(Key::Layout('a'));
            en.key_click(Key::Layout('b'));
            en.key_click(Key::Layout('c'));
            en.key_click(Key::Layout('d'));
            en.key_click(Key::Layout('e'));
            en.key_click(Key::Layout('f'));
            en.key_click(Key::Layout('g'));
            en.key_click(Key::Layout('h'));
            en.key_click(Key::Layout('i'));
            en.key_click(Key::Layout('j'));
            en.key_click(Key::Layout('k'));
            en.key_click(Key::Layout('l'));
            en.key_click(Key::Layout('m'));
            en.key_click(Key::Layout('n'));
            en.key_click(Key::Layout('o'));
            en.key_click(Key::Layout('p'));
            en.key_click(Key::Layout('q'));
            en.key_click(Key::Layout('r'));
            en.key_click(Key::Layout('s'));
            en.key_click(Key::Layout('t'));
            en.key_click(Key::Layout('u'));
            en.key_click(Key::Layout('v'));
            en.key_click(Key::Layout('w'));
            en.key_click(Key::Layout('x'));
            en.key_click(Key::Layout('y'));
            en.key_click(Key::Layout('z'));
        }, 3000);
    }

    /// **pass** `-=[],.;'/\
    #[test]
    fn enigo_key_misc() {
        let mut en = enigo::Enigo::new();
        set_timeout(|| {
            en.key_click(Key::Layout('`'));
            en.key_click(Key::Layout('-'));
            en.key_click(Key::Layout('='));
            en.key_click(Key::Layout('['));
            en.key_click(Key::Layout(']'));
            en.key_click(Key::Layout(','));
            en.key_click(Key::Layout('.'));
            en.key_click(Key::Layout(';'));
            en.key_click(Key::Layout('\''));
            en.key_click(Key::Layout('/'));
            en.key_click(Key::Layout('\\'));
        }, 3000);
    }

    /// **pass** F1-F12 etc.
    #[test]
    fn enigo_key_invisible() {
        let mut en = enigo::Enigo::new();
        set_timeout(|| {
            en.key_click(Key::F1);
            en.key_click(Key::F2);
            en.key_click(Key::F3);
            en.key_click(Key::F4);
            en.key_click(Key::F5);
            en.key_click(Key::F6);
            en.key_click(Key::F7);
            en.key_click(Key::F8);
            en.key_click(Key::F9);
            en.key_click(Key::F10);
            en.key_click(Key::F11);
            en.key_click(Key::F12);

            en.key_click(Key::Escape);
            en.key_click(Key::Tab);
            en.key_click(Key::CapsLock);
            en.key_click(Key::Shift);
            en.key_click(Key::Control);
            en.key_click(Key::Alt);
            en.key_click(Key::Space);
            en.key_click(Key::UpArrow);
            en.key_click(Key::RightArrow);
            en.key_click(Key::DownArrow);
            en.key_click(Key::LeftArrow);
            en.key_click(Key::Return);
            en.key_click(Key::Backspace);
            en.key_click(Key::Delete);
            en.key_click(Key::Home);
            en.key_click(Key::PageUp);
            en.key_click(Key::PageDown);
            en.key_click(Key::End);
        }, 3000);
    }

    /// **pass**
    #[test]
    fn display_keyboard() {
        // mock action
        let mock_action = KeyboardAction {
            evs: vec![
                KeyboardEv {
                    code: Keycode::H,
                    press: true,
                    timestamp: 500,
                },
                KeyboardEv {
                    code: Keycode::E,
                    press: true,
                    timestamp: 1500,
                },
                KeyboardEv {
                    code: Keycode::L,
                    press: true,
                    timestamp: 2000,
                },
                KeyboardEv {
                    code: Keycode::L,
                    press: true,
                    timestamp: 2500,
                },
                KeyboardEv {
                    code: Keycode::O,
                    press: true,
                    timestamp: 3000,
                },
                KeyboardEv {
                    code: Keycode::Space,
                    press: true,
                    timestamp: 3500,
                },
                KeyboardEv {
                    code: Keycode::W,
                    press: true,
                    timestamp: 4000,
                },
                KeyboardEv {
                    code: Keycode::O,
                    press: true,
                    timestamp: 4500,
                },
                KeyboardEv {
                    code: Keycode::R,
                    press: true,
                    timestamp: 5000,
                },
                KeyboardEv {
                    code: Keycode::L,
                    press: true,
                    timestamp: 5500,
                },
                KeyboardEv {
                    code: Keycode::D,
                    press: true,
                    timestamp: 6000,
                },
                KeyboardEv {
                    code: Keycode::LShift,
                    press: true,
                    timestamp: 6500,
                },
                KeyboardEv {
                    code: Keycode::Numpad1,
                    press: true,
                    timestamp: 6600,
                },
                KeyboardEv {
                    code: Keycode::LShift,
                    press: false,
                    timestamp: 6700,
                },
            ],
            till: 6700,
        };

        // simulate
        let mut player = KeyboardPlayer::new();
        match player.load(mock_action) {
            Ok(_) => {
                set_timeout(|| {
                    match player.do_play() {
                        Ok(_) => {
                            println!("done.")
                        }
                        Err(_) => {
                            println!("failed.")
                        }
                    }
                }, 3_000)
            }
            Err(_) => println!("error when load actions.")
        }
    }

    /// **pass**
    #[test]
    fn display_mouse() {
        // mock action
        let mock_action = MouseAction {
            evs: vec![
                // MouseEv {
                //     ev_name: MouseEventName::LeftDown,
                //     position: (505, 1246),
                //     timestamp: 160,
                // },
                // MouseEv {
                //     ev_name: MouseEventName::LeftUp,
                //     position: (405, 1246),
                //     timestamp: 224,
                // },
                // MouseEv {
                //     ev_name: MouseEventName::RightDown,
                //     position: (75, 1208),
                //     timestamp: 1200,
                // },
                // MouseEv {
                //     ev_name: MouseEventName::RightUp,
                //     position: (75, 1208),
                //     timestamp: 1280,
                // },
                MouseEv {
                    ev_name: MouseEventName::LeftDown,
                    position: (123, 27),
                    timestamp: 2251,
                },
                MouseEv {
                    ev_name: MouseEventName::LeftUp,
                    position: (218, 345),
                    timestamp: 3795,
                },
            ],
            till: 3795,
        };

        // simulate
        let mut player = MousePlayer::new();
        match player.load(mock_action) {
            Ok(_) => {
                set_timeout(|| {
                    match player.do_play() {
                        Ok(_) => {
                            println!("done.");
                        }
                        Err(_) => {
                            println!("failed.");
                        }
                    }
                }, 3000);
            }
            Err(_) => ()
        }
    }
}
// endregion
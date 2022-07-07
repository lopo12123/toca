use enigo::{Enigo, KeyboardControllable};
use crate::{
    mapper::KeyboardMapper,
    record::{KeyboardAction, KeyboardEv},
    set_timeout,
};

// region keyboard event player
pub struct KeyboardPlayer {
    /// simulator
    instance: Enigo,
    /// duration of the action
    duration: u64,
    /// events in the action
    ev_queue: Vec<KeyboardEv>,
}

impl KeyboardPlayer {
    pub fn new() -> KeyboardPlayer {
        KeyboardPlayer {
            instance: Enigo::new(),
            duration: 0,
            ev_queue: vec![],
        }
    }

    /// load an action record to play later.
    pub fn load(&mut self, action: KeyboardAction) {
        self.ev_queue = action.evs;
        self.duration = action.till;
    }

    /// auto-play keyboard event using simulator.
    pub fn do_play(&mut self) {
        let mut last_act_time = 0;
        if self.ev_queue.len() > 0 && self.duration > 0 {
            for ev in self.ev_queue.iter() {
                set_timeout(|| {
                    match KeyboardMapper::dq_to_enigo(ev.code) {
                        Some(key) => {
                            println!("going to press: {:?}", key);
                            self.instance.key_click(key);
                        }
                        None => ()
                    }
                }, ev.timestamp - last_act_time);
                last_act_time = ev.timestamp;
            }
        }
    }

    pub fn get_record(&self) -> Vec<KeyboardEv> {
        self.ev_queue.clone()
    }

    pub fn get_duration(&self) -> u64 {
        self.duration
    }
}
// endregion

// region unit test
#[cfg(test)]
mod test {
    use super::*;
    use enigo::Key;

    #[test]
    fn enigo_key_visible() {
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

            en.key_click(Key::Meta);
        }, 6000);
    }
}

// endregion
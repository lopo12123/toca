use std::sync::{Arc, Mutex};
use std::time::Instant;
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};
use serde_json::{from_str as json_parse, to_string as json_stringify, Result as SerdeResult};
use crate::{KeyboardMapper};

// region keyboard event recorder
/// single record of keyboard event
#[derive(Copy, Clone)]
pub struct KeyboardEv {
    /// key code
    pub code: Keycode,
    /// press or release
    pub press: bool,
    /// timestamp from the start
    pub timestamp: u64,
}

/// the result of `KeyboardRecorder.do_record`
pub struct KeyboardAction {
    pub evs: Vec<KeyboardEv>,
    pub till: u64,
}

#[derive(Deserialize, Serialize)]
pub struct KeyboardEvSerializable {
    /// key code
    pub code: String,
    /// press or release
    pub press: bool,
    /// timestamp from the start
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize)]
pub struct KeyboardActionSerializable {
    pub evs: Vec<KeyboardEvSerializable>,
    pub till: u64,
}

impl KeyboardAction {
    pub fn from_string(string_source: &str) -> Result<KeyboardAction, ()> {
        let action_string_code: SerdeResult<KeyboardActionSerializable> = json_parse(string_source);
        match action_string_code {
            Ok(_action) => {
                let mut action = KeyboardAction {
                    evs: vec![],
                    till: _action.till,
                };

                for ev in _action.evs.iter() {
                    match KeyboardMapper::front_to_dq(&ev.code) {
                        Some(code) => {
                            action.evs.push(KeyboardEv {
                                code,
                                press: ev.press,
                                timestamp: ev.timestamp,
                            })
                        }
                        None => ()
                    }
                }

                Ok(action)
            }
            Err(_) => Err(())
        }
    }

    pub fn to_string(&self) -> Result<String, ()> {
        let mut action_string_code = KeyboardActionSerializable {
            evs: vec![],
            till: self.till,
        };

        for ev in self.evs.iter() {
            match KeyboardMapper::dq_to_front(ev.code) {
                Some(code) => {
                    action_string_code.evs.push(KeyboardEvSerializable {
                        code: String::from(code),
                        press: ev.press,
                        timestamp: ev.timestamp,
                    })
                }
                None => ()
            }
        }

        match json_stringify(&action_string_code) {
            Ok(s) => Ok(s),
            Err(_) => Err(())
        }
    }
}

pub struct KeyboardRecorder {
    /// stop signal
    recording: Arc<Mutex<bool>>,
    /// Here, we've wrapped your vector in a Arc<Mutex<T>> so we can
    /// write to it inside our closure.
    ev_queue: Arc<Mutex<Vec<KeyboardEv>>>,
}

impl KeyboardRecorder {
    pub fn new() -> KeyboardRecorder {
        KeyboardRecorder {
            recording: Arc::new(Mutex::new(false)),
            ev_queue: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_record(&self) -> Vec<KeyboardEv> {
        (*self.ev_queue.lock().unwrap()).clone()
    }

    /// Doing record work in main thread.
    /// Anyway, the listener(s) is working in separator thread(s).
    /// But the guard(s) of listener(s) can only live in the scope of this function.
    /// So this call need to be 'block' until you press the key representing 'stop_code'.
    pub fn do_record(&mut self, stop_code: Keycode) -> KeyboardAction {
        // start recording: clear records and set the signal
        *self.ev_queue.lock().unwrap() = vec![];
        *self.recording.lock().unwrap() = true;

        // instance
        let device_state = DeviceState::new();
        // record start time as zero
        let timeline = Instant::now();

        // We make a clone of the Arc<Mutex<T>> here so we can move it
        // into our closure without moving self into the closure.
        let ev_queue_down = Arc::clone(&self.ev_queue);
        let ev_queue_up = Arc::clone(&self.ev_queue);
        let recording = Arc::clone(&self.recording);

        // Note the `move` here on the closure.
        let _guard_down = device_state.on_key_down(move |key| {
            // if the stop key is pressed, stop record.
            if key == &stop_code {
                let mut recording = recording.lock().unwrap();
                *recording = false;
                return;
            }

            // We lock the mutex here and write to it.
            let mut ev_queue_down = ev_queue_down.lock().unwrap();
            ev_queue_down.push(KeyboardEv {
                code: key.clone(),
                press: true,
                timestamp: timeline.elapsed().as_millis() as u64,
            })
        });

        // Note the `move` here on the closure.
        let _guard_up = device_state.on_key_up(move |key| {
            // We lock the mutex here and write to it.
            let mut ev_queue_up = ev_queue_up.lock().unwrap();
            ev_queue_up.push(KeyboardEv {
                code: key.clone(),
                press: false,
                timestamp: timeline.elapsed().as_millis() as u64,
            })
        });

        // a block loop till the stop key is pressed.
        loop {
            if !*self.recording.lock().unwrap() {
                // jump out of the loop, the guard(s) will `drop` then.
                return KeyboardAction {
                    evs: (*self.ev_queue.lock().unwrap()).clone(),
                    till: timeline.elapsed().as_millis() as u64,
                };
            }
        }
    }

    // Do record work in separator thread(s).
    // The async version of `do_record`, but it is unsafe somehow, logically.
    // I use a signal to make sure the main thread lives longer then the record thread.
    // But that does not means the life of `do_record_async` is longer then record thread (in face, this call will return immediately)
    // For example, maybe there is a `while signal {}` loop after this call, and the signal will be changed to `false` inside the record thread.
    // Therefore i can say the main thread is always lives longer then the record thread.
    // pub fn do_record_async(&mut self, stop_code: Keycode) {
    //
    // }
}
// endregion

// region mouse event recorder
/// single record of mouse event
#[derive(Copy, Clone, Debug)]
pub enum MouseEventName {
    // 1
    LeftDown,
    // 2
    LeftUp,
    // 3
    RightDown,
    // 4
    RightUp,
    // 5
    MidDown,
    // 6
    MidUp,
}

#[derive(Copy, Clone)]
pub struct MouseEv {
    /// mouse event name
    pub ev_name: MouseEventName,
    /// position
    pub position: (i32, i32),
    /// timestamp from the start
    pub timestamp: u64,
}

/// the result of `MouseRecorder.do_record`
pub struct MouseAction {
    pub evs: Vec<MouseEv>,
    pub till: u64,
}

#[derive(Deserialize, Serialize)]
pub struct MouseEvSerializable {
    /// mouse event name
    /// 1: LeftDown
    /// 2: LeftUp
    /// 3: RightDown
    /// 4: RightUp
    /// 5: MidDown
    /// 6: MidUp
    /// _: invalid
    pub ev_name: usize,
    /// position
    pub position: [i32; 2],
    /// timestamp from the start
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize)]
pub struct MouseActionSerializable {
    pub evs: Vec<MouseEvSerializable>,
    pub till: u64,
}

impl MouseAction {
    pub fn from_string(string_source: &str) -> Result<MouseAction, ()> {
        let action_string_code: SerdeResult<MouseActionSerializable> = json_parse(string_source);
        match action_string_code {
            Ok(_action) => {
                let mut action = MouseAction {
                    evs: vec![],
                    till: _action.till,
                };

                for ev in _action.evs.iter() {
                    match ev.ev_name {
                        1 => {
                            action.evs.push(MouseEv {
                                ev_name: MouseEventName::LeftDown,
                                position: (ev.position[0], ev.position[1]),
                                timestamp: ev.timestamp,
                            })
                        }
                        2 => {
                            action.evs.push(MouseEv {
                                ev_name: MouseEventName::LeftUp,
                                position: (ev.position[0], ev.position[1]),
                                timestamp: ev.timestamp,
                            })
                        }
                        3 => {
                            action.evs.push(MouseEv {
                                ev_name: MouseEventName::RightDown,
                                position: (ev.position[0], ev.position[1]),
                                timestamp: ev.timestamp,
                            })
                        }
                        4 => {
                            action.evs.push(MouseEv {
                                ev_name: MouseEventName::RightUp,
                                position: (ev.position[0], ev.position[1]),
                                timestamp: ev.timestamp,
                            })
                        }
                        5 => {
                            action.evs.push(MouseEv {
                                ev_name: MouseEventName::MidDown,
                                position: (ev.position[0], ev.position[1]),
                                timestamp: ev.timestamp,
                            })
                        }
                        6 => {
                            action.evs.push(MouseEv {
                                ev_name: MouseEventName::MidUp,
                                position: (ev.position[0], ev.position[1]),
                                timestamp: ev.timestamp,
                            })
                        }
                        _ => ()
                    }
                }

                Ok(action)
            }
            Err(_) => Err(())
        }
    }

    pub fn to_string(&self) -> Result<String, ()> {
        let mut action_string_code = MouseActionSerializable {
            evs: vec![],
            till: self.till,
        };

        for ev in self.evs.iter() {
            match ev.ev_name {
                MouseEventName::LeftDown => {
                    action_string_code.evs.push(MouseEvSerializable {
                        ev_name: 1,
                        position: [ev.position.0, ev.position.1],
                        timestamp: ev.timestamp,
                    })
                }
                MouseEventName::LeftUp => {
                    action_string_code.evs.push(MouseEvSerializable {
                        ev_name: 2,
                        position: [ev.position.0, ev.position.1],
                        timestamp: ev.timestamp,
                    })
                }
                MouseEventName::RightDown => {
                    action_string_code.evs.push(MouseEvSerializable {
                        ev_name: 3,
                        position: [ev.position.0, ev.position.1],
                        timestamp: ev.timestamp,
                    })
                }
                MouseEventName::RightUp => {
                    action_string_code.evs.push(MouseEvSerializable {
                        ev_name: 4,
                        position: [ev.position.0, ev.position.1],
                        timestamp: ev.timestamp,
                    })
                }
                MouseEventName::MidDown => {
                    action_string_code.evs.push(MouseEvSerializable {
                        ev_name: 5,
                        position: [ev.position.0, ev.position.1],
                        timestamp: ev.timestamp,
                    })
                }
                MouseEventName::MidUp => {
                    action_string_code.evs.push(MouseEvSerializable {
                        ev_name: 6,
                        position: [ev.position.0, ev.position.1],
                        timestamp: ev.timestamp,
                    })
                }
            }
        }

        match json_stringify(&action_string_code) {
            Ok(s) => Ok(s),
            Err(_) => Err(())
        }
    }
}

pub struct MouseRecorder {
    /// stop signal
    recording: Arc<Mutex<bool>>,
    /// Here, we've wrapped your vector in a Arc<Mutex<>> so we can
    /// write to it inside our closure.
    ev_queue: Arc<Mutex<Vec<MouseEv>>>,
}

impl MouseRecorder {
    pub fn new() -> MouseRecorder {
        MouseRecorder {
            recording: Arc::new(Mutex::new(false)),
            ev_queue: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_record(&self) -> Vec<MouseEv> {
        (*self.ev_queue.lock().unwrap()).clone()
    }

    /// Doing record work in main thread.
    /// Anyway, the listener(s) is working in separator thread(s).
    /// But the guard(s) of listener(s) can only live in the scope of this function.
    /// So this call need to be 'block' until you press the key representing 'stop_code'.
    pub fn do_record(&mut self, stop_code: Keycode) -> MouseAction {
        // start recording: clear records and set the signal
        *self.ev_queue.lock().unwrap() = vec![];
        *self.recording.lock().unwrap() = true;

        // instance
        let device_state = DeviceState::new();
        // record start time as zero
        let timeline = Instant::now();

        // region an extra listener to watch the stop signal
        let recording = Arc::clone(&self.recording);

        let _guard_stop = device_state.on_key_down(move |key| {
            if key == &stop_code {
                let mut recording = recording.lock().unwrap();
                *recording = false;
            }
        });
        // endregion

        // region mouse down
        let ev_queue_down = Arc::clone(&self.ev_queue);
        let device_state_down = DeviceState::new();
        let _guard_down = device_state.on_mouse_down(move |btn| {
            match *btn {
                1 => {
                    let mut ev_queue_down = ev_queue_down.lock().unwrap();
                    ev_queue_down.push(MouseEv {
                        ev_name: MouseEventName::LeftDown,
                        position: device_state_down.get_mouse().coords,
                        timestamp: timeline.elapsed().as_millis() as u64,
                    });
                }
                2 => {
                    let mut ev_queue_down = ev_queue_down.lock().unwrap();
                    ev_queue_down.push(MouseEv {
                        ev_name: MouseEventName::RightDown,
                        position: device_state_down.get_mouse().coords,
                        timestamp: timeline.elapsed().as_millis() as u64,
                    });
                }
                3 => {
                    let mut ev_queue_down = ev_queue_down.lock().unwrap();
                    ev_queue_down.push(MouseEv {
                        ev_name: MouseEventName::MidDown,
                        position: device_state_down.get_mouse().coords,
                        timestamp: timeline.elapsed().as_millis() as u64,
                    });
                }
                _ => ()  // ignore other button event
            }
        });
        // endregion

        // region mouse up
        let ev_queue_up = Arc::clone(&self.ev_queue);
        let device_state_up = DeviceState::new();
        let _guard_up = device_state.on_mouse_up(move |btn| {
            match *btn {
                1 => {
                    let mut ev_queue_up = ev_queue_up.lock().unwrap();
                    ev_queue_up.push(MouseEv {
                        ev_name: MouseEventName::LeftUp,
                        position: device_state_up.get_mouse().coords,
                        timestamp: timeline.elapsed().as_millis() as u64,
                    });
                }
                2 => {
                    let mut ev_queue_up = ev_queue_up.lock().unwrap();
                    ev_queue_up.push(MouseEv {
                        ev_name: MouseEventName::RightUp,
                        position: device_state_up.get_mouse().coords,
                        timestamp: timeline.elapsed().as_millis() as u64,
                    });
                }
                3 => {
                    let mut ev_queue_up = ev_queue_up.lock().unwrap();
                    ev_queue_up.push(MouseEv {
                        ev_name: MouseEventName::MidUp,
                        position: device_state_up.get_mouse().coords,
                        timestamp: timeline.elapsed().as_millis() as u64,
                    });
                }
                _ => ()  // ignore other button event
            }
        });
        // endregion

        // a block loop till the stop key is pressed.
        loop {
            if !*self.recording.lock().unwrap() {
                // jump out of the loop, the guard(s) will `drop` then.
                return MouseAction {
                    evs: (*self.ev_queue.lock().unwrap()).clone(),
                    till: timeline.elapsed().as_millis() as u64,
                };
            }
        }
    }
}
// endregion

// region unit test
#[cfg(test)]
mod test {
    use super::*;

    /// 键盘行为录制测试 - 无断言, 需要自行判断输出是否正确
    #[test]
    fn keyboard_recorder() {
        let mut recorder = KeyboardRecorder::new();

        println!("record start. (press any key to record, press ESC to stop.)");
        let action = recorder.do_record(Keycode::Escape);
        println!("record stop. duration: {}ms", action.till);

        for ev in action.evs {
            println!("[{}ms]: {} {}", ev.timestamp, (if ev.press { "Press" } else { "Release" }), ev.code);
        }
    }

    /// 键盘行为录制测试 - 打印出 JSON 字符串
    #[test]
    fn keyboard_recorder_to_string() {
        let mut recorder = KeyboardRecorder::new();

        println!("record start. (press any key to record, press ESC to stop.)");
        let action = recorder.do_record(Keycode::Escape);
        println!("record stop. duration: {}ms", action.till);

        println!("action in string: \n{:#?}", action.to_string().unwrap());
    }

    /// 鼠标行为录制测试 - 无断言, 需要自行判断输出是否正确
    #[test]
    fn mouse_recorder() {
        let mut recorder = MouseRecorder::new();

        println!("record start. (press ESC to stop.)");
        let action = recorder.do_record(Keycode::Escape);
        println!("record stop. last: {}ms", action.till);

        for ev in action.evs {
            println!("[{}ms]: {:?} at {:?}", ev.timestamp, ev.ev_name, ev.position);
        }
    }

    /// 鼠标行为录制测试 - 打印出 JSON 字符串
    #[test]
    fn mouse_recorder_to_string() {
        let mut recorder = MouseRecorder::new();

        println!("record start. (press ESC to stop.)");
        let action = recorder.do_record(Keycode::Escape);
        println!("record stop. last: {}ms", action.till);
        
        println!("action in string: \n{:#?}", action.to_string().unwrap());
    }
}
// endregion
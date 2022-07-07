use std::sync::{Arc, Mutex};
use std::time::Instant;
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};

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
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    MidDown,
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
    fn keyboard_recorder_test() {
        let mut recorder = KeyboardRecorder::new();

        println!("record start. (press any key to record, press ESC to stop.)");
        let action = recorder.do_record(Keycode::Escape);
        println!("record stop. duration: {}ms", action.till);

        for ev in action.evs {
            println!("[{}ms]: {} {}", ev.timestamp, (if ev.press { "Press" } else { "Release" }), ev.code);
        }
    }

    /// 鼠标行为录制测试 - 无断言, 需要自行判断输出是否正确
    #[test]
    fn mouse_recorder_test() {
        let mut recorder = MouseRecorder::new();

        println!("record start. (press ESC to stop.)");
        let action = recorder.do_record(Keycode::Escape);
        println!("record stop. last: {}ms", action.till);

        for ev in action.evs {
            println!("[{}ms]: {:?} at {:?}", ev.timestamp, ev.ev_name, ev.position);
        }
    }
}
// endregion
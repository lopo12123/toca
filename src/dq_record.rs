use std::sync::{Arc, Mutex};
use std::time::Instant;
use device_query::{DeviceEvents, DeviceState, Keycode};
use crate::set_timeout;

// region keyboard event
#[derive(Copy, Clone)]
pub struct KeyboardEv {
    pub code: Keycode,
    pub press: bool,
    pub timestamp: u128,
}

pub struct KeyboardRecorder {
    recording: Arc<Mutex<bool>>,
    duration: u128,
    // Here, we've wrapped your vector in a Arc<Mutex<>> so we can
    // write to it inside our closure.
    ev_queue: Arc<Mutex<Vec<KeyboardEv>>>,
}

#[allow(unused)]
impl KeyboardRecorder {
    pub fn new() -> Self {
        KeyboardRecorder {
            recording: Arc::new(Mutex::new(false)),
            duration: 0,
            ev_queue: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn start_record(&mut self, stop_code: Keycode) {
        // start recording
        *self.recording.lock().unwrap() = true;

        // instance
        let device_state = DeviceState::new();
        // record start time as zero
        let mut timeline = Instant::now();

        // We make a clone of the Arc<Mutex<T>> here so we can move it
        // into our closure without moving self into the closure.
        let ev_queue = Arc::clone(&self.ev_queue);
        let recording = Arc::clone(&self.recording);

        // Note the `move` here on the closure.
        let _guard = device_state.on_key_down(move |key| {
            // if the stop key is pressed, stop record.
            if key == &stop_code {
                let mut recording = recording.lock().unwrap();
                *recording = false;
                return;
            }

            // We lock the mutex here and write to it.
            let mut ev_queue = ev_queue.lock().unwrap();
            ev_queue.push(KeyboardEv {
                code: key.clone(),
                press: true,
                timestamp: timeline.elapsed().as_millis(),
            })
        });

        // Note the `move` here on the closure.
        // let _guard = device_state.on_key_up(|key| {
        //     println!("up: {:#?}", key);
        // });

        loop {
            if !*self.recording.lock().unwrap() {
                return;
            }
        }
    }

    pub fn get_record(&mut self) -> Vec<KeyboardEv> {
        (*self.ev_queue.lock().unwrap()).clone()
    }
}
// endregion
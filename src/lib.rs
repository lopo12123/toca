use std::thread::sleep;
use std::time::Duration;

/// javascript-like `setTimeout` but sync
pub fn set_timeout<T>(mut callback: T, ms: u64)
    where T: FnMut() -> () {
    sleep(Duration::from_millis(ms));
    callback();
}

pub use device_query::Keycode;

mod mapper;
mod global_store;

pub mod record;
pub mod display;
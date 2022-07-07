#[macro_use]
extern crate lazy_static;

use std::thread::sleep;
use std::time::Duration;

/// javascript-like `setTimeout` but sync
pub fn set_timeout<T>(mut callback: T, ms: u64)
    where T: FnMut() -> () {
    sleep(Duration::from_millis(ms));
    callback();
}

mod global_store;

pub mod mapper;
pub mod record;
pub mod display;

pub use device_query::Keycode;
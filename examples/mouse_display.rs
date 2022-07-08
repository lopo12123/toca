extern crate toca;

use toca::{MouseAction, MouseEv, MouseEventName, MousePlayer, set_timeout};

fn main() {
    // mock action
    let mock_action = MouseAction {
        evs: vec![
            MouseEv {
                ev_name: MouseEventName::LeftDown,
                position: (505, 1246),
                timestamp: 160,
            },
            MouseEv {
                ev_name: MouseEventName::LeftUp,
                position: (405, 1246),
                timestamp: 224,
            },
            MouseEv {
                ev_name: MouseEventName::RightDown,
                position: (75, 1208),
                timestamp: 1200,
            },
            MouseEv {
                ev_name: MouseEventName::RightUp,
                position: (75, 1208),
                timestamp: 1280,
            },
        ],
        till: 1280,
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
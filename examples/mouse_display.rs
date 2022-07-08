extern crate toca;

use toca::{MouseAction, MouseEv, MouseEventName, MousePlayer, set_timeout};

fn from_string() {
    // mock action
    let mock_action = MouseAction::from_string("{\"evs\":[{\"ev_name\":1,\"position\":[1953,367],\"timestamp\":1474},{\"ev_name\":2,\"position\":[1953,367],\"timestamp\":1562},{\"ev_name\":3,\"position\":[2194,281],\"timestamp\":2018},{\"ev_name\":4,\"position\":[2195,280],\"timestamp\":2090},{\"ev_name\":1,\"position\":[2263,337],\"timestamp\":2618},{\"ev_name\":2,\"position\":[2263,337],\"timestamp\":2698},{\"ev_name\":3,\"position\":[2130,389],\"timestamp\":3106},{\"ev_name\":4,\"position\":[2130,389],\"timestamp\":3194},{\"ev_name\":1,\"position\":[2169,445],\"timestamp\":3732},{\"ev_name\":2,\"position\":[2169,445],\"timestamp\":3810},{\"ev_name\":5,\"position\":[2099,365],\"timestamp\":4370},{\"ev_name\":6,\"position\":[2099,365],\"timestamp\":4458}],\"till\":5679}").unwrap();

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

fn from_struct() {
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


fn main() {
    // from_string();
    from_struct();
}
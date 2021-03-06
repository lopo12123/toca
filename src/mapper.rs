use device_query::{Keycode as DqKey, MouseButton as DqButton};
use enigo::{Key as EnigoKey, MouseButton as EnigoButton};
use crate::MouseEventName;

// region keyboard mapper
pub struct KeyboardMapper {}

#[allow(unused)]
impl KeyboardMapper {
    /// `Keycode` in `device_query` => `Key` in `Enigo`
    pub fn dq_to_enigo(key_in_dq: DqKey) -> Option<EnigoKey> {
        match key_in_dq {
            // F1-F12
            DqKey::F1 => Some(EnigoKey::F1),
            DqKey::F2 => Some(EnigoKey::F2),
            DqKey::F3 => Some(EnigoKey::F3),
            DqKey::F4 => Some(EnigoKey::F4),
            DqKey::F5 => Some(EnigoKey::F5),
            DqKey::F6 => Some(EnigoKey::F6),
            DqKey::F7 => Some(EnigoKey::F7),
            DqKey::F8 => Some(EnigoKey::F8),
            DqKey::F9 => Some(EnigoKey::F9),
            DqKey::F10 => Some(EnigoKey::F10),
            DqKey::F11 => Some(EnigoKey::F11),
            DqKey::F12 => Some(EnigoKey::F12),
            // 0-9
            DqKey::Key0 => Some(EnigoKey::Layout('0')),
            DqKey::Key1 => Some(EnigoKey::Layout('1')),
            DqKey::Key2 => Some(EnigoKey::Layout('2')),
            DqKey::Key3 => Some(EnigoKey::Layout('3')),
            DqKey::Key4 => Some(EnigoKey::Layout('4')),
            DqKey::Key5 => Some(EnigoKey::Layout('5')),
            DqKey::Key6 => Some(EnigoKey::Layout('6')),
            DqKey::Key7 => Some(EnigoKey::Layout('7')),
            DqKey::Key8 => Some(EnigoKey::Layout('8')),
            DqKey::Key9 => Some(EnigoKey::Layout('9')),
            // A-Z
            DqKey::A => Some(EnigoKey::Layout('a')),
            DqKey::B => Some(EnigoKey::Layout('b')),
            DqKey::C => Some(EnigoKey::Layout('c')),
            DqKey::D => Some(EnigoKey::Layout('d')),
            DqKey::E => Some(EnigoKey::Layout('e')),
            DqKey::F => Some(EnigoKey::Layout('f')),
            DqKey::G => Some(EnigoKey::Layout('g')),
            DqKey::H => Some(EnigoKey::Layout('h')),
            DqKey::I => Some(EnigoKey::Layout('i')),
            DqKey::J => Some(EnigoKey::Layout('j')),
            DqKey::K => Some(EnigoKey::Layout('k')),
            DqKey::L => Some(EnigoKey::Layout('l')),
            DqKey::M => Some(EnigoKey::Layout('m')),
            DqKey::N => Some(EnigoKey::Layout('n')),
            DqKey::O => Some(EnigoKey::Layout('o')),
            DqKey::P => Some(EnigoKey::Layout('p')),
            DqKey::Q => Some(EnigoKey::Layout('q')),
            DqKey::R => Some(EnigoKey::Layout('r')),
            DqKey::S => Some(EnigoKey::Layout('s')),
            DqKey::T => Some(EnigoKey::Layout('t')),
            DqKey::U => Some(EnigoKey::Layout('u')),
            DqKey::V => Some(EnigoKey::Layout('v')),
            DqKey::W => Some(EnigoKey::Layout('w')),
            DqKey::X => Some(EnigoKey::Layout('x')),
            DqKey::Y => Some(EnigoKey::Layout('y')),
            DqKey::Z => Some(EnigoKey::Layout('z')),
            // from left to right, from top to bottom
            DqKey::Escape => Some(EnigoKey::Escape),
            DqKey::Tab => Some(EnigoKey::Tab),
            DqKey::CapsLock => Some(EnigoKey::CapsLock),
            DqKey::LShift | DqKey::RShift => Some(EnigoKey::Shift),
            DqKey::LControl | DqKey::RControl => Some(EnigoKey::Control),
            DqKey::LAlt | DqKey::RAlt => Some(EnigoKey::Alt),
            DqKey::Space => Some(EnigoKey::Space),
            DqKey::Up => Some(EnigoKey::UpArrow),
            DqKey::Right => Some(EnigoKey::RightArrow),
            DqKey::Down => Some(EnigoKey::DownArrow),
            DqKey::Left => Some(EnigoKey::LeftArrow),
            DqKey::Enter => Some(EnigoKey::Return),
            DqKey::Backspace => Some(EnigoKey::Backspace),
            // DqKey::Insert => None,
            DqKey::Delete => Some(EnigoKey::Delete),
            DqKey::Home => Some(EnigoKey::Home),
            DqKey::PageUp => Some(EnigoKey::PageUp),
            DqKey::PageDown => Some(EnigoKey::PageDown),
            DqKey::End => Some(EnigoKey::End),
            // belows have passed the simulate test
            DqKey::Grave => Some(EnigoKey::Layout('`')),
            DqKey::Minus | DqKey::NumpadSubtract => Some(EnigoKey::Layout('-')),
            DqKey::Equal => Some(EnigoKey::Layout('=')),
            DqKey::LeftBracket => Some(EnigoKey::Layout('[')),
            DqKey::RightBracket => Some(EnigoKey::Layout(']')),
            DqKey::Comma => Some(EnigoKey::Layout(',')),
            DqKey::Dot => Some(EnigoKey::Layout('.')),
            DqKey::Semicolon => Some(EnigoKey::Layout(';')),
            DqKey::Apostrophe => Some(EnigoKey::Layout('\'')),
            DqKey::Slash | DqKey::NumpadDivide => Some(EnigoKey::Layout('/')),
            DqKey::BackSlash => Some(EnigoKey::Layout('\\')),
            // belows have no exact target in Enigo but can also use in typing
            DqKey::Numpad0 => Some(EnigoKey::Layout('0')),
            DqKey::Numpad1 => Some(EnigoKey::Layout('1')),
            DqKey::Numpad2 => Some(EnigoKey::Layout('2')),
            DqKey::Numpad3 => Some(EnigoKey::Layout('3')),
            DqKey::Numpad4 => Some(EnigoKey::Layout('4')),
            DqKey::Numpad5 => Some(EnigoKey::Layout('5')),
            DqKey::Numpad6 => Some(EnigoKey::Layout('6')),
            DqKey::Numpad7 => Some(EnigoKey::Layout('7')),
            DqKey::Numpad8 => Some(EnigoKey::Layout('8')),
            DqKey::Numpad9 => Some(EnigoKey::Layout('9')),
            _ => None
        }
    }

    /// `Key` in `Enigo` => `Keycode` in `device_query`
    pub fn enigo_to_dq(key_in_enigo: EnigoKey) -> Option<DqKey> {
        match key_in_enigo {
            // F1-F12
            EnigoKey::F1 => Some(DqKey::F1),
            EnigoKey::F2 => Some(DqKey::F2),
            EnigoKey::F3 => Some(DqKey::F3),
            EnigoKey::F4 => Some(DqKey::F4),
            EnigoKey::F5 => Some(DqKey::F5),
            EnigoKey::F6 => Some(DqKey::F6),
            EnigoKey::F7 => Some(DqKey::F7),
            EnigoKey::F8 => Some(DqKey::F8),
            EnigoKey::F9 => Some(DqKey::F9),
            EnigoKey::F10 => Some(DqKey::F10),
            EnigoKey::F11 => Some(DqKey::F11),
            EnigoKey::F12 => Some(DqKey::F12),
            // 0-9
            EnigoKey::Layout('0') => Some(DqKey::Key0),
            EnigoKey::Layout('1') => Some(DqKey::Key1),
            EnigoKey::Layout('2') => Some(DqKey::Key2),
            EnigoKey::Layout('3') => Some(DqKey::Key3),
            EnigoKey::Layout('4') => Some(DqKey::Key4),
            EnigoKey::Layout('5') => Some(DqKey::Key5),
            EnigoKey::Layout('6') => Some(DqKey::Key6),
            EnigoKey::Layout('7') => Some(DqKey::Key7),
            EnigoKey::Layout('8') => Some(DqKey::Key8),
            EnigoKey::Layout('9') => Some(DqKey::Key9),
            // A-Z
            EnigoKey::Layout('a') => Some(DqKey::A),
            EnigoKey::Layout('b') => Some(DqKey::B),
            EnigoKey::Layout('c') => Some(DqKey::C),
            EnigoKey::Layout('d') => Some(DqKey::D),
            EnigoKey::Layout('e') => Some(DqKey::E),
            EnigoKey::Layout('f') => Some(DqKey::F),
            EnigoKey::Layout('g') => Some(DqKey::G),
            EnigoKey::Layout('h') => Some(DqKey::H),
            EnigoKey::Layout('i') => Some(DqKey::I),
            EnigoKey::Layout('j') => Some(DqKey::J),
            EnigoKey::Layout('k') => Some(DqKey::K),
            EnigoKey::Layout('l') => Some(DqKey::L),
            EnigoKey::Layout('m') => Some(DqKey::M),
            EnigoKey::Layout('n') => Some(DqKey::N),
            EnigoKey::Layout('o') => Some(DqKey::O),
            EnigoKey::Layout('p') => Some(DqKey::P),
            EnigoKey::Layout('q') => Some(DqKey::Q),
            EnigoKey::Layout('r') => Some(DqKey::R),
            EnigoKey::Layout('s') => Some(DqKey::S),
            EnigoKey::Layout('t') => Some(DqKey::T),
            EnigoKey::Layout('u') => Some(DqKey::U),
            EnigoKey::Layout('v') => Some(DqKey::V),
            EnigoKey::Layout('w') => Some(DqKey::W),
            EnigoKey::Layout('x') => Some(DqKey::X),
            EnigoKey::Layout('y') => Some(DqKey::Y),
            EnigoKey::Layout('z') => Some(DqKey::Z),
            // from left to right, from top to bottom
            EnigoKey::Escape => Some(DqKey::Escape),
            EnigoKey::Tab => Some(DqKey::Tab),
            EnigoKey::CapsLock => Some(DqKey::CapsLock),
            EnigoKey::Shift => Some(DqKey::LShift),
            EnigoKey::Control => Some(DqKey::LControl),
            EnigoKey::Alt => Some(DqKey::LAlt),
            EnigoKey::Space => Some(DqKey::Space),
            EnigoKey::UpArrow => Some(DqKey::Up),
            EnigoKey::RightArrow => Some(DqKey::Right),
            EnigoKey::DownArrow => Some(DqKey::Down),
            EnigoKey::LeftArrow => Some(DqKey::Left),
            EnigoKey::Return => Some(DqKey::Enter),
            EnigoKey::Backspace => Some(DqKey::Backspace),
            // DqKey::Insert => None,
            EnigoKey::Delete => Some(DqKey::Delete),
            EnigoKey::Home => Some(DqKey::Home),
            EnigoKey::PageUp => Some(DqKey::PageUp),
            EnigoKey::PageDown => Some(DqKey::PageDown),
            EnigoKey::End => Some(DqKey::End),
            // belows have passed the simulate test
            EnigoKey::Layout('`') => Some(DqKey::Grave),
            EnigoKey::Layout('-') => Some(DqKey::Minus),
            EnigoKey::Layout('=') => Some(DqKey::Equal),
            EnigoKey::Layout('[') => Some(DqKey::LeftBracket),
            EnigoKey::Layout(']') => Some(DqKey::RightBracket),
            EnigoKey::Layout(',') => Some(DqKey::Comma),
            EnigoKey::Layout('.') => Some(DqKey::Dot),
            EnigoKey::Layout(';') => Some(DqKey::Semicolon),
            EnigoKey::Layout('\'') => Some(DqKey::Apostrophe),
            EnigoKey::Layout('/') => Some(DqKey::Slash),
            EnigoKey::Layout('\\') => Some(DqKey::BackSlash),
            _ => None
        }
    }

    /// `Keycode` in `device_query` => `code` in frontend
    pub fn dq_to_front(key_in_dq: DqKey) -> Option<&'static str> {
        match key_in_dq {
            // F1-F12
            DqKey::F1 => Some("F1"),
            DqKey::F2 => Some("F2"),
            DqKey::F3 => Some("F3"),
            DqKey::F4 => Some("F4"),
            DqKey::F5 => Some("F5"),
            DqKey::F6 => Some("F6"),
            DqKey::F7 => Some("F7"),
            DqKey::F8 => Some("F8"),
            DqKey::F9 => Some("F9"),
            DqKey::F10 => Some("F10"),
            DqKey::F11 => Some("F11"),
            DqKey::F12 => Some("F12"),
            // 0-9
            DqKey::Key0 => Some("Digit0"),
            DqKey::Key1 => Some("Digit1"),
            DqKey::Key2 => Some("Digit2"),
            DqKey::Key3 => Some("Digit3"),
            DqKey::Key4 => Some("Digit4"),
            DqKey::Key5 => Some("Digit5"),
            DqKey::Key6 => Some("Digit6"),
            DqKey::Key7 => Some("Digit7"),
            DqKey::Key8 => Some("Digit8"),
            DqKey::Key9 => Some("Digit9"),
            // A-Z
            DqKey::A => Some("KeyA"),
            DqKey::B => Some("KeyB"),
            DqKey::C => Some("KeyC"),
            DqKey::D => Some("KeyD"),
            DqKey::E => Some("KeyE"),
            DqKey::F => Some("KeyF"),
            DqKey::G => Some("KeyG"),
            DqKey::H => Some("KeyH"),
            DqKey::I => Some("KeyI"),
            DqKey::J => Some("KeyJ"),
            DqKey::K => Some("KeyK"),
            DqKey::L => Some("KeyL"),
            DqKey::M => Some("KeyM"),
            DqKey::N => Some("KeyN"),
            DqKey::O => Some("KeyO"),
            DqKey::P => Some("KeyP"),
            DqKey::Q => Some("KeyQ"),
            DqKey::R => Some("KeyR"),
            DqKey::S => Some("KeyS"),
            DqKey::T => Some("KeyT"),
            DqKey::U => Some("KeyU"),
            DqKey::V => Some("KeyV"),
            DqKey::W => Some("KeyW"),
            DqKey::X => Some("KeyX"),
            DqKey::Y => Some("KeyY"),
            DqKey::Z => Some("KeyZ"),
            // from left to right, from top to bottom
            DqKey::Escape => Some("Escape"),
            DqKey::Tab => Some("Tab"),
            DqKey::CapsLock => Some("CapsLock"),
            DqKey::LShift => Some("ShiftLeft"),
            DqKey::RShift => Some("ShiftRight"),
            DqKey::LControl => Some("ControlLeft"),
            DqKey::RControl => Some("ControlRight"),
            DqKey::LAlt => Some("AltLeft"),
            DqKey::RAlt => Some("AltRight"),
            DqKey::Space => Some("Space"),
            DqKey::Up => Some("ArrowUp"),
            DqKey::Right => Some("ArrowRight"),
            DqKey::Down => Some("ArrowDown"),
            DqKey::Left => Some("ArrowLeft"),
            DqKey::Enter => Some("Enter"),
            DqKey::Backspace => Some("Backspace"),
            // DqKey::Insert => None,
            DqKey::Delete => Some("Delete"),
            DqKey::Home => Some("Home"),
            DqKey::PageUp => Some("PageUp"),
            DqKey::PageDown => Some("PageDown"),
            DqKey::End => Some("End"),
            // belows have passed the simulate test
            DqKey::Grave => Some("Backquote"),
            DqKey::Minus => Some("Minus"),
            DqKey::Equal => Some("Equal"),
            DqKey::LeftBracket => Some("BracketLeft"),
            DqKey::RightBracket => Some("BracketRight"),
            DqKey::Comma => Some("Comma"),
            DqKey::Dot => Some("Period"),
            DqKey::Semicolon => Some("Semicolon"),
            DqKey::Apostrophe => Some("Quote"),
            DqKey::Slash => Some("Slash"),
            DqKey::BackSlash => Some("BackSlash"),
            // belows have no exact target in Enigo but can also use in typing
            DqKey::Numpad0 => Some("Numpad0"),
            DqKey::Numpad1 => Some("Numpad1"),
            DqKey::Numpad2 => Some("Numpad2"),
            DqKey::Numpad3 => Some("Numpad3"),
            DqKey::Numpad4 => Some("Numpad4"),
            DqKey::Numpad5 => Some("Numpad5"),
            DqKey::Numpad6 => Some("Numpad6"),
            DqKey::Numpad7 => Some("Numpad7"),
            DqKey::Numpad8 => Some("Numpad8"),
            DqKey::Numpad9 => Some("Numpad9"),
            DqKey::NumpadAdd => Some("NumpadAdd"),
            DqKey::NumpadSubtract => Some("NumpadSubtract"),
            DqKey::NumpadMultiply => Some("NumpadMultiply"),
            DqKey::NumpadDivide => Some("NumpadDivide"),
            _ => None
        }
    }

    /// `code` in frontend => `Keycode` in `device_query`
    pub fn front_to_dq(code_in_front: &str) -> Option<DqKey> {
        match code_in_front {
            // F1-F12
            "F1" => Some(DqKey::F1),
            "F2" => Some(DqKey::F2),
            "F3" => Some(DqKey::F3),
            "F4" => Some(DqKey::F4),
            "F5" => Some(DqKey::F5),
            "F6" => Some(DqKey::F6),
            "F7" => Some(DqKey::F7),
            "F8" => Some(DqKey::F8),
            "F9" => Some(DqKey::F9),
            "F10" => Some(DqKey::F10),
            "F11" => Some(DqKey::F11),
            "F12" => Some(DqKey::F12),
            // 0-9
            "Digit0" => Some(DqKey::Key0),
            "Digit1" => Some(DqKey::Key1),
            "Digit2" => Some(DqKey::Key2),
            "Digit3" => Some(DqKey::Key3),
            "Digit4" => Some(DqKey::Key4),
            "Digit5" => Some(DqKey::Key5),
            "Digit6" => Some(DqKey::Key6),
            "Digit7" => Some(DqKey::Key7),
            "Digit8" => Some(DqKey::Key8),
            "Digit9" => Some(DqKey::Key9),
            // A-Z
            "KeyA" => Some(DqKey::A),
            "KeyB" => Some(DqKey::B),
            "KeyC" => Some(DqKey::C),
            "KeyD" => Some(DqKey::D),
            "KeyE" => Some(DqKey::E),
            "KeyF" => Some(DqKey::F),
            "KeyG" => Some(DqKey::G),
            "KeyH" => Some(DqKey::H),
            "KeyI" => Some(DqKey::I),
            "KeyJ" => Some(DqKey::J),
            "KeyK" => Some(DqKey::K),
            "KeyL" => Some(DqKey::L),
            "KeyM" => Some(DqKey::M),
            "KeyN" => Some(DqKey::N),
            "KeyO" => Some(DqKey::O),
            "KeyP" => Some(DqKey::P),
            "KeyQ" => Some(DqKey::Q),
            "KeyR" => Some(DqKey::R),
            "KeyS" => Some(DqKey::S),
            "KeyT" => Some(DqKey::T),
            "KeyU" => Some(DqKey::U),
            "KeyV" => Some(DqKey::V),
            "KeyW" => Some(DqKey::W),
            "KeyX" => Some(DqKey::X),
            "KeyY" => Some(DqKey::Y),
            "KeyZ" => Some(DqKey::Z),
            // from left to right, from top to bottom
            "Escape" => Some(DqKey::Escape),
            "Tab" => Some(DqKey::Tab),
            "CapsLock" => Some(DqKey::CapsLock),
            "ShiftLeft" => Some(DqKey::LShift),
            "ShiftRight" => Some(DqKey::RShift),
            "ControlLeft" => Some(DqKey::LControl),
            "ControlRight" => Some(DqKey::RControl),
            "AltLeft" => Some(DqKey::LAlt),
            "AltRight" => Some(DqKey::RAlt),
            "Space" => Some(DqKey::Space),
            "ArrowUp" => Some(DqKey::Up),
            "ArrowRight" => Some(DqKey::Right),
            "ArrowDown" => Some(DqKey::Down),
            "ArrowLeft" => Some(DqKey::Left),
            "Enter" => Some(DqKey::Enter),
            "Backspace" => Some(DqKey::Backspace),
            // DqKey::Insert => None,
            "Delete" => Some(DqKey::Delete),
            "Home" => Some(DqKey::Home),
            "PageUp" => Some(DqKey::PageUp),
            "PageDown" => Some(DqKey::PageDown),
            "End" => Some(DqKey::End),
            // belows have passed the simulate test
            "Backquote" => Some(DqKey::Grave),
            "Minus" => Some(DqKey::Minus),
            "Equal" => Some(DqKey::Equal),
            "BracketLeft" => Some(DqKey::LeftBracket),
            "BracketRight" => Some(DqKey::RightBracket),
            "Comma" => Some(DqKey::Comma),
            "Period" => Some(DqKey::Dot),
            "Semicolon" => Some(DqKey::Semicolon),
            "Quote" => Some(DqKey::Apostrophe),
            "Slash" => Some(DqKey::Slash),
            "BackSlash" => Some(DqKey::BackSlash),
            // belows have no exact target in Enigo but can also use in typing
            "Numpad0" => Some(DqKey::Numpad0),
            "Numpad1" => Some(DqKey::Numpad1),
            "Numpad2" => Some(DqKey::Numpad2),
            "Numpad3" => Some(DqKey::Numpad3),
            "Numpad4" => Some(DqKey::Numpad4),
            "Numpad5" => Some(DqKey::Numpad5),
            "Numpad6" => Some(DqKey::Numpad6),
            "Numpad7" => Some(DqKey::Numpad7),
            "Numpad8" => Some(DqKey::Numpad8),
            "Numpad9" => Some(DqKey::Numpad9),
            "NumpadAdd" => Some(DqKey::NumpadAdd),
            "NumpadSubtract" => Some(DqKey::NumpadSubtract),
            "NumpadMultiply" => Some(DqKey::NumpadMultiply),
            "NumpadDivide" => Some(DqKey::NumpadDivide),
            _ => None
        }
    }

    /// `Key` in `Enigo` => `code` in frontend
    pub fn enigo_to_front(key_in_enigo: EnigoKey) -> Option<&'static str> {
        match key_in_enigo {
            // F1-F12
            EnigoKey::F1 => Some("F1"),
            EnigoKey::F2 => Some("F2"),
            EnigoKey::F3 => Some("F3"),
            EnigoKey::F4 => Some("F4"),
            EnigoKey::F5 => Some("F5"),
            EnigoKey::F6 => Some("F6"),
            EnigoKey::F7 => Some("F7"),
            EnigoKey::F8 => Some("F8"),
            EnigoKey::F9 => Some("F9"),
            EnigoKey::F10 => Some("F10"),
            EnigoKey::F11 => Some("F11"),
            EnigoKey::F12 => Some("F12"),
            // 0-9
            EnigoKey::Layout('0') => Some("Digit0"),
            EnigoKey::Layout('1') => Some("Digit1"),
            EnigoKey::Layout('2') => Some("Digit2"),
            EnigoKey::Layout('3') => Some("Digit3"),
            EnigoKey::Layout('4') => Some("Digit4"),
            EnigoKey::Layout('5') => Some("Digit5"),
            EnigoKey::Layout('6') => Some("Digit6"),
            EnigoKey::Layout('7') => Some("Digit7"),
            EnigoKey::Layout('8') => Some("Digit8"),
            EnigoKey::Layout('9') => Some("Digit9"),
            // A-Z
            EnigoKey::Layout('a') => Some("KeyA"),
            EnigoKey::Layout('b') => Some("KeyB"),
            EnigoKey::Layout('c') => Some("KeyC"),
            EnigoKey::Layout('d') => Some("KeyD"),
            EnigoKey::Layout('e') => Some("KeyE"),
            EnigoKey::Layout('f') => Some("KeyF"),
            EnigoKey::Layout('g') => Some("KeyG"),
            EnigoKey::Layout('h') => Some("KeyH"),
            EnigoKey::Layout('i') => Some("KeyI"),
            EnigoKey::Layout('j') => Some("KeyJ"),
            EnigoKey::Layout('k') => Some("KeyK"),
            EnigoKey::Layout('l') => Some("KeyL"),
            EnigoKey::Layout('m') => Some("KeyM"),
            EnigoKey::Layout('n') => Some("KeyN"),
            EnigoKey::Layout('o') => Some("KeyO"),
            EnigoKey::Layout('p') => Some("KeyP"),
            EnigoKey::Layout('q') => Some("KeyQ"),
            EnigoKey::Layout('r') => Some("KeyR"),
            EnigoKey::Layout('s') => Some("KeyS"),
            EnigoKey::Layout('t') => Some("KeyT"),
            EnigoKey::Layout('u') => Some("KeyU"),
            EnigoKey::Layout('v') => Some("KeyV"),
            EnigoKey::Layout('w') => Some("KeyW"),
            EnigoKey::Layout('x') => Some("KeyX"),
            EnigoKey::Layout('y') => Some("KeyY"),
            EnigoKey::Layout('z') => Some("KeyZ"),
            // from left to right, from top to bottom
            EnigoKey::Escape => Some("Escape"),
            EnigoKey::Tab => Some("Tab"),
            EnigoKey::CapsLock => Some("CapsLock"),
            EnigoKey::Shift => Some("ShiftLeft"),
            EnigoKey::Control => Some("ControlLeft"),
            EnigoKey::Alt => Some("AltLeft"),
            EnigoKey::Space => Some("Space"),
            EnigoKey::UpArrow => Some("ArrowUp"),
            EnigoKey::RightArrow => Some("ArrowRight"),
            EnigoKey::DownArrow => Some("ArrowDown"),
            EnigoKey::LeftArrow => Some("ArrowLeft"),
            EnigoKey::Return => Some("Enter"),
            EnigoKey::Backspace => Some("Backspace"),
            // DqKey::Insert => None,
            EnigoKey::Delete => Some("Delete"),
            EnigoKey::Home => Some("Home"),
            EnigoKey::PageUp => Some("PageUp"),
            EnigoKey::PageDown => Some("PageDown"),
            EnigoKey::End => Some("End"),
            // belows have passed the simulate test
            EnigoKey::Layout('`') => Some("Backquote"),
            EnigoKey::Layout('-') => Some("Minus"),
            EnigoKey::Layout('=') => Some("Equal"),
            EnigoKey::Layout('[') => Some("BracketLeft"),
            EnigoKey::Layout(']') => Some("BracketRight"),
            EnigoKey::Layout(',') => Some("Comma"),
            EnigoKey::Layout('.') => Some("Period"),
            EnigoKey::Layout(';') => Some("Semicolon"),
            EnigoKey::Layout('\'') => Some("Quote"),
            EnigoKey::Layout('/') => Some("Slash"),
            EnigoKey::Layout('\\') => Some("BackSlash"),
            _ => None
        }
    }

    /// `code` in frontend => `Key` in `Enigo`
    pub fn front_to_enigo(code_in_front: &str) -> Option<EnigoKey> {
        match code_in_front {
            // F1-F12
            "F1" => Some(EnigoKey::F1),
            "F2" => Some(EnigoKey::F2),
            "F3" => Some(EnigoKey::F3),
            "F4" => Some(EnigoKey::F4),
            "F5" => Some(EnigoKey::F5),
            "F6" => Some(EnigoKey::F6),
            "F7" => Some(EnigoKey::F7),
            "F8" => Some(EnigoKey::F8),
            "F9" => Some(EnigoKey::F9),
            "F10" => Some(EnigoKey::F10),
            "F11" => Some(EnigoKey::F11),
            "F12" => Some(EnigoKey::F12),
            // 0-9
            "Digit0" | "Numpad0" => Some(EnigoKey::Layout('0')),
            "Digit1" | "Numpad1" => Some(EnigoKey::Layout('1')),
            "Digit2" | "Numpad2" => Some(EnigoKey::Layout('2')),
            "Digit3" | "Numpad3" => Some(EnigoKey::Layout('3')),
            "Digit4" | "Numpad4" => Some(EnigoKey::Layout('4')),
            "Digit5" | "Numpad5" => Some(EnigoKey::Layout('5')),
            "Digit6" | "Numpad6" => Some(EnigoKey::Layout('6')),
            "Digit7" | "Numpad7" => Some(EnigoKey::Layout('7')),
            "Digit8" | "Numpad8" => Some(EnigoKey::Layout('8')),
            "Digit9" | "Numpad9" => Some(EnigoKey::Layout('9')),
            // A-Z
            "KeyA" => Some(EnigoKey::Layout('a')),
            "KeyB" => Some(EnigoKey::Layout('b')),
            "KeyC" => Some(EnigoKey::Layout('c')),
            "KeyD" => Some(EnigoKey::Layout('d')),
            "KeyE" => Some(EnigoKey::Layout('e')),
            "KeyF" => Some(EnigoKey::Layout('f')),
            "KeyG" => Some(EnigoKey::Layout('g')),
            "KeyH" => Some(EnigoKey::Layout('h')),
            "KeyI" => Some(EnigoKey::Layout('i')),
            "KeyJ" => Some(EnigoKey::Layout('j')),
            "KeyK" => Some(EnigoKey::Layout('k')),
            "KeyL" => Some(EnigoKey::Layout('l')),
            "KeyM" => Some(EnigoKey::Layout('m')),
            "KeyN" => Some(EnigoKey::Layout('n')),
            "KeyO" => Some(EnigoKey::Layout('o')),
            "KeyP" => Some(EnigoKey::Layout('p')),
            "KeyQ" => Some(EnigoKey::Layout('q')),
            "KeyR" => Some(EnigoKey::Layout('r')),
            "KeyS" => Some(EnigoKey::Layout('s')),
            "KeyT" => Some(EnigoKey::Layout('t')),
            "KeyU" => Some(EnigoKey::Layout('u')),
            "KeyV" => Some(EnigoKey::Layout('v')),
            "KeyW" => Some(EnigoKey::Layout('w')),
            "KeyX" => Some(EnigoKey::Layout('x')),
            "KeyY" => Some(EnigoKey::Layout('y')),
            "KeyZ" => Some(EnigoKey::Layout('z')),
            // from left to right, from top to bottom
            "Escape" => Some(EnigoKey::Escape),
            "Tab" => Some(EnigoKey::Tab),
            "CapsLock" => Some(EnigoKey::CapsLock),
            "ShiftLeft" | "ShiftRight" => Some(EnigoKey::Shift),
            "ControlLeft" | "ControlRight" => Some(EnigoKey::Control),
            "AltLeft" | "AltRight" => Some(EnigoKey::Alt),
            "Space" => Some(EnigoKey::Space),
            "ArrowUp" => Some(EnigoKey::UpArrow),
            "ArrowRight" => Some(EnigoKey::RightArrow),
            "ArrowDown" => Some(EnigoKey::DownArrow),
            "ArrowLeft" => Some(EnigoKey::LeftArrow),
            "Enter" => Some(EnigoKey::Return),
            "Backspace" => Some(EnigoKey::Backspace),
            // DqKey::Insert => None,
            "Delete" => Some(EnigoKey::Delete),
            "Home" => Some(EnigoKey::Home),
            "PageUp" => Some(EnigoKey::PageUp),
            "PageDown" => Some(EnigoKey::PageDown),
            "End" => Some(EnigoKey::End),
            // belows have passed the simulate test
            "Backquote" => Some(EnigoKey::Layout('`')),
            "Minus" | "NumpadSubtract" => Some(EnigoKey::Layout('-')),
            "Equal" => Some(EnigoKey::Layout('=')),
            "BracketLeft" => Some(EnigoKey::Layout('[')),
            "BracketRight" => Some(EnigoKey::Layout(']')),
            "Comma" => Some(EnigoKey::Layout(',')),
            "Period" => Some(EnigoKey::Layout('.')),
            "Semicolon" => Some(EnigoKey::Layout(';')),
            "Quote" => Some(EnigoKey::Layout('\'')),
            "Slash" | "NumpadDivide" => Some(EnigoKey::Layout('/')),
            "BackSlash" => Some(EnigoKey::Layout('\\')),
            _ => None
        }
    }
}
// endregion

// region mouse mapper
pub struct MouseMapper {}

#[allow(unused)]
impl MouseMapper {
    /// parse custom enum `MouseEventName` to `(EnigoButton, is_press)`
    pub fn parse_ev_name(ev_name: MouseEventName) -> (EnigoButton, bool) {
        match ev_name {
            MouseEventName::LeftDown => (EnigoButton::Left, true),
            MouseEventName::LeftUp => (EnigoButton::Left, false),
            MouseEventName::RightDown => (EnigoButton::Right, false),
            MouseEventName::RightUp => (EnigoButton::Right, false),
            MouseEventName::MidDown => (EnigoButton::Middle, false),
            MouseEventName::MidUp => (EnigoButton::Middle, false),
        }
    }

    /// `MouseButton` in `device_query` => `MouseButton` in `Enigo`
    pub fn dq_to_enigo(button_in_dq: DqButton) -> Option<EnigoButton> {
        match button_in_dq {
            1 => Some(EnigoButton::Left),
            2 => Some(EnigoButton::Right),
            3 => Some(EnigoButton::Middle),
            _ => None
        }
    }

    /// `MouseButton` in `Enigo` => `MouseButton` in `device_query`
    pub fn enigo_to_dq(button_in_enigo: EnigoButton) -> Option<DqButton> {
        match button_in_enigo {
            EnigoButton::Left => Some(1),
            EnigoButton::Right => Some(2),
            EnigoButton::Middle => Some(3),
            _ => None
        }
    }

    /// `MouseButton` in `device_query` => `button` in frontend
    pub fn dq_to_front(button_in_dq: DqButton) -> Option<usize> {
        match button_in_dq {
            1 => Some(0),
            2 => Some(2),
            3 => Some(1),
            _ => None
        }
    }

    /// `button` in frontend => `MouseButton` in `device_query`
    pub fn front_to_dq(code_in_front: usize) -> Option<DqButton> {
        match code_in_front {
            0 => Some(1),
            2 => Some(2),
            1 => Some(3),
            _ => None
        }
    }

    /// `MouseButton` in `Enigo` => `button` in frontend
    pub fn enigo_to_front(button_in_enigo: EnigoButton) -> Option<usize> {
        match button_in_enigo {
            EnigoButton::Left => Some(0),
            EnigoButton::Right => Some(2),
            EnigoButton::Middle => Some(1),
            _ => None
        }
    }

    /// `button` in frontend => `MouseButton` in `Enigo`
    pub fn front_to_enigo(code_in_front: usize) -> Option<EnigoButton> {
        match code_in_front {
            0 => Some(EnigoButton::Left),
            2 => Some(EnigoButton::Right),
            1 => Some(EnigoButton::Middle),
            _ => None
        }
    }
}
// endregion
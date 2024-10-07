use rdev::{listen, Event, EventType, Key as keyer};
use enigo::*;

fn main() {
    if let Err(error) = listen(move |event| {
        callback(event)
    }) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    let mut set = Enigo::new(&Settings::default()).unwrap();
    let _ = match event.event_type {
        EventType::KeyPress(keyer::F1) => set.key(Key::F13, Direction::Click),
        EventType::KeyPress(keyer::F2) => set.key(Key::F14, Direction::Click),
        EventType::KeyPress(keyer::F3) => set.key(Key::F15, Direction::Click),
        EventType::KeyPress(keyer::F4) => set.key(Key::F16, Direction::Click),
        EventType::KeyPress(keyer::F5) => set.key(Key::F17, Direction::Click),
        EventType::KeyPress(keyer::F6) => set.key(Key::F18, Direction::Click),
        EventType::KeyPress(keyer::F7) => set.key(Key::F19, Direction::Click),
        EventType::KeyPress(keyer::F8) => set.key(Key::F20, Direction::Click),
        EventType::KeyPress(keyer::F9) => set.key(Key::F21, Direction::Click),
        EventType::KeyPress(keyer::F10) => set.key(Key::F22, Direction::Click),
        EventType::KeyPress(keyer::F11) => set.key(Key::F23, Direction::Click),
        EventType::KeyPress(keyer::F12) => set.key(Key::F24, Direction::Click),
        _ => Ok(()),
    };
}
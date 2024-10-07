use rdev::{listen, Event, EventType, Key as keyer};
use enigo::*;
use std::{sync::atomic::{AtomicBool, Ordering}, thread, time::Duration};

static SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);

fn main() {
    if let Err(error) = listen(move |event| {
        match event.event_type {
            EventType::KeyPress(keyer::ShiftLeft) | EventType::KeyPress(keyer::ShiftRight) => {
                SHIFT_PRESSED.store(true, Ordering::SeqCst);
            },
            EventType::KeyRelease(keyer::ShiftLeft) | EventType::KeyRelease(keyer::ShiftRight) => {
                SHIFT_PRESSED.store(false, Ordering::SeqCst);
            },
            EventType::KeyPress(_) => {
                callback(event);
            },
            _ => (),
        }
    }) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    let mut set = match Enigo::new(&Settings::default()) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to initialize Enigo: {:?}", e);
            return;
        }
    };

    if SHIFT_PRESSED.load(Ordering::SeqCst) {
        match event.event_type {
            EventType::KeyPress(keyer::F1) => {
                println!("Shift + F1 pressed");
                if let Err(e) = set.key(Key::F13, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F2) => {
                println!("Shift + F2 pressed");
                if let Err(e) = set.key(Key::F14, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F3) => {
                println!("Shift + F3 pressed");
                if let Err(e) = set.key(Key::F15, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F4) => {
                println!("Shift + F4 pressed");
                if let Err(e) = set.key(Key::F16, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F5) => {
                println!("Shift + F5 pressed");
                if let Err(e) = set.key(Key::F17, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F6) => {
                println!("Shift + F6 pressed");
                if let Err(e) = set.key(Key::F18, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F7) => {
                println!("Shift + F7 pressed");
                if let Err(e) = set.key(Key::F19, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F8) => {
                println!("Shift + F8 pressed");
                if let Err(e) = set.key(Key::F20, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F9) => {
                println!("Shift + F9 pressed");
                if let Err(e) = set.key(Key::F21, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F10) => {
                println!("Shift + F10 pressed");
                if let Err(e) = set.key(Key::F22, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F11) => {
                println!("Shift + F11 pressed");
                if let Err(e) = set.key(Key::F23, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            EventType::KeyPress(keyer::F12) => {
                println!("Shift + F12 pressed");
                if let Err(e) = set.key(Key::F24, Direction::Click) {
                    println!("Failed to set key: {:?}", e);
                }
            },
            _ => (),
        }
    }

    thread::sleep(Duration::from_millis(10));
}

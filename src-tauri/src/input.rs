use std::collections::HashMap;
use rdev::{listen, Event};

pub fn start() {
    fn callback(event: Event) {
        match event.event_type {
            rdev::EventType::KeyPress(key) => println!("Key Pressed: {:?}", key),
            rdev::EventType::KeyRelease(key) => println!("Key Released: {:?}", key),
            _ => {}
        }
    }

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
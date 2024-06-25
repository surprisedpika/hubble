use rdev::{ listen, Event };

use crate::get_keys;

pub fn start() {
    let callback = move |event: Event| {
        let keys = get_keys();
        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                keys.write().unwrap().insert(key);
            }
            rdev::EventType::KeyRelease(key) => {
                keys.write().unwrap().remove(&key);
            }
            _ => {}
        }
    };

    listen(callback).unwrap();
}

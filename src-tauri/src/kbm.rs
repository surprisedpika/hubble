use rdev::{ listen, Event };

use crate::get_keys;

pub fn start() {
    let callback = move |event: Event| {
        let keys = get_keys();
        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                keys.write().unwrap().insert(format!("kb_{:?}", key));
            }
            rdev::EventType::KeyRelease(key) => {
                keys.write().unwrap().remove(&format!("kb_{:?}", key));
            }
            rdev::EventType::ButtonPress(button) => {
                keys.write().unwrap().insert(format!("ms_{:?}", button));
            }
            rdev::EventType::ButtonRelease(button) => {
                keys.write().unwrap().remove(&format!("ms_{:?}", button));
            }
            rdev::EventType::Wheel { delta_x, delta_y } => {
                if delta_y != 0 {
                    if delta_y > 0 {
                        keys.write().unwrap().insert(String::from("mw_Up"));
                    } else {
                        keys.write().unwrap().insert(String::from("mw_Down"));
                    }
                }
                if delta_x != 0 {
                    if delta_x > 0 {
                        keys.write().unwrap().insert(String::from("mw_Right"));
                    } else {
                        keys.write().unwrap().insert(String::from("mw_Left"));
                    }
                } else {
                    if delta_y == 0 {
                        // Edge case: Wheel was scrolled but not in X or Y direction
                        // Should be impossible, but it's there just in case
                        keys.write().unwrap().insert(String::from("mw_Unknown"));
                    }
                }
            }
            rdev::EventType::MouseMove { x, y } => {
                keys.write()
                    .unwrap()
                    .retain(|k| !k.starts_with("mx_") && !k.starts_with("my_"));
                keys.write().unwrap().replace(format!("mx_{:?}", x));
                keys.write().unwrap().replace(format!("my_{:?}", y));
            }
        }
    };

    listen(callback).unwrap();
}

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
                //TODO: Wheel events get "stuck on" until a subsequent event
                if delta_y != 0 {
                    if delta_y > 0 {
                        keys.write().unwrap().insert(String::from("mw_Up"));
                        return;
                    } else {
                        keys.write().unwrap().insert(String::from("mw_Down"));
                        return;
                    }
                } else if delta_x != 0 {
                    if delta_x > 0 {
                        keys.write().unwrap().insert(String::from("mw_Right"));
                        return;
                    } else {
                        keys.write().unwrap().insert(String::from("mw_Left"));
                        return;
                    }
                }
            }
            _ => {}
        }

        if !matches!(event.event_type, rdev::EventType::Wheel { .. }) {
            keys.write()
                .unwrap()
                .retain(|k| !k.starts_with("mw_"));
        }
    };

    listen(callback).unwrap();
}

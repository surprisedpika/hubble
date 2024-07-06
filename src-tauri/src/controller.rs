use hidapi::HidApi;

use crate::{
    controllers::{ procon::Procon, steaminput::SteamInput },
    get_controller,
    get_controller_polling_state,
};

pub trait GetBit {
    /** Returns the bit at the index `n`, which can range from `0 - (num_bits - 1)`<br/> 
        When `n = 0`, the **least significant** bit is returned. When it is `(num_bits - 1)`, the most significant bit is returned.
     */
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool;
}

impl GetBit for u128 {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool {
        let mask = 1 << n.into();
        (self & mask) == mask
    }
}

impl GetBit for u8 {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool {
        let mask = 1 << n.into();
        (self & mask) == mask
    }
}

#[derive(Clone)]
enum Vendor {
    Nintendo,
    Sony,
    Microsoft,
}

impl Vendor {
    fn id(&self) -> u16 {
        match self {
            Vendor::Nintendo => 0x057e,
            Vendor::Sony => 0x054c,
            Vendor::Microsoft => 0x045e,
        }
    }

    fn product_ids(&self) -> Vec<u16> {
        match self {
            Vendor::Nintendo => vec![0x2009],
            _ => Vec::new(),
        }
    }

    fn iter() -> impl Iterator<Item = Vendor> {
        [Vendor::Nintendo, Vendor::Sony, Vendor::Microsoft].iter().cloned()
    }
}

/** Generic controller struct all other controllers are converted into
 */
#[derive(Clone, serde::Serialize)]
pub struct Controller {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,

    pub r_trigger: bool,
    pub l_trigger: bool,
    pub r_bumper: bool,
    pub l_bumper: bool,

    pub l_stick_click: bool,
    pub r_stick_click: bool,

    pub dpad_north: bool,
    pub dpad_east: bool,
    pub dpad_west: bool,
    pub dpad_south: bool,

    pub face_top_left: bool,
    pub face_bottom_left: bool,
    pub face_top_right: bool,
    pub face_bottom_right: bool,

    pub l_stick: (f32, f32),
    pub r_stick: (f32, f32),

    pub unknown: Vec<bool>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            north: false,
            east: false,
            south: false,
            west: false,

            r_trigger: false,
            r_bumper: false,
            l_trigger: false,
            l_bumper: false,
            l_stick_click: false,
            r_stick_click: false,
            dpad_north: false,
            dpad_east: false,
            dpad_west: false,
            dpad_south: false,
            face_top_left: false,
            face_bottom_left: false,
            face_top_right: false,
            face_bottom_right: false,
            l_stick: (0f32, 0f32),
            r_stick: (0f32, 0f32),
            unknown: Vec::new(),
        }
    }
}

/** Iterates through every known controller device and attempts to open one of them.<br>
    Returns the first device it finds, or an error if it finds none
*/
fn get_ids(api: &HidApi) -> Option<(u16, u16)> {
    let vendors = Vendor::iter();
    for vendor in vendors {
        let product_ids = vendor.product_ids();
        let pids = product_ids.iter();
        for pid in pids {
            let device = api.open(vendor.id(), *pid);
            if let Ok(_) = device {
                println!("Controller found");
                return Some((vendor.id(), *pid));
            }
        }
    }
    None
}

pub fn start() {
    println!("Controller listening started");

    let api = hidapi::HidApi::new().unwrap();
    let mut ids: Option<(u16, u16)> = get_ids(&api);
    while get_controller_polling_state().read().unwrap().clone() == true {
        if let Some(ok_ids) = ids {
            if let Ok(ref device) = api.open(ok_ids.0, ok_ids.1) {
                let mut buf = [0u8; 16];
                let result = device.read_timeout(&mut buf[..], 3000);
                // Device was unplugged
                if let Ok(res) = result {
                    let data_arr: &[u8] = &buf[..res];
                    let mut data: u128 = 0;
                    for byte in data_arr.iter().rev() {
                        data = (data << 8) | (*byte as u128);
                    }
                    // This is very scientific dont worry about it
                    let is_steaminput = (data << 124) == 0;
                    let controller_data = if is_steaminput {
                        SteamInput::from_bytes(data)
                    } else {
                        Procon::from_bytes(data)
                    };
                    let controller = &get_controller();
                    let mut writeable_controller = controller.write().unwrap();
                    *writeable_controller = controller_data;
                }
            } else {
                ids = get_ids(&api);
            }
        } else {
            ids = get_ids(&api);
        }
    }
}

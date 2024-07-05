use crate::{ controllers::{ procon::Procon, steaminput::SteamInput }, get_controller };

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

enum VID {
    Nintendo = 0x057e,
    Sony = 0x054c,
    Microsoft = 0x045e,
}

enum PID {
    ProCon = 0x2009,
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

pub fn start() {
    println!("Controller listening started");

    let api = hidapi::HidApi::new().unwrap();

    loop {
        let (vid, pid) = (VID::Nintendo, PID::ProCon);
        let procon = api.open(vid as u16, pid as u16);
        if let Ok(ref device) = procon {
            let mut buf = [0u8; 16];
            let result = device.read_timeout(&mut buf[..], 3000);
            if let Ok(res) = result {
                let data_arr: &[u8] = &buf[..res];
                let mut data: u128 = 0;
                for byte in data_arr.iter().rev() {
                    // print!("{:08b} ", byte);
                    data = (data << 8) | (*byte as u128);
                }
                // print!("\n");
                let controller = &get_controller();
                let mut writeable_controller = controller.write().unwrap();
                // This is very scientific dont worry about it
                let is_steaminput = (data << 124) == 0;
                let controller_data = if is_steaminput {
                    SteamInput::from_bytes(data)
                } else {
                    Procon::from_bytes(data)
                };
                *writeable_controller = controller_data;
            }
        }
    }
}

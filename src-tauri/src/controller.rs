use crate::{ controllers::{ procon::Procon, steaminput::SteamInput }, get_controller };

pub trait GetBit {
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

    pub d_up: bool,
    pub d_right: bool,
    pub d_left: bool,
    pub d_down: bool,

    pub face_left_top: bool,
    pub face_left_bottom: bool,
    pub face_right_top: bool,
    pub face_right_bottom: bool,

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
            d_up: false,
            d_right: false,
            d_left: false,
            d_down: false,
            face_left_top: false,
            face_left_bottom: false,
            face_right_top: false,
            face_right_bottom: false,
            l_stick: (0f32, 0f32),
            r_stick: (0f32, 0f32),
            unknown: Vec::new(),
        }
    }
}

pub fn start() {
    println!("Controller listening started");

    // pro con vendor id: 1406 (0x057E)
    // pro con product id: 8201 (0x2009)

    let api = hidapi::HidApi::new().unwrap();
    let (procon_vid, procon_pid) = (0x057e, 0x2009);
    let procon = api.open(procon_vid, procon_pid);

    if let Ok(ref device) = procon {
        println!("Controller Found");
        loop {
            let mut buf = [0u8; 16];
            let res = device.read_timeout(&mut buf[..], 3000).unwrap();
            let data_arr: &[u8] = &buf[..res];
            let mut data: u128 = 0;
            for byte in data_arr.iter().rev() {
                // print!("{:08b} ", byte);
                data = (data << 8) | (*byte as u128);
            }
            // print!("\n");
            let controller = &get_controller();
            let mut writeable_controller = controller.write().unwrap();
            let controller_data = SteamInput::from_bytes(data);
            *writeable_controller = controller_data;
        }
    }
}

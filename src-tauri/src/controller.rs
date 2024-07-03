use crate::get_controller;

trait GetBits {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool;

    // fn get_bits<T: Into<u8>>(&self, index: T, num_bits: u8) -> Option<Vec<bool>>;
}

// impl GetBits for u8 {
//     fn get_bit<T: Into<u8>>(&self, n: T) -> bool {
//         let mask = 1 << n.into();
//         (self & mask) == mask
//     }

//     fn get_bits<T: Into<u8>>(&self, index: T, num_bits: u8) -> Option<Vec<bool>> {
//         let index = index.into();
//         if index + num_bits >= 8 || num_bits == 0 {
//             None
//         } else {
//             let mut bits: Vec<bool> = Vec::new();

//             for i in index..index + num_bits {
//                 bits.push(self.get_bit(i));
//             }

//             Some(bits)
//         }
//     }
// }

impl GetBits for u128 {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool {
        let mask = 1 << n.into();
        (self & mask) == mask
    }

    // fn get_bits<T: Into<u8>>(&self, index: T, num_bits: u8) -> Option<Vec<bool>> {
    //     let index = index.into();
    //     if index + num_bits >= 128 || num_bits == 0 {
    //         None
    //     } else {
    //         let mut bits: Vec<bool> = Vec::new();

    //         for i in index..index + num_bits {
    //             bits.push(self.get_bit(i));
    //         }

    //         Some(bits)
    //     }
    // }
}

#[derive(Clone, Copy)]
enum Procon {
    // 0-7: Input report ID

    B = 08,
    A = 09,
    Y = 10,
    X = 11,
    L = 12,
    R = 13,
    Zl = 14,
    Zr = 15,

    Plus = 16,
    Minus = 17,
    LStick = 18,
    RStick = 19,
    Home = 20,
    Screenshot = 21,
    Unknown1 = 22,
    Unknown2 = 23,

    // 24: Dpad
    // 25: Dpad
    // 26: Dpad
    // 27: Dpad
    Unknown3 = 28,
    Unknown4 = 29,
    Unknown5 = 30,
    Unknown6 = 31,

    // Lstick: 32 - 55
    // Rstick: 56 - 79
}

impl Procon {
    // No, I don't know why nintendo did it this way either
    fn decode_dpad(data: u128) -> (bool, bool, bool, bool) {
        let mut dpad_bits: u8 = 0;
        dpad_bits += data.get_bit(24) as u8;
        dpad_bits += (data.get_bit(25) as u8) << 1;
        dpad_bits += (data.get_bit(26) as u8) << 2;
        dpad_bits += (data.get_bit(27) as u8) << 3;

        if dpad_bits > 7 {
            return (false, false, false, false);
        } else {
            let up = if dpad_bits == 7 || dpad_bits <= 1 { true } else { false };
            let down = if 3 <= dpad_bits && dpad_bits <= 5 { true } else { false };
            let left = if 5 <= dpad_bits && dpad_bits <= 7 { true } else { false };
            let right = if 1 <= dpad_bits && dpad_bits <= 3 { true } else { false };
            (up, down, left, right)
        }
    }

    fn get_stick_data(data: u128, left: bool) -> (f32, f32) {
        let offset = if left { 4 } else { 8 };
        let bytes = data.to_le_bytes();
        let data = &bytes[offset..];
        let stick_x: f32 = u16::from_le_bytes([data[0], data[1]]).into();
        let stick_y: f32 = u16::from_le_bytes([data[2], data[3]]).into();
        let x: f32 = (stick_x - 32767f32) / 32767f32;
        let y: f32 = (stick_y - 32767f32) / 32767f32;
        (x, y)
    }
}

impl Into<u8> for Procon {
    fn into(self) -> u8 {
        self as u8
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
    pub fn from_procon_bytes(data: u128) -> Controller {
        let dpad = Procon::decode_dpad(data);

        Controller {
            north: data.get_bit(Procon::X),
            east: data.get_bit(Procon::A),
            south: data.get_bit(Procon::B),
            west: data.get_bit(Procon::Y),

            r_trigger: data.get_bit(Procon::Zr),
            l_trigger: data.get_bit(Procon::Zl),
            r_bumper: data.get_bit(Procon::R),
            l_bumper: data.get_bit(Procon::L),

            r_stick_click: data.get_bit(Procon::RStick),
            l_stick_click: data.get_bit(Procon::LStick),

            d_up: dpad.0,
            d_down: dpad.1,
            d_left: dpad.2,
            d_right: dpad.3,

            face_left_top: data.get_bit(Procon::Plus),
            face_left_bottom: data.get_bit(Procon::Home),
            face_right_top: data.get_bit(Procon::Minus),
            face_right_bottom: data.get_bit(Procon::Screenshot),
            l_stick: Procon::get_stick_data(data, true),
            r_stick: Procon::get_stick_data(data, false),

            unknown: vec![
                data.get_bit(Procon::Unknown1),
                data.get_bit(Procon::Unknown2),
                data.get_bit(Procon::Unknown3),
                data.get_bit(Procon::Unknown4),
                data.get_bit(Procon::Unknown5),
                data.get_bit(Procon::Unknown6)
            ],
        }
    }

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
        loop {
            let mut buf = [0u8; 16];
            let res = device.read(&mut buf[..]).unwrap();
            let data_arr: &[u8] = &buf[..res];
            let mut data: u128 = 0;
            for byte in data_arr.iter().rev() {
                data = (data << 8) | (*byte as u128);
            }
            let controller = &get_controller();
            let mut writeable_controller = controller.write().unwrap();
            let controller_data = Controller::from_procon_bytes(data);
            *writeable_controller = controller_data;
        }
    }
}

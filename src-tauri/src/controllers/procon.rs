use crate::controller::{ Controller, GetBit };

#[derive(Clone, Copy)]
pub enum Procon {
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
    fn decode_dpad(data: u128) -> (bool, bool, bool, bool) {
        /*
            The procon dpad data is encoded as a sort of vector, where after reversing the bits, 
            a value of `0` means up is being pressed, `1` means up right, `2` means right, etc. 
            until `7` loops around to meaning up left. A value of `8` means there is no dpad input
        */
        let mut dpad_bits: u8 = 0;
        dpad_bits |= data.get_bit(24) as u8;
        dpad_bits |= (data.get_bit(25) as u8) << 1;
        dpad_bits |= (data.get_bit(26) as u8) << 2;
        dpad_bits |= (data.get_bit(27) as u8) << 3;

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

    /** Converts the 4 bytes of stick data to two `f32`s ranging from -1 to 1
     */
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

    pub fn from_bytes(data: u128) -> Controller {
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

            dpad_north: dpad.0,
            dpad_south: dpad.1,
            dpad_west: dpad.2,
            dpad_east: dpad.3,

            face_top_left: data.get_bit(Procon::Plus),
            face_bottom_left: data.get_bit(Procon::Home),
            face_top_right: data.get_bit(Procon::Minus),
            face_bottom_right: data.get_bit(Procon::Screenshot),
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
}

impl Into<u8> for Procon {
    fn into(self) -> u8 {
        self as u8
    }
}

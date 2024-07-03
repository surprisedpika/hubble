// 2nd byte is 0 when using steaminput, or 0b11 when using hid
use crate::controller::{ Controller, GetBit };

#[derive(Clone, Copy)]
pub enum SteamInput {
    // This might actually be xinput or it might be steaminput but only for procons who knows

    // 0 - 23: Always (?) 0b0
    // 24-31: Always (?) 0b1011

    // 32-55: rstick
    // 56-79: lstick

    LTrigger = 80,
    LBumper = 81,
    Unknown1 = 82,
    Unknown2 = 83,
    DPadLeft = 84,
    DPadRight = 85,
    DPadUp = 86,
    DPadDown = 87,

    Unknown3 = 88,
    Unknown4 = 89,
    Screenshot = 90,
    Home = 91,
    LStick = 92,
    RStick = 93,
    Plus = 94,
    Minus = 95,

    RTrigger = 96,
    RBumper = 97,
    Unknown5 = 98,
    Unknown6 = 99,
    A = 100,
    B = 101,
    X = 102,
    Y = 103,

    // 104-111: Always (?) 0b10000000
    // 112-119: Motion Data
    // 120-127: Always (?) 0b00110000
}

impl SteamInput {
    fn get_stick_data(data: u128, left: bool) -> (f32, f32) {
        // this is true
        // 24 bits per stick
        // each direction is an i12 (dw about it)
        // the first 12 bits are the y
        // the second 12 bits are the x
        // big endian
        let offset = if left { 7 } else { 4 };
        let bytes = data.to_be_bytes();
        let data = &bytes[offset..offset + 3];
        // this is less true
        // data[0] == 1(sign)7(most significant bits of stick_y)
        // data[1] == 4(least significant bits of stick_y)1(sign)3(most significant bits of stick_x)
        // data[2] == 8(least significant bits of stick_x)
        if left {
            print!("{:08b} ", data[0]);
            print!("{:08b} ", (data[1] >> 4) << 4);
            print!("\n");
        }
        (0f32, 0f32)
    }

    pub fn from_bytes(d: u128) -> Controller {
        let data = d.reverse_bits();
        Controller {
            north: data.get_bit(SteamInput::X),
            east: data.get_bit(SteamInput::A),
            south: data.get_bit(SteamInput::B),
            west: data.get_bit(SteamInput::Y),
            r_trigger: data.get_bit(SteamInput::RTrigger),
            l_trigger: data.get_bit(SteamInput::LTrigger),
            r_bumper: data.get_bit(SteamInput::RBumper),
            l_bumper: data.get_bit(SteamInput::LBumper),
            l_stick_click: data.get_bit(SteamInput::LStick),
            r_stick_click: data.get_bit(SteamInput::RStick),
            d_up: data.get_bit(SteamInput::DPadUp),
            d_right: data.get_bit(SteamInput::DPadRight),
            d_left: data.get_bit(SteamInput::DPadLeft),
            d_down: data.get_bit(SteamInput::DPadDown),
            face_left_top: data.get_bit(SteamInput::Minus),
            face_left_bottom: data.get_bit(SteamInput::Screenshot),
            face_right_top: data.get_bit(SteamInput::Plus),
            face_right_bottom: data.get_bit(SteamInput::Home),
            l_stick: SteamInput::get_stick_data(data, true),
            r_stick: SteamInput::get_stick_data(data, false),
            unknown: vec![
                data.get_bit(SteamInput::Unknown1),
                data.get_bit(SteamInput::Unknown2),
                data.get_bit(SteamInput::Unknown3),
                data.get_bit(SteamInput::Unknown4),
                data.get_bit(SteamInput::Unknown5),
                data.get_bit(SteamInput::Unknown6)
            ],
        }
    }
}

impl Into<u8> for SteamInput {
    fn into(self) -> u8 {
        self as u8
    }
}

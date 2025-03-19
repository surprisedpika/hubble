use crate::controller::{ Controller, GetBit };

#[derive(Clone, Copy)]
pub enum SteamInput {
    // This might only work for steam controllers or it might generalise I don't know

    // 0 - 7   (1 byte): Always (?) 0b0
    // 8 - 15  (1 byte): As above
    // 16 - 23 (1 byte): As above
    // 24 - 31 (1 byte): Always (?) 0b1011

    // 32 - 55 (3 byte): rstick
    // 56 - 79 (3 byte): lstick

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

    // 104-111 (1 byte): Always (?) 0b10000000
    // 112-119 (1 byte): Motion Data
    // 120-127 (1 byte): Always (?) 0b00110000
}

impl SteamInput {
    /** Converts the 3 bytes of stick data to two `f32`s ranging from -1 to 1
     */
    fn get_stick_data(data: u128, left: bool) -> (f32, f32) {
        let offset = if left { 7 } else { 4 };
        let bytes = data.to_le_bytes();
        let data = &bytes[offset..offset + 3];

        // data is an array of the 3 bytes that store the stick data
        let first_byte_y = data[0].reverse_bits();
        let second_byte_y = (data[1] & 0b00001111).reverse_bits() >> 4;
        // y_component & x_component are `uint12`s (ranging from 0 > 4096)
        let y_component: u16 = ((first_byte_y as u16) << 4) | (second_byte_y as u16);
        let y: f32 = -(((y_component as f32) - 2048f32) / 2048f32);

        let first_byte_x = (data[1] & 0b11110000).reverse_bits();
        let second_byte_x = data[2].reverse_bits();
        let x_component = ((first_byte_x as u16) << 8) | (second_byte_x as u16);
        let x: f32 = ((x_component as f32) - 2048f32) / 2048f32;
        (x, y)
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
            dpad_north: data.get_bit(SteamInput::DPadUp),
            dpad_east: data.get_bit(SteamInput::DPadRight),
            dpad_west: data.get_bit(SteamInput::DPadLeft),
            dpad_south: data.get_bit(SteamInput::DPadDown),
            face_top_left: data.get_bit(SteamInput::Minus),
            face_bottom_left: data.get_bit(SteamInput::Screenshot),
            face_top_right: data.get_bit(SteamInput::Plus),
            face_bottom_right: data.get_bit(SteamInput::Home),
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

// gilrs: doesnt support switch controllers (directx only)
// sdl2: need to build stuff (nerd alert)
// gamepad: doesnt work
// steamworks: doesnt work

// hidapi: too low level, last resort

/*
	Byte # 	 Remarks
	0 		 Input report ID
	1-3  	 Buttons:
		first byte:
			ZR, ZL, R, L, X, Y, A, B
	4-7 	 Left analog stick data
	8-11  	 Right analog stick data
*/

trait GetBit {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool;
}

impl GetBit for u8 {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool {
        let mask = 1 << n.into();
        (self & mask) == mask
    }
}

impl GetBit for u64 {
    fn get_bit<T: Into<u8>>(&self, n: T) -> bool {
        let mask = 1 << n.into();
        (self & mask) == mask
    }
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

    Minus = 16,
    Plus = 17,
    LStick = 18,
    RStick = 19,
    Home = 20,
    Screenshot = 21,
    Unknown1 = 22,
    Unknown2 = 23,

    // 24-27: Dpad
}

impl Procon {
    // No, I don't know why nintendo did it this way either
    fn decode_dpad(data: u64) -> (bool, bool, bool, bool) {
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
}

impl Into<u8> for Procon {
    fn into(self) -> u8 {
        self as u8
    }
}

struct Controller {
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

    pub l_stick: (u16, u16),
    pub r_stick: (u16, u16),

    pub unknown: Vec<bool>,
}

impl Controller {
    pub fn from_procon_bytes(data: u64) -> Controller {
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
            l_stick: (0, 0),
            r_stick: (0, 0),

            unknown: vec![data.get_bit(Procon::Unknown1), data.get_bit(Procon::Unknown2)],
        }
    }
}

pub fn start() {
    println!("Controller listening started");

    // pro con vendor id: 1406 (0x057E)
    // pro con product id: 8201 (0x2009)

    let api = hidapi::HidApi::new().unwrap();
    let (procon_vid, procon_pid) = (0x057e, 0x2009);
    let device = api.open(procon_vid, procon_pid).unwrap();

    loop {
        let mut buf = [0u8; 12];
        let res = device.read(&mut buf[..]).unwrap();
        let data_arr: &[u8] = &buf[..res];
        let mut data: u64 = 0;
        for byte in data_arr.iter().rev() {
            data = (data << 8) | (*byte as u64);
        }
        let controller = Controller::from_procon_bytes(data);
    }
}

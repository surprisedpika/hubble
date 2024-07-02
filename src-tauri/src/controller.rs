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

#[derive(Clone, Copy)]
enum Procon {
    B = 1 << 08,
    A = 1 << 09,
    Y = 1 << 10,
    X = 1 << 11,
    L = 1 << 12,
    R = 1 << 13,
    Zl = 1 << 14,
    Zr = 1 << 15,

    Minus = 1 << 16,
    Plus = 1 << 17,
    LStick = 1 << 18,
    RStick = 1 << 19,
    Home = 1 << 20,
    Screenshot = 1 << 21,
    Unknown1 = 1 << 22,
    Unknown2 = 1 << 23,

    // 24-27: Dpad
}

impl Procon {
    fn is_pressed(&self, data: u64) -> bool {
        let button = *self as u64;
        return (data & button) == button;
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
        Controller {
            north: Procon::X.is_pressed(data),
            east: Procon::A.is_pressed(data),
            south: Procon::B.is_pressed(data),
            west: Procon::Y.is_pressed(data),

            r_trigger: Procon::Zr.is_pressed(data),
            l_trigger: Procon::Zl.is_pressed(data),
            r_bumper: Procon::R.is_pressed(data),
            l_bumper: Procon::L.is_pressed(data),

            r_stick_click: Procon::RStick.is_pressed(data),
            l_stick_click: Procon::LStick.is_pressed(data),

            unknown: vec![Procon::Unknown1.is_pressed(data), Procon::Unknown2.is_pressed(data)],
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
        println!("{:#064b}", data);
        // 24-27: dpad

        // looking at just 24-27:
        // NO + NO: 0b0001
        // DR + NO: 0b0100
        // DL + NO: 0b0110
        // DU + NO: 0b0000
        // DD + NO: 0b0010
        // DU + DL: 0b1110
        // DU + DR: 0b1000
        // DD + DL: 0b1010
        // DD + DR: 0b1100
    }
}

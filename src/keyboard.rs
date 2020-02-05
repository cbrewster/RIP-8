use minifb::{Key, Window};

pub struct Keyboard {
    pub keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16],
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.keys[0x0] = window.is_key_down(Key::Key1);
        self.keys[0x1] = window.is_key_down(Key::Key2);
        self.keys[0x2] = window.is_key_down(Key::Key3);
        self.keys[0x3] = window.is_key_down(Key::Key4);
        self.keys[0x4] = window.is_key_down(Key::Q);
        self.keys[0x5] = window.is_key_down(Key::W);
        self.keys[0x6] = window.is_key_down(Key::E);
        self.keys[0x7] = window.is_key_down(Key::R);
        self.keys[0x8] = window.is_key_down(Key::A);
        self.keys[0x9] = window.is_key_down(Key::S);
        self.keys[0xA] = window.is_key_down(Key::D);
        self.keys[0xB] = window.is_key_down(Key::F);
        self.keys[0xC] = window.is_key_down(Key::Z);
        self.keys[0xD] = window.is_key_down(Key::X);
        self.keys[0xE] = window.is_key_down(Key::C);
        self.keys[0xF] = window.is_key_down(Key::V);
    }

    pub fn first_pressed_key(&self) -> Option<u8> {
        self.keys.iter().position(|pressed| *pressed).map(|code| code as u8)
    }
}

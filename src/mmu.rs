pub const SCREEN_WIDTH:  i32 = 64;
pub const SCREEN_HEIGHT: i32 = 32;
pub const SCREEN_PIXELS: i32 = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct Mmu {
    data: [u8; 0xFFF],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            data: [0; 0xFFF],
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high_byte = self.read_byte(address) as u16;
        let low_byte = self.read_byte(address) as u16;
        (high_byte << 8) | low_byte
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x200..=0xFFF => self.data[address as usize],
            _ => panic!("Invalid memory accessed!"),
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x200..=0xFFF => self.data[address as usize] = byte,
            _ => panic!("Invalid memory accessed!"),
        }
    }
}

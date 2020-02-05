pub const SCREEN_WIDTH:  usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

const GLYPH_BUF_SIZE: usize = 5*16;
const GLYPH_BUF_LAST_BYTE: u16 = (GLYPH_BUF_SIZE - 1) as u16;

const GLYPHS: [u8; GLYPH_BUF_SIZE] = [
    // 0
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    // 1
    0x20, 0x60, 0x20, 0x20, 0x70,
    // 2
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    // 3
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    // 4
    0x90, 0x90, 0xF0, 0x10, 0x10,
    // 5
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    // 6
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    // 7
    0xF0, 0x10, 0x20, 0x40, 0x40,
    // 8
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    // 9
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    // A
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    // B
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    // C
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    // D
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    // E
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    // F
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Mmu {
    data: [u8; 0xFFF],
    screen: [u32; SCREEN_PIXELS],
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            data: [0; 0xFFF],
            screen: [0; SCREEN_PIXELS],
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high_byte = self.read_byte(address) as u16;
        let low_byte = self.read_byte(address + 1) as u16;
        (high_byte << 8) | low_byte
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x000..=GLYPH_BUF_LAST_BYTE => GLYPHS[address as usize],
            0x200..=0xFFF => self.data[address as usize],
            _ => 0,
        }
    }

    pub fn get_glyph_address(&self, glyph: u8) -> u16 {
        println!("Glyph {}", glyph);
        if glyph > 0xF {
            panic!("Tried to get invalid glyph");
        }
        // Glyphs start at 0x0 and are 5 bytes wide
        glyph as u16 * 5
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        match address {
            0x200..=0xFFF => self.data[address as usize] = byte,
            _ => panic!("Invalid memory write! 0x{:X?}", address),
        }
    }

    pub fn clear_display(&mut self) {
        self.screen = [0; SCREEN_PIXELS];
    }

    pub fn display_buffer(&self) -> &[u32] {
        &self.screen
    }

    pub fn xor_pixel(&mut self, x: u8, y: u8) -> bool {
        let x = x % SCREEN_WIDTH as u8;
        let y = y % SCREEN_HEIGHT as u8;
        let index = y as usize * SCREEN_WIDTH + x as usize;
        if self.screen[index] == 0x00FF00 {
            self.screen[index] = 0x000000;
            true
        } else {
            self.screen[index] = 0x00FF00;
            false
        }
    }
}

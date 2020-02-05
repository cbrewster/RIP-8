use crate::mmu::Mmu;

pub struct Cpu {
    /// The program counter
    pc: u16,
    /// The stack pointer
    sp: u8,

    /// The stack
    stack: [u16; 16],

    /// 16 general purpose 8-bit registers
    v: [u8; 16],
    /// 16-bit register usually used for storing an address
    i: u16,

    /// Delay timer
    dt: u8,
    /// Sound timer
    st: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 200,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        let instruction = mmu.read_word(self.pc);
        self.pc += 2;

        let nnn = instruction & 0x0FFF;
        let n   = (instruction & 0x000F) as u8;
        let x   = ((instruction & 0x0F00) >> 8) as u8;
        let y   = ((instruction & 0x00F0) >> 4) as u8;
        let kk  = (instruction & 0x00FF) as u8;

        let a = (instruction & 0xF000) >> 12;
        let b = (instruction & 0x0F00) >> 8;
        let c = (instruction & 0x00F0) >> 4;
        let d = instruction & 0x000F;


        match (a, b, c, d) {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1,   _,   _,   _) => self.jmp(nnn),
            (0x2,   _,   _,   _) => self.call(nnn),
            (0x3,   _,   _,   _) => self.se(x, kk),
            (0x4,   _,   _,   _) => self.sne(x, kk),
            (0x5,   _,   _, 0x0) => self.se_xy(x, y),
            (0x6,   _,   _,   _) => self.ld(x, kk),
            (0x7,   _,   _,   _) => self.add(x, kk),
            (0x8,   _,   _, 0x0) => self.ld_xy(x, y),
            (0x8,   _,   _, 0x1) => self.or(x, y),
            (0x8,   _,   _, 0x2) => self.and(x, y),
            (0x8,   _,   _, 0x3) => self.xor(x, y),
            (0x8,   _,   _, 0x4) => self.add_xy(x, y),
            (0x8,   _,   _, 0x5) => self.sub(x, y),
            (0x8,   _,   _, 0x6) => self.shr(x),
            (0x8,   _,   _, 0x7) => self.subn(x, y),
            (0x8,   _,   _, 0xE) => self.shl(x),
            (0x9,   _,   _, 0x0) => self.sne_xy(x, y),
            (0xA,   _,   _,   _) => self.addr(nnn),
            (0xB,   _,   _,   _) => self.jmp_v0(nnn),
            (0xC,   _,   _,   _) => self.rnd(x, kk),
            (0xD,   _,   _,   _) => self.drw(x, y, n),
            (0xE,   _, 0x9, 0xE) => self.skp(x),
            (0xE,   _, 0xA, 0x1) => self.skpn(x),
            (0xF,   _, 0x0, 0x7) => self.ld_dt(x),
            (0xF,   _, 0x0, 0xA) => self.ld_key(x),
            (0xF,   _, 0x1, 0x5) => self.set_dt(x),
            (0xF,   _, 0x1, 0x8) => self.set_st(x),
            (0xF,   _, 0x1, 0xE) => self.add_addr(x),
            (0xF,   _, 0x2, 0x9) => self.ld_sprite(x),
            (0xF,   _, 0x3, 0x3) => self.ld_bcd(mmu, x),
            (0xF,   _, 0x5, 0x5) => self.store_regs(mmu, x),
            (0xF,   _, 0x6, 0x5) => self.ld_regs(mmu, x),
            _ => println!("Unexpected instruction {:x?}", instruction),
        }
    }

    fn cls(&mut self) {
        // TODO: Clear screen
    }

    fn ret(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    fn jmp(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn call(&mut self, nnn: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
    }

    fn se(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] == kk {
            self.pc += 2;
        }
    }

    fn sne(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] != kk {
            self.pc += 2;
        }
    }

    fn se_xy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
    }

    fn ld(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
    }

    fn add(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = self.v[x as usize].wrapping_add(kk);
    }

    fn ld_xy(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
    }

    fn or(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
    }

    fn and(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
    }

    fn xor(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];
        if x_val as u16 + y_val as u16 > 0xFF {
            self.v[0xF] = 0x01;
        }
        self.v[x as usize] = x_val.wrapping_add(y_val);
    }

    fn sub(&mut self, x: u8, y: u8) {
        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];
        if x_val > y_val {
            self.v[0xF] = 0x01;
        }
        self.v[x as usize] = x_val.wrapping_sub(y_val);
    }

    fn shr(&mut self, x: u8) {
        let x_val = self.v[x as usize];
        if (x_val & 0x01) == 0x01 {
            self.v[0xF] = 0x01;
        }
        self.v[x as usize] = x >> 1;
    }

    fn subn(&mut self, x: u8, y: u8) {
        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];
        if y_val > x_val {
            self.v[0xF] = 0x01;
        }
        self.v[x as usize] = y_val.wrapping_sub(x_val);
    }

    fn shl(&mut self, x: u8) {
        let x_val = self.v[x as usize];
        if (x_val & 0x80) == 0x80 {
            self.v[0xF] = 0x01;
        }
        self.v[x as usize] = x << 1;
    }

    fn sne_xy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
    }

    fn addr(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn jmp_v0(&mut self, nnn: u16) {
        self.pc = self.v[0x0] as u16 + nnn;
    }

    fn rnd(&mut self, x: u8, kk: u8) {
        // TODO: Generate random number
        let num = 4;
        self.v[x as usize] = num & kk;
    }

    fn drw(&mut self, x: u8, y:u8, n: u8) {
        // TODO: Display something
    }

    fn skp(&mut self, x: u8) {
        // TODO: Check input
    }

    fn skpn(&mut self, x: u8) {
        // TODO: Check input
    }

    fn ld_dt(&mut self, x: u8) {
        self.v[x as usize] = self.dt;
    }

    fn ld_key(&mut self, x: u8) {
        // TODO: Check input
    }

    fn set_dt(&mut self, x: u8) {
        self.dt = self.v[x as usize];
    }

    fn set_st(&mut self, x: u8) {
        self.st = self.v[x as usize];
    }

    fn add_addr(&mut self, x: u8) {
        self.i = self.i.wrapping_add(x as u16);
    }

    fn ld_sprite(&mut self, x: u8) {
        // TODO: Load adress of sprite x into i
    }

    fn ld_bcd(&mut self, mmu: &mut Mmu, x: u8) {
        let hundreds = x / 100;
        let tenths = (x % 100) / 10;
        let ones = x % 10;

        mmu.write_byte(self.i,     hundreds);
        mmu.write_byte(self.i + 1, tenths);
        mmu.write_byte(self.i + 2, ones);
    }

    fn store_regs(&mut self, mmu: &mut Mmu, x: u8) {
        for i in 0..=x {
            mmu.write_byte(self.i + i as u16, self.v[i as usize]);
        }
    }

    fn ld_regs(&mut self, mmu: &mut Mmu, x: u8) {
        for i in 0..=x {
            self.v[i as usize] = mmu.read_byte(self.i + i as u16);
        }
    }
}
use crate::{
    cpu::Cpu,
    mmu::Mmu,
};

pub struct Chip {
    cpu: Cpu,
    mmu: Mmu,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
        }
    }
}

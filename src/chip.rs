use crate::{
    cpu::{Cpu, ExecutionStatus},
    mmu::Mmu,
    keyboard::Keyboard,
};

pub struct Chip {
    pub cpu: Cpu,
    pub mmu: Mmu,
    pub keyboard: Keyboard,
    waiting_for_input: Option<u8>,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
            keyboard: Keyboard::new(),
            waiting_for_input: None,
        }
    }

    pub fn execute(&mut self) {
        if let Some(x) = self.waiting_for_input {
            if let Some(key) = self.keyboard.first_pressed_key() {
                self.waiting_for_input = None;
                self.cpu.provide_key(x, key);
            }
        } else {
            match self.cpu.step(&mut self.mmu, &self.keyboard) {
                ExecutionStatus::WaitForKey(x) => {
                    self.waiting_for_input = Some(x);
                },
                _ => {}
            }
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.waiting_for_input.is_none() {
            self.cpu.decrement_timers();
        }
    }
}

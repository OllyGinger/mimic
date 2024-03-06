use crate::{interruptable::Interruptable, tickable::Tickable};

use super::memory::Memory;
const MEMORY_SIZE: usize = 1024 * 1024; //1MB

pub struct TestMemory {
    memory: Vec<u8>,
    interrupt_flag: u8,
}

impl TestMemory {
    pub fn new() -> TestMemory {
        TestMemory {
            memory: vec![0u8; MEMORY_SIZE],
            interrupt_flag: 0u8,
        }
    }
}

impl Memory for TestMemory {
    fn read8(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Interruptable for TestMemory {
    fn get_interrupt(&self) -> u8 {
        self.interrupt_flag
    }

    fn reset_interrupt(&mut self) {
        self.interrupt_flag = 0u8;
    }
}

impl Tickable for TestMemory {
    fn tick(&mut self) {}
}

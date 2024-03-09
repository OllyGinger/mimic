use super::memory::{Memory, MemoryRangeInclusive};
const MEMORY_SIZE: usize = 1024 * 1024; //1MB

pub struct TestMemory {
    memory: Vec<u8>,
    mapped_ranges: Vec<MemoryRangeInclusive>,
    interrupt_flag: u8,
}

impl TestMemory {
    pub fn new() -> TestMemory {
        TestMemory {
            memory: vec![0u8; MEMORY_SIZE],
            mapped_ranges: vec![0x0000..=MEMORY_SIZE],
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

    fn get_interrupt(&self) -> u8 {
        self.interrupt_flag
    }

    fn reset_interrupt(&mut self) {
        self.interrupt_flag = 0u8;
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

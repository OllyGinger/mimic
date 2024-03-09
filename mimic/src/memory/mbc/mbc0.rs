use crate::memory::memory::{Memory, MemoryRangeInclusive};

pub struct MBC0 {
    rom: Vec<u8>,
    mapped_ranges: Vec<MemoryRangeInclusive>,
}

pub fn new(data: Vec<u8>) -> MBC0 {
    let data_len = data.len();
    MBC0 {
        rom: data,
        mapped_ranges: vec![0x0000..=data_len - 1],
    }
}

impl MBC0 {}

impl Memory for MBC0 {
    fn read8(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
    fn write8(&mut self, address: u16, value: u8) {
        self.rom[address as usize] = value
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

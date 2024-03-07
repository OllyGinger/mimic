use crate::memory::memory::Memory;

pub struct MBC0 {
    rom: Vec<u8>,
}

pub fn new(data: Vec<u8>) -> MBC0 {
    MBC0 { rom: data }
}

impl MBC0 {}

impl Memory for MBC0 {
    fn read8(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
    fn write8(&mut self, address: u16, value: u8) {
        self.rom[address as usize] = value
    }
}
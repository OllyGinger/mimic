use crate::{
    cartridge,
    memory::memory::{Memory, MemoryRangeInclusive},
};

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank: usize,

    ram: Vec<u8>,
    ram_bank: usize,
    ram_enabled: bool,
    ram_mode: bool,

    mapped_ranges: Vec<MemoryRangeInclusive>,
}

pub fn new(data: Vec<u8>) -> MBC1 {
    let ram_size = cartridge::parse_ram_size(data[0x149]);

    MBC1 {
        rom: data,
        rom_bank: 1,

        ram: std::iter::repeat(0u8).take(ram_size).collect(),
        ram_bank: 0,
        ram_enabled: false,
        ram_mode: false,

        mapped_ranges: vec![
            0x0000..=0x7fff, // ROM banks
            0xA000..=0xbfff, // RAM banks
            0xff50..=0xff50, // Boot rom enable
        ],
    }
}

impl MBC1 {}

impl Memory for MBC1 {
    fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            // ROM Banks
            0x0000..=0x7fff => {
                let addr = if address < 0x4000 {
                    address as usize
                } else {
                    self.rom_bank * 0x4000 | (address as usize & 0x3fff)
                };

                return Some(self.rom[addr]);
            }
            // RAM Banks
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let bank = if self.ram_mode { self.ram_bank } else { 0 };
                    return Some(self.ram[bank * 0x2000 | (address as usize & 0x1fff)]);
                } else {
                    return Some(0);
                }
            }
            _ => {
                return None;
            }
        };
    }
    fn read8(&self, address: u16) -> u8 {
        if let Some(x) = self.try_read8(address) {
            x
        } else {
            panic!("Unmapped mbc address: {:#06X}", address)
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            // RAM Enable
            0x0000..=0x1fff => {
                self.ram_enabled = value & 0x0F == 0x0A;
            }
            0x2000..=0x3fff => {
                self.rom_bank = (self.rom_bank & 0x60) | (value as usize & 0x1F);
            }
            0x4000..=0x5fff => {
                self.rom_bank = (self.rom_bank & 0x1F) | ((value as usize & 0x03) << 5);
            }
            0x6000..=0x7fff => {
                self.ram_mode = value & 0x01 == 0x01;
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    let bank = if self.ram_mode { self.ram_bank } else { 0 };
                    self.ram[bank * 0x2000 | (address as usize & 0x1fff)] = value;
                }
            }
            _ => {
                panic!("Unmapped mbc1 address: {:#06X}", address);
            }
        }
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

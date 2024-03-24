use crate::{memory::memory::Memory, tickable::Tickable};
use bitflags::bitflags;

pub struct Serial {
    mapped_ranges: Vec<crate::memory::memory::MemoryRangeInclusive>,
    string_buffer: String,
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            mapped_ranges: vec![
                0xff01..=0xff02, // Serial
            ],
            string_buffer: String::new(),
        }
    }
}

impl Memory for Serial {
    fn try_read8(&self, address: u16) -> Option<u8> {
        Some(0)
    }

    fn read8(&self, address: u16) -> u8 {
        self.try_read8(address)
            .expect(&format!("Unmapped address: {:#06X}", address))
    }

    fn write8(&mut self, address: u16, value: u8) {}

    fn mapped_ranges(&self) -> &Vec<crate::memory::memory::MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

impl Tickable for Serial {
    fn tick(&mut self, _cycles: u8) {}
}

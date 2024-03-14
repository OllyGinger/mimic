use std::ops::RangeInclusive;

use crate::tickable::Tickable;

pub type MemoryRangeInclusive = RangeInclusive<usize>;

pub trait Memory {
    fn read8(&self, address: u16) -> u8;
    fn write8(&mut self, address: u16, value: u8);

    fn read16(&self, address: u16) -> u16 {
        (self.read8(address) as u16) | ((self.read8(address + 1) as u16) << 8)
    }
    fn write16(&mut self, address: u16, value: u16) {
        self.write8(address, (value & 0xFF) as u8);
        self.write8(address + 1, ((value >> 8) & 0xFF) as u8);
    }

    fn get_interrupt(&self) -> u8 {
        0
    }
    fn reset_interrupt(&mut self) {}

    /// Get a list of memory mapped ranges that are supported by this object
    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive>;
}

pub trait TickableMemory: Memory + Tickable {}
impl<T: Memory + Tickable> TickableMemory for T {}

use crate::{
    memory::memory::{Memory, MemoryRangeInclusive},
    tickable::Tickable,
};

pub struct Timer {
    mapped_ranges: Vec<MemoryRangeInclusive>,

    divider: u8,
    counter: u8,
    modulo: u8,
    enabled: bool,
    step: u32,
    internal_counter: u32,
    internal_divider: u32,
    pub interrupt: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            mapped_ranges: vec![0xFF04..=0xFF07],
            divider: 0,
            counter: 0,
            modulo: 0,
            enabled: false,
            step: 256,
            internal_counter: 0,
            internal_divider: 0,
            interrupt: 0,
        }
    }
}

impl Memory for Timer {
    fn read8(&self, address: u16) -> u8 {
        self.try_read8(address)
            .expect(&format!("Unmapped address: {:#06X}", address))
    }

    fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            0xFF04 => Some(self.divider),
            0xFF05 => Some(self.counter),
            0xFF06 => Some(self.modulo),
            0xFF07 => {
                let value = (if self.enabled { 0x4 } else { 0 })
                    | (match self.step {
                        16 => 1,
                        64 => 2,
                        256 => 3,
                        _ => 0,
                    });

                Some(value)
            }
            _ => panic!("Un-mapped timer address: {:06x}", address),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => {
                self.divider = 0;
            }
            0xFF05 => {
                self.counter = value;
            }
            0xFF06 => {
                self.modulo = value;
            }
            0xFF07 => {
                self.enabled = value & 0x4 != 0;
                self.step = match value & 0x3 {
                    1 => 16,
                    2 => 64,
                    3 => 256,
                    _ => 1024,
                };
            }
            _ => panic!("Un-mapped timer address: {:06x}", address),
        };
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

impl Tickable for Timer {
    fn tick(&mut self, ticks: u8) {
        self.internal_divider += ticks as u32;
        while self.internal_divider >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.internal_divider -= 256;
        }

        if self.enabled {
            self.internal_counter += ticks as u32;

            while self.internal_counter >= self.step {
                self.counter = self.counter.wrapping_add(1);
                if self.counter == 0 {
                    self.counter = self.modulo;
                    self.interrupt |= 0x04;
                }
                self.internal_counter -= self.step;
            }
        }
    }
}

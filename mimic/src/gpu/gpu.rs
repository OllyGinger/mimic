use crate::memory::memory::Memory;

const VRAM_SIZE: usize = 0x4000;

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    vram_bank: usize,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            vram_bank: 1,
        }
    }
}

impl Memory for GPU {
    fn read8(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            _ => panic!("Unmapped GPU address: {:#06x}", address),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9fff => {
                self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = value
            }
            _ => panic!("Unmapped GPU address: {:#06x}", address),
        }
    }
}

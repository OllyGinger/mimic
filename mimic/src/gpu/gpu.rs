use super::{lcd::LCD, palette::Palette};
use crate::memory::memory::{Memory, MemoryRangeInclusive};

const VRAM_SIZE: usize = 0x4000;

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    vram_bank: usize,

    // LCD Control
    lcd: LCD,

    // GPU Control
    bg_palette: Palette,

    // Internals
    mapped_ranges: Vec<MemoryRangeInclusive>,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            vram_bank: 1,

            // LCD Control
            lcd: LCD::new(),

            // GPU Control
            bg_palette: Palette::new(),

            // Internals
            mapped_ranges: vec![
                0x8000..=0x9FFF, // VRam
                0xFF40..=0xFF4B, // GPU/LCD Control
            ],
        }
    }
}

impl Memory for GPU {
    fn read8(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],

            // LCD Control
            0xff40 => self.lcd.lcd_control.bits(),
            0xff42 => self.lcd.scy,
            0xff43 => self.lcd.scx,
            0xFF44 => self.lcd.ly,

            // GPU Control
            0xFF47 => self.bg_palette.get_bits(),

            _ => panic!("Unmapped GPU address: {:#06x}", address),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9fff => {
                self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = value
            }

            // LCD Control
            0xff40 => self.lcd.update_control(value),
            0xff42 => self.lcd.scy = value,
            0xff43 => self.lcd.scx = value,
            0xff44 => panic!("Attempted to write to LY register (0xFF44). Read only."),

            // GPU Control
            0xFF47 => self.bg_palette.update(value),

            _ => panic!("Unmapped GPU address: {:#06x}", address),
        }
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

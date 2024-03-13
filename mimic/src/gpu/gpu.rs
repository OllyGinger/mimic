use super::palette::Palette;
use crate::memory::memory::{Memory, MemoryRangeInclusive};
use bitflags::bitflags;

const VRAM_SIZE: usize = 0x4000;

bitflags!(
    pub struct LCDControl: u8 {
        const BG_ON =           0b_0000_0001;
        const OBJ_ON =          0b_0000_0010;
        const OBJ_SIZE =        0b_0000_0100;
        const BG_MAP =          0b_0000_1000;
        const BG_ADDR =         0b_0001_0000;
        const WINDOW_ON =       0b_0010_0000;
        const WINDOW_MAP =      0b_0100_0000;
        const LCD_ON =          0b_1000_0000;
    }
);

bitflags!(
    pub struct LCDStat: u8 {
        const MODE =            0b_0000_0011; // 2 bits
        const LYC_COMPARE =     0b_0000_0100;
        const HBLANK_INT =      0b_0000_1000;
        const VBLANK_INT =      0b_0001_0000;
        const ACCESS_OAM_INT =  0b_0010_0000;
        const LYC_INT =         0b_0100_0000;

        const UNUSED =          0b_1000_0000;
    }
);

#[derive(PartialEq)]
enum Mode {
    AccessOAM,
    AccessVRam,
    HBlank,
    VBlank,
}

const ACCESS_OAM_CYCLES: isize = 21;
const ACCESS_VRAM_CYCLES: isize = 43;
const HBLANK_CYCLES: isize = 50;
const VBLANK_LINE_CYCLES: isize = 114;

impl Mode {
    fn bits(&self) -> u8 {
        match *self {
            Mode::AccessOAM => 2,
            Mode::AccessVRam => 3,
            Mode::HBlank => 0,
            Mode::VBlank => 1,
        }
    }

    fn cycles(&self, sx: u8) -> isize {
        match *self {
            Mode::AccessOAM => ACCESS_OAM_CYCLES,
            Mode::AccessVRam => ACCESS_VRAM_CYCLES + sx as isize,
            Mode::HBlank => HBLANK_CYCLES - sx as isize,
            Mode::VBlank => VBLANK_LINE_CYCLES,
        }
    }
}

pub struct GPU {
    mode: Mode,
    cycles: isize,

    // Memory
    vram: [u8; VRAM_SIZE],
    vram_bank: usize,

    // GPU Control
    lcd_status: LCDStat, // LCD Status
    lcd_control: LCDControl,
    scy: u8, // Scroll viewport Y
    scx: u8, // Scroll viewport X
    ly: u8,  // Current line
    lyc: u8, // LY compare

    window_pos_y: u8, // WY, WX: Window Y position, X position plus 7
    window_pos_x: u8, // WY, WX: Window Y position, X position plus 7
    window_pos_y_triggered: bool,

    bg_palette: Palette,

    // Internals
    mapped_ranges: Vec<MemoryRangeInclusive>,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            mode: Mode::AccessOAM,
            cycles: 0,

            // Memory
            vram: [0; VRAM_SIZE],
            vram_bank: 1,

            // LCD Control
            lcd_status: LCDStat::empty(),
            lcd_control: LCDControl::empty(),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            window_pos_y: 0,
            window_pos_x: 0,
            window_pos_y_triggered: false,

            // GPU Control
            bg_palette: Palette::new(),

            // Internals
            mapped_ranges: vec![
                0x8000..=0x9FFF, // VRam
                0xFF40..=0xFF4B, // GPU/LCD Control
            ],
        }
    }

    pub fn set_lcd_control(&mut self, control: u8) {
        let new_control = LCDControl::from_bits_truncate(control);

        // LCD turning off
        if !new_control.contains(LCDControl::LCD_ON)
            && self.lcd_control.contains(LCDControl::LCD_ON)
        {
            self.ly = 0;
            self.window_pos_y_triggered = false;
        }

        // LCD turning on
        if new_control.contains(LCDControl::LCD_ON)
            && !self.lcd_control.contains(LCDControl::LCD_ON)
        {
            self.mode = Mode::HBlank;
            self.lcd_status.insert(LCDStat::LYC_COMPARE);
            self.cycles = Mode::AccessOAM.cycles(self.scx);
        }

        self.lcd_control = new_control;
    }
}

impl Memory for GPU {
    fn read8(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9fff => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],

            // LCD Control
            0xff40 => self.lcd_control.bits(),
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xFF44 => self.ly,

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
            0xff40 => self.set_lcd_control(value),
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
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

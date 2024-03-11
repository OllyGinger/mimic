use bitflags::bitflags;

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

pub struct LCD {
    mode: Mode,
    cycles: isize,

    pub stat: LCDStat, // LCD Status
    pub lcd_control: LCDControl,
    pub scy: u8, // Scroll viewport Y
    pub scx: u8, // Scroll viewport X
    pub ly: u8,  // Current line

    wy_triggered: bool,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            mode: Mode::AccessOAM,
            cycles: 0,

            stat: LCDStat::empty(),
            lcd_control: LCDControl::empty(),
            scy: 0,
            scx: 0,
            ly: 0,

            wy_triggered: false,
        }
    }

    pub fn update_control(&mut self, control: u8) {
        let new_control = LCDControl::from_bits_truncate(control);

        // LCD turning off
        if !new_control.contains(LCDControl::LCD_ON)
            && self.lcd_control.contains(LCDControl::LCD_ON)
        {
            self.ly = 0;
            self.wy_triggered = false;
        }

        // LCD turning on
        if new_control.contains(LCDControl::LCD_ON)
            && !self.lcd_control.contains(LCDControl::LCD_ON)
        {
            self.mode = Mode::HBlank;
            self.stat.insert(LCDStat::LYC_COMPARE);
            self.cycles = Mode::AccessOAM.cycles(self.scx);
        }

        self.lcd_control = new_control;
    }
}

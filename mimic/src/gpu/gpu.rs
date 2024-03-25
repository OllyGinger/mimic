use super::{
    palette::{Palette, PaletteColour},
    BackBufferTexture, PixelColour, BACK_BUFFER_TILES_WIDE, FRAME_BUFFER_TEXTURE_HEIGHT,
    FRAME_BUFFER_TEXTURE_WIDTH, LCD_SCREEN_WIDTH, TILE_SIZE,
};
use crate::{
    cpu::interrupts,
    int_utils::IntExt,
    memory::memory::{Memory, MemoryRangeInclusive},
    tickable::Tickable,
    utils,
};
use bitflags::bitflags;

const VRAM_SIZE: usize = 0x4000;
const VRAM_OAM_SIZE: usize = 0xA0;

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
    AccessOAM,  // Mode 2 - OAM Scan. Access: VRAM CGB Palettes
    AccessVRam, // Mode 3 - Drawing pixels. Access: None
    HBlank,     // Mode 0 - End of a scanline. Access: All
    VBlank,     // Mode 1 - End of a frame. Access: All
}

const ACCESS_OAM_CYCLES: isize = 21;
const ACCESS_VRAM_CYCLES: isize = 43;
const HBLANK_CYCLES: isize = 50;
const VBLANK_LINE_CYCLES: isize = 114;

const RENDER_LINE_COUNT: u8 = 144;
const MAX_VBLANK_LINE: u8 = 153;

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
    interrupt_request: u8,

    // Memory
    vram: [u8; VRAM_SIZE],
    vram_bank: usize,
    vram_oam: [u8; VRAM_OAM_SIZE],
    dma_control: u8,

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
    obj_palette_0: Palette, // OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data
    obj_palette_1: Palette, // OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data

    // Internals
    mapped_ranges: Vec<MemoryRangeInclusive>,
    back_buffer: BackBufferTexture, // Used to draw the screen line by line
    current_window_line: i32, // The line which the window is rendering so it can continue if the pos is changed
}

impl GPU {
    pub fn new() -> GPU {
        let mut gpu = GPU {
            mode: Mode::AccessOAM,
            cycles: 0,
            interrupt_request: 0,
            dma_control: 0,

            // Memory
            vram: [0; VRAM_SIZE],
            vram_bank: 1,
            vram_oam: [0; VRAM_OAM_SIZE],

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
            obj_palette_0: Palette::new(),
            obj_palette_1: Palette::new(),

            // Internals
            mapped_ranges: vec![
                0x8000..=0x9FFF, // VRam
                0xFE00..=0xFE9F, // OAM
                0xFF40..=0xFF4B, // GPU/LCD Control
            ],
            back_buffer: [(0u8, 0u8, 0u8);
                FRAME_BUFFER_TEXTURE_HEIGHT * FRAME_BUFFER_TEXTURE_WIDTH],
            current_window_line: 0,
        };

        gpu.back_buffer
            .fill(gpu.bg_palette.get_pixel_color(PaletteColour::White));
        gpu
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

    fn scanline(self: &mut GPU) {
        if !self.lcd_control.contains(LCDControl::LCD_ON) {
            self.back_buffer
                .fill(self.bg_palette.get_pixel_color(PaletteColour::White));
            return;
        }

        // Check if the background is enabled, if not then the
        // background should just be white
        if self.lcd_control.contains(LCDControl::BG_ON) {
            self.draw_bg_scanline();
        }

        //if self.lcd_control.contains(LCDControl::WINDOW_ON) {
        //    self.draw_window_scanline();
        //}
        //
        //if self.lcd_control.contains(LCDControl::OBJ_ON) {
        //    self.draw_sprites_scanline();
        //}
    }

    // Draw the background to the backbuffer. This is done
    // once per HSync. By the time we get to the VSync this
    // whole screen should be drawn
    fn draw_bg_scanline(self: &mut GPU) {
        let slice_start = LCD_SCREEN_WIDTH * self.ly as usize;
        let slice_end = LCD_SCREEN_WIDTH + slice_start;

        let mut tile_data_base: u16 = if self.lcd_control.contains(LCDControl::BG_ADDR) {
            0x8000
        } else {
            0x9000
        };

        // Pick the addressing mode
        let tile_map_base: u16 = if self.lcd_control.contains(LCDControl::BG_MAP) {
            0x9C00
        } else {
            0x9800
        };

        let addr_mode_8000 = tile_data_base == 0x8000;
        let palette = &self.bg_palette;
        let y = self.ly.wrapping_add(self.scy);
        let row = (y / 8) as usize;
        for i in 0..LCD_SCREEN_WIDTH {
            let x = (i as u8).wrapping_add(self.scx);
            let col = (x / 8) as usize;

            // Read which tile sits at this position in vram
            let tile_index = self
                .read8(tile_map_base + ((BACK_BUFFER_TILES_WIDE as u16 * row as u16) + col as u16));

            // Shift the memory base back to 0x8000 to reference the second block of tiles
            if !addr_mode_8000 && tile_index >= 128 {
                tile_data_base = 0x8000;
            }

            // Read the specific pixel from the tile data
            let mut tile_pixel_data_address =
                tile_data_base + (tile_index as u16 * (TILE_SIZE * 2) as u16);
            tile_pixel_data_address += ((y as u8 % TILE_SIZE as u8) * 2) as u16;
            let tile_data = utils::from_u16(self.read16(tile_pixel_data_address));
            let bit = (x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;

            let colour_idx = ((tile_data[1] as u8).bit(bit) << 1) | (tile_data[0] as u8).bit(bit);

            let pixels = &mut self.back_buffer[slice_start..slice_end];
            pixels[i] = palette.get_pixel_color(PaletteColour::from_u8(colour_idx));
        }
    }

    fn change_mode(self: &mut GPU, mode: Mode) {
        self.mode = mode;
        self.cycles += self.mode.cycles(self.scx);
        if match self.mode {
            Mode::HBlank => self.lcd_status.contains(LCDStat::HBLANK_INT),
            Mode::VBlank => {
                self.window_pos_y_triggered = false;
                self.interrupt_request |= interrupts::InterruptFlag::VBLANK.bits(); // Vblank int
                self.lcd_status.contains(LCDStat::VBLANK_INT)
            }
            Mode::AccessOAM => {
                if self.lcd_control.contains(LCDControl::WINDOW_ON)
                    && !self.window_pos_y_triggered
                    && self.ly == self.window_pos_y
                {
                    self.window_pos_y_triggered = true;
                    self.current_window_line = -1;
                }
                self.lcd_status.contains(LCDStat::ACCESS_OAM_INT)
            }
            _ => false,
        } {
            self.interrupt_request |= interrupts::InterruptFlag::LCD.bits(); // Stat interrrupt
        }
    }

    pub fn get_bg_tile_as_pixels(
        self: &GPU,
        tile_num: u16,
    ) -> [PixelColour; TILE_SIZE * TILE_SIZE] {
        let mut tile_data = [(0, 0, 0); TILE_SIZE * TILE_SIZE];
        let mut address = 0x8000_u16 + (tile_num as u16 * (TILE_SIZE * 2) as u16);

        for y in 0..TILE_SIZE {
            let row_data = utils::from_u16(self.read16(address));
            for x in 0..TILE_SIZE {
                let mask = 0x80 >> x;
                let a = if ((row_data[0] & mask) as u8) > 0 {
                    1
                } else {
                    0
                };
                let b = if ((row_data[1] & mask) as u8) > 0 {
                    1
                } else {
                    0
                };
                let val = ((b << 1) | a) as u8;

                // TODO: Only uses BG palette
                tile_data[TILE_SIZE * y + x] =
                    self.bg_palette.get_pixel_color(PaletteColour::from_u8(val));
            }

            address += 2;
        }

        tile_data
    }

    pub fn get_scx(self: &GPU) -> u8 {
        self.scx
    }

    pub fn get_scy(self: &GPU) -> u8 {
        self.scy
    }

    pub fn get_back_buffer(&self) -> &BackBufferTexture {
        &self.back_buffer
    }

    fn check_lyc_interrupt(&mut self) {
        if self.ly != self.lyc {
            self.lcd_status.remove(LCDStat::LYC_COMPARE);
        } else {
            self.lcd_status.insert(LCDStat::LYC_COMPARE);
            if self.lcd_status.contains(LCDStat::LYC_INT) {
                self.interrupt_request |= interrupts::InterruptFlag::LCD.bits();
            }
        }
    }

    fn get_lcd_stat(&self) -> u8 {
        if !self.lcd_control.contains(LCDControl::LCD_ON) {
            LCDStat::UNUSED.bits
        } else {
            self.mode.bits() | self.lcd_status.bits | LCDStat::UNUSED.bits
        }
    }

    fn set_lcd_stat(&mut self, value: u8) {
        let new_stat = LCDStat::from_bits_truncate(value);
        self.lcd_status = (self.lcd_status & LCDStat::LYC_COMPARE)
            | (new_stat & LCDStat::HBLANK_INT)
            | (new_stat & LCDStat::VBLANK_INT)
            | (new_stat & LCDStat::ACCESS_OAM_INT)
            | (new_stat & LCDStat::LYC_INT)
    }
}

impl Memory for GPU {
    fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            0x8000..=0x9fff => {
                Some(self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)])
            }
            0xFE00..=0xFE9F => Some(self.vram_oam[address as usize - 0xFE00]),

            // LCD Control
            0xff40 => Some(self.lcd_control.bits()),
            0xff42 => Some(self.scy),
            0xff43 => Some(self.scx),
            0xFF44 => Some(self.ly),
            0xFF45 => Some(self.lyc),
            0xff46 => Some(self.dma_control),
            // Palettes
            0xFF47 => Some(self.bg_palette.get_bits()),
            0xff48 => Some(self.obj_palette_0.get_bits()),
            0xff49 => Some(self.obj_palette_1.get_bits()),

            // Window
            0xFF4A => Some(self.window_pos_y),
            0xFF4B => Some(self.window_pos_x),

            _ => None,
        }
    }

    fn read8(&self, address: u16) -> u8 {
        if let Some(x) = self.try_read8(address) {
            x
        } else {
            panic!("Unmapped GPU address: {:#06X}", address)
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9fff => {
                self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = value
            }
            0xFE00..=0xFE9F => self.vram_oam[address as usize - 0xFE00] = value,

            // LCD Control
            0xff40 => self.set_lcd_control(value),
            0xff41 => self.set_lcd_stat(value),
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => panic!("Attempted to write to LY register (0xFF44). Read only."),
            0xff45 => self.lyc = value,
            0xff46 => self.dma_control = value,

            // Palettes
            0xFF47 => self.bg_palette.update(value),
            0xff48 => self.obj_palette_0.update(value),
            0xff49 => self.obj_palette_1.update(value),

            // Window
            0xff4a => self.window_pos_y = value,
            0xff4b => self.window_pos_x = value,

            _ => panic!("Unmapped GPU address: {:#06x}", address),
        }
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }

    fn get_interrupt(&self) -> u8 {
        self.interrupt_request
    }

    fn reset_interrupt(&mut self) {
        self.interrupt_request = 0x00;
    }
}

impl Tickable for GPU {
    fn tick(&mut self, _cycles: u8) {
        self.cycles -= 1;
        if self.cycles == 1 && self.mode == Mode::AccessVRam {
            if self.lcd_status.contains(LCDStat::HBLANK_INT) {
                self.interrupt_request |= interrupts::InterruptFlag::LCD.bits();
            }
        }
        if self.cycles > 0 {
            return;
        }

        match self.mode {
            Mode::AccessOAM => self.change_mode(Mode::AccessVRam),
            Mode::AccessVRam => {
                self.scanline();
                self.change_mode(Mode::HBlank);
            }
            Mode::HBlank => {
                self.ly += 1;
                if self.ly < RENDER_LINE_COUNT {
                    self.change_mode(Mode::AccessOAM);
                } else {
                    self.change_mode(Mode::VBlank);
                }
                self.check_lyc_interrupt();
            }
            Mode::VBlank => {
                self.ly += 1;
                if self.ly > MAX_VBLANK_LINE {
                    // End of the frame, ready to start next
                    self.ly = 0;
                    self.change_mode(Mode::AccessOAM);
                } else {
                    self.cycles += VBLANK_LINE_CYCLES
                }
                self.check_lyc_interrupt();
            }
        }
    }
}

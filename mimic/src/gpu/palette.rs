use super::PixelColour;

#[derive(Copy, Clone)]
pub enum PaletteColour {
    White,
    Light,
    Dark,
    Black,
}

impl PaletteColour {
    pub fn from_u8(value: u8) -> PaletteColour {
        match value {
            1 => PaletteColour::Light,
            2 => PaletteColour::Dark,
            3 => PaletteColour::Black,
            _ => PaletteColour::White,
        }
    }
}

pub struct Palette {
    white: PaletteColour,
    light: PaletteColour,
    dark: PaletteColour,
    black: PaletteColour,
    bits: u8,
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            white: PaletteColour::Black,
            light: PaletteColour::Black,
            dark: PaletteColour::Black,
            black: PaletteColour::Black,
            bits: 0xff,
        }
    }

    pub fn get_bits(&self) -> u8 {
        self.bits
    }

    pub fn get_pixel_color(&self, color: PaletteColour) -> PixelColour {
        // Lighter Colors
        //const COLOURS: [PixelColour; 4] = [
        //    (0xe0, 0xf8, 0xd0), // White
        //    (0x88, 0xc0, 0x70), // Light grey
        //    (0x34, 0x68, 0x56), // Dark grey
        //    (0x08, 0x18, 0x20), // Black
        //];

        const COLOURS: [PixelColour; 4] = [
            (0x9b, 0xbc, 0x0f), // White
            (0x8b, 0xac, 0x0f), // Light grey
            (0x30, 0x62, 0x30), // Dark grey
            (0x0f, 0x38, 0x0f), // Black
        ];

        match color {
            PaletteColour::Black => COLOURS[self.black as usize],
            PaletteColour::Light => COLOURS[self.light as usize],
            PaletteColour::Dark => COLOURS[self.dark as usize],
            PaletteColour::White => COLOURS[self.white as usize],
        }
    }

    pub fn update(&mut self, value: u8) {
        self.white = PaletteColour::from_u8((value >> 0) & 0x3);
        self.light = PaletteColour::from_u8((value >> 2) & 0x3);
        self.dark = PaletteColour::from_u8((value >> 4) & 0x3);
        self.black = PaletteColour::from_u8((value >> 6) & 0x3);
        self.bits = value;
    }
}

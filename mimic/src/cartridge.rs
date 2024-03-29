use log::warn;

use crate::memory::mbc::{self, MbcType};
use crate::memory::memory::{Memory, MemoryRangeInclusive};
use crate::tickable::Tickable;

#[derive(Debug, PartialEq)]
pub enum CGBMode {
    None,
    SupportsCGB,
    RequiresCGB,
}

#[derive(Debug)]
pub struct Header {
    pub entry_point: [u8; 4],
    title: String,
    pub mbc_type: MbcType,
    pub rom_size: usize,
    ram_size: usize,
    pub cgb_mode: CGBMode,
    pub header_checksum: u8,
}

pub struct Cartridge {
    pub header: Header,
    pub mbc: Box<dyn Memory>,
    pub boot_rom_data: Option<Vec<u8>>,
    pub boot_rom_enabled: bool,
}

pub fn new(boot_rom_data: Option<Vec<u8>>, cart_data: Vec<u8>) -> Cartridge {
    let header = parse_header(&cart_data);
    let mbc = mbc::new(header.mbc_type, cart_data.clone());
    Cartridge {
        header: header,
        mbc: mbc,
        boot_rom_data: boot_rom_data.clone(),
        boot_rom_enabled: boot_rom_data.is_some(),
    }
}

fn parse_header(cart_data: &Vec<u8>) -> Header {
    // Entry point
    let entry_point = cart_data[0x00..0x04].try_into().unwrap();

    // Title
    let title_bytes = cart_data[0x134..0x143].to_vec();
    let title = String::from_utf8(title_bytes).unwrap();

    let cgb_mode = match cart_data[0x143] {
        0x00 => CGBMode::None,
        0x80 => CGBMode::SupportsCGB,
        0xC0 => CGBMode::RequiresCGB,
        _ => panic!("Unknown CGB flag {:02x}", cart_data[0x143]),
    };

    // $0147 - Cartridge Type
    let mbc_type = match MbcType::try_from(cart_data[0x147]) {
        Ok(t) => t,
        Err(_) => {
            panic!("Failed to parse cartridge type: {}", cart_data[0x147]);
        }
    };

    // Mem sizes
    let rom_size = parse_rom_size(cart_data[0x0148]);
    let ram_size = parse_ram_size(cart_data[0x0149]);

    Header {
        entry_point,
        title,
        mbc_type,
        rom_size,
        ram_size,
        cgb_mode,
        header_checksum: 0,
    }
}

fn parse_rom_size(val: u8) -> usize {
    (32 * 1024) * (1 << val)
}

pub fn parse_ram_size(val: u8) -> usize {
    match val {
        0x0 => 0,
        0x2 => 8 * 1024,
        0x3 => 32 * 1024,
        0x4 => 128 * 1024,
        0x5 => 64 * 1024,
        _ => {
            warn!(target: "cartridge", "Invalid RAM size while parsing cart header: {}", val);
            0
        }
    }
}

impl Cartridge {}

impl Memory for Cartridge {
    fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            0xff50 => {
                if self.boot_rom_enabled {
                    return Some(0x00);
                } else {
                    return Some(0x01);
                }
            }
            _ => {}
        }

        // Read the boot rom if it exists
        if let Some(boot_rom_data) = &self.boot_rom_data {
            match address {
                0x0000..=0x00FF if self.boot_rom_enabled => Some(boot_rom_data[address as usize]),
                _ => self.mbc.try_read8(address),
            }
        } else {
            self.mbc.try_read8(address)
        }
    }

    fn read8(&self, address: u16) -> u8 {
        self.try_read8(address)
            .expect(&format!("Unmapped address: {:#06X}", address))
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0xff50 => self.boot_rom_enabled = value == 0x00,
            _ => {
                self.mbc.write8(address, value);
            }
        }
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        self.mbc.mapped_ranges()
    }
}

impl Tickable for Cartridge {
    fn tick(&mut self, _cycles: u8) {}
}

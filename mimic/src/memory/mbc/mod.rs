pub mod mbc0;
pub mod mbc1;

use super::memory::Memory;

pub trait MBC: Memory {}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum MbcType {
    MBC0 = 0,
    MBC1 = 1,
    // 2
    //MBC3 = 3,
}

impl TryFrom<u8> for MbcType {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            x if x == MbcType::MBC0 as u8 => Ok(MbcType::MBC0),
            x if x == MbcType::MBC1 as u8 => Ok(MbcType::MBC1),
            //x if x == MbcType::MBC3 as u8 => Ok(MbcType::MBC3),
            _ => Err("Unknown MbcType"),
        }
    }
}

pub fn new(mbc_type: MbcType, data: Vec<u8>) -> Box<dyn Memory> {
    match mbc_type {
        MbcType::MBC0 => Box::new(mbc0::new(data)),
        MbcType::MBC1 => Box::new(mbc1::new(data)),
        //MbcType::MBC3 => Box::new(MBC3::new(data)),
    }
}

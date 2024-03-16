use crate::{memory::memory::Memory, tickable::Tickable};
use bitflags::bitflags;

bitflags!(
     struct MasterControl: u8 {
        const CH0_ENABLED  = 0b_0000_0001; // ReadOnly
        const CH1_ENABLED  = 0b_0000_0010; // ReadOnly
        const CH2_ENABLED  = 0b_0000_0100; // ReadOnly
        const CH3_ENABLED  = 0b_0000_1000; // ReadOnly
        const AUDIO_ENABLE = 0b_1000_0000;
    }
);

pub struct Audio {
    mapped_ranges: Vec<crate::memory::memory::MemoryRangeInclusive>,

    master_control: MasterControl,
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            mapped_ranges: vec![
                0xff10..=0xff26, // Audio
                0xff30..=0xff3f, // Wave Pattern
            ],
            master_control: MasterControl::empty(),
        }
    }

    pub fn set_master_control(&mut self, value: u8) {
        let mut new_master_control = MasterControl::from_bits_truncate(value);

        // Channel enables are read-only. Don't allow them to be set
        new_master_control.remove(MasterControl::CH0_ENABLED);
        new_master_control.remove(MasterControl::CH1_ENABLED);
        new_master_control.remove(MasterControl::CH2_ENABLED);
        new_master_control.remove(MasterControl::CH3_ENABLED);

        self.master_control = MasterControl::from_bits_truncate(value);
    }
}

impl Memory for Audio {
    fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            0xff26 => Some(self.master_control.bits),

            //_ => panic!("Unmapped audio address: {:#06x}", address),
            _ => Some(0),
        }
    }

    fn read8(&self, address: u16) -> u8 {
        self.try_read8(address)
            .expect(&format!("Unmapped address: {:#06X}", address))
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0xff26 => self.set_master_control(value),

            //_ => panic!("Unmapped audio address: {:#06x}", address),
            _ => {}
        }
    }

    fn mapped_ranges(&self) -> &Vec<crate::memory::memory::MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

impl Tickable for Audio {
    fn tick(&mut self, _cycles: u8) {}
}

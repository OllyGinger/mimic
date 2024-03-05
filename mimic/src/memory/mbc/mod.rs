use crate::Memory;

pub mod mbc0;

pub trait MBC: Memory {
    // These are part of the Memory trait, but aren't required for MBCs.
    // TODO: Potentially split up the `Memory` trait into `Tickable` and `Interruptable` for example.
    fn get_interrupt(&self) -> u8 {
        0
    }
    fn reset_interrupt(&mut self) {}
}

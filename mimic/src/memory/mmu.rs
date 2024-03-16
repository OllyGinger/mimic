use crate::cartridge::Cartridge;
use crate::tickable::Tickable;

use super::memory::TickableMemory;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

const HRAM_SIZE: usize = 0x7f;

pub struct MMU {
    /// Map of memory interfaces, each with its own address range. Multiple address ranges
    /// may map to the same memory interface
    interfaces: BTreeMap<(usize, usize), Rc<RefCell<dyn TickableMemory>>>,

    interrupt_flag: u8,
    hram: [u8; HRAM_SIZE],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            interfaces: BTreeMap::<(usize, usize), Rc<RefCell<dyn TickableMemory>>>::new(),
            interrupt_flag: 0u8,
            hram: [0u8; HRAM_SIZE],
        }
    }

    /// Add a memory interface to the MMU at the specified memory Range
    ///
    /// # Arguments
    ///
    /// * `address_ranges` - The memory ranges to add the interface to. These must NOT overlap
    ///     with any previously added ranges.
    /// * `interface` - The memory interface for this range
    pub fn add_interface(&mut self, interface: Rc<RefCell<dyn TickableMemory>>) {
        for range in interface.borrow().mapped_ranges() {
            let ord_range = (*range.start(), *range.end());
            if self.check_map_overlap(ord_range) {
                panic!(
                    "Address ranges must not overlap. Range: {:#06x}-{:#06x}",
                    ord_range.0, ord_range.1
                );
            }

            self.interfaces.insert(ord_range, interface.clone());
        }
    }

    fn check_map_overlap(&self, range: (usize, usize)) -> bool {
        for (key, _) in self.interfaces.iter() {
            if range.0 < key.1 && range.1 > key.0 {
                return true; // Overlap found
            }
        }
        false // No overlap found
    }

    pub fn map_cartridge(&mut self, cart: Rc<RefCell<Cartridge>>) {
        self.add_interface(cart);
    }

    pub fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            0xff80..=0xfffe => Some(self.hram[address as usize & 0x007f]),
            _ => {
                if let Some(binding) = self.try_get_mapped_interface(address) {
                    let interface = binding.borrow();
                    interface.try_read8(address)
                } else {
                    None
                }
            }
        }
    }

    pub fn read8(&self, address: u16) -> u8 {
        self.try_read8(address)
            .expect(&format!("Unmapped address: {:#06X}", address))
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        match address {
            0xff80..=0xfffe => self.hram[address as usize & 0x007f] = value,
            _ => {
                let binding = self.get_mapped_interface(address);
                let mut interface = binding.borrow_mut();
                interface.write8(address, value);
            }
        }
    }

    pub fn read16(&self, address: u16) -> u16 {
        let msb = self.read8(address);
        let lsb = self.read8(address + 1);
        (msb as u16) | ((lsb as u16) << 8)
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write8(address, (value & 0xFF) as u8);
        self.write8(address + 1, ((value >> 8) & 0xFF) as u8);
    }

    fn try_get_mapped_interface(&self, address: u16) -> Option<Rc<RefCell<dyn TickableMemory>>> {
        for (range, interface) in &self.interfaces {
            if address as usize >= range.0 && (address as usize) <= range.1 {
                return Some(interface.clone());
            }
        }

        None
    }

    fn get_mapped_interface(&self, address: u16) -> Rc<RefCell<dyn TickableMemory>> {
        if let Some(interface) = self.try_get_mapped_interface(address) {
            interface
        } else {
            // List out valid ranges
            let valid_ranges = self
                .interfaces
                .keys()
                .map(|(start, end)| format!("{:#06x}-{:#06x}", start, end))
                .collect::<Vec<String>>();
            panic!(
                "Address is not mapped to MMU: {:#06x}\nMapped ranges:\n  {}",
                address,
                valid_ranges.join("\n  ")
            );
        }
    }
}

impl Tickable for MMU {
    fn tick(&mut self, cycles: u8) {
        for (_, interface) in &mut self.interfaces {
            let mut memory = interface.borrow_mut();
            memory.tick(cycles);
            self.interrupt_flag |= memory.get_interrupt();
            memory.reset_interrupt();
        }
    }
}

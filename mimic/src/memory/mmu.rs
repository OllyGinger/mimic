use crate::cartridge::Cartridge;

use super::memory::Memory;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

pub struct MMU {
    /// Map of memory interfaces, each with its own address range. Multiple address ranges
    /// may map to the same memory interface
    interfaces: BTreeMap<(usize, usize), Rc<RefCell<dyn Memory>>>,

    interrupt_flag: u8,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            interfaces: BTreeMap::<(usize, usize), Rc<RefCell<dyn Memory>>>::new(),
            interrupt_flag: 0u8,
        }
    }

    /// Add a memory interface to the MMU at the specified memory Range
    ///
    /// # Arguments
    ///
    /// * `address_ranges` - The memory ranges to add the interface to. These must NOT overlap
    ///     with any previously added ranges.
    /// * `interface` - The memory interface for this range
    pub fn add_interface(&mut self, interface: Rc<RefCell<dyn Memory>>) {
        for range in interface.borrow().mapped_ranges() {
            let ord_range = (range.start, range.end);
            if self.interfaces.contains_key(&ord_range) {
                panic!("Address ranges must not overlap. Range: {:?}", ord_range);
            }

            self.interfaces.insert(ord_range, interface.clone());
        }
    }

    pub fn map_cartridge(&mut self, cart: Rc<RefCell<Cartridge>>) {
        self.add_interface(cart);
    }

    pub fn tick(&mut self) {
        for (_, interface) in &mut self.interfaces {
            let mut memory = interface.borrow_mut();
            memory.tick();
            self.interrupt_flag |= memory.get_interrupt();
            memory.reset_interrupt();
        }
    }

    pub fn read8(&self, address: u16) -> u8 {
        let binding = self.get_mapped_interface(address);
        let interface = binding.borrow();
        interface.read8(address)
    }

    pub fn write8(&mut self, address: u16, value: u8) {
        let binding = self.get_mapped_interface(address);
        let mut interface = binding.borrow_mut();
        interface.write8(address, value);
    }

    pub fn read16(&self, address: u16) -> u16 {
        let binding = self.get_mapped_interface(address);
        let interface = binding.borrow();
        (interface.read8(address) as u16) | ((interface.read8(address + 1) as u16) << 8)
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        let binding = self.get_mapped_interface(address);
        let mut interface = binding.borrow_mut();
        interface.write8(address, (value & 0xFF) as u8);
        interface.write8(address + 1, ((value >> 8) & 0xFF) as u8);
    }

    fn get_mapped_interface(&self, address: u16) -> Rc<RefCell<dyn Memory>> {
        for (range, interface) in &self.interfaces {
            if address as usize >= range.0 && (address as usize) < range.1 {
                return interface.clone();
            }
        }

        panic!("Address is not mapped to MMU: {:#06x}", address);
    }
}

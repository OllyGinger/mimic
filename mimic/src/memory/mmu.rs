use super::memory::Memory;

pub struct MMU {
    interfaces: Vec<Box<dyn Memory>>,

    interrupt_flag: u8,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            interfaces: Vec::<Box<dyn Memory>>::new(),
            interrupt_flag: 0u8,
        }
    }

    pub fn add_interface(&mut self, interface: Box<dyn Memory>) {
        self.interfaces.push(interface);
    }

    pub fn tick(&mut self) {
        for interface in &mut self.interfaces {
            interface.tick();
            self.interrupt_flag |= interface.get_interrupt();
            interface.reset_interrupt();
        }
    }
}

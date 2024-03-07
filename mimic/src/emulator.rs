use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::{self, Cartridge};
use crate::cpu::cpu::CPU;
use crate::main_window;
use crate::memory::mmu::MMU;

pub struct Emulator {
    cpu: CPU,
    gpu: (),
}

impl Emulator {
    pub fn from_rom_path(rom_path: String) -> Self {
        let rom_bytes = std::fs::read(rom_path).unwrap();
        let cart = cartridge::new(rom_bytes);

        let mut mmu = MMU::new();
        mmu.map_cartridge(Rc::new(RefCell::new(cart)));

        let cpu = CPU::new(mmu);

        Emulator { cpu, gpu: () }
    }

    pub fn run(&mut self) {
        //let main_window = main_window::new("Mimic".to_string(), 2048, 1024);
        //main_window.main_loop(|run, ui| {
        //    let w = ui.window("hello world");
        //    w.build(|| {
        //        ui.text("Hello World!");
        //        if ui.button("Exit") {
        //            *run = false;
        //        }
        //    });
        //});
        loop {
            self.cpu.tick();
        }
    }
}

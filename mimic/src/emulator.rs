use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::{self, Cartridge};
use crate::cpu::cpu::CPU;
use crate::gpu::gpu::GPU;
use crate::main_window;
use crate::memory::mmu::MMU;

pub struct Emulator {
    cpu: CPU,
    gpu: Rc<RefCell<GPU>>,
}

impl Emulator {
    pub fn from_rom_path(boot_rom_path: Option<String>, rom_path: String) -> Self {
        let rom_bytes = std::fs::read(rom_path).unwrap();
        let boot_rom_bytes: Option<Vec<u8>> = if let Some(path) = boot_rom_path {
            Some(std::fs::read(path).unwrap())
        } else {
            None
        };
        let cart = cartridge::new(boot_rom_bytes, rom_bytes);

        let mut mmu = MMU::new();
        mmu.map_cartridge(Rc::new(RefCell::new(cart)));

        let gpu = Rc::new(RefCell::new(GPU::new()));
        mmu.add_interface(gpu.clone());

        let cpu = CPU::new(mmu);
        Emulator {
            cpu,
            gpu: gpu.clone(),
        }
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

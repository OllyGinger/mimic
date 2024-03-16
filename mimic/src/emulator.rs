use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::{self};
use crate::cpu::cpu::CPU;
use crate::debugger::code_debugger::CodeDebugger;
use crate::gpu::gpu::GPU;
use crate::memory::mmu::MMU;
use crate::{io, main_window};

pub struct Emulator {
    pub cpu: CPU,
    gpu: Rc<RefCell<GPU>>,

    code_debugger: CodeDebugger,
}

impl Emulator {
    pub fn start(boot_rom_path: Option<String>, rom_path: String) {
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

        let audio = Rc::new(RefCell::new(io::audio::Audio::new()));
        mmu.add_interface(audio.clone());

        let cpu = CPU::new(mmu);
        let mut em = Emulator {
            cpu,
            gpu: gpu.clone(),
            code_debugger: CodeDebugger::new(),
        };

        let main_window = main_window::new("Mimic".to_string(), 2048, 1024);
        let func = move |_run: &mut bool, ui: &mut imgui::Ui| {
            em.draw(ui);
        };

        main_window.main_loop(func);
    }

    pub fn run(&mut self) {}

    fn draw(&mut self, ui: &mut imgui::Ui) {
        let code_debugger = &mut self.code_debugger;
        code_debugger.draw(ui, &self.cpu);
    }
}

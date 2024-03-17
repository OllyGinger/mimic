use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use crate::cartridge::{self};
use crate::cpu::cpu::CPU;
use crate::debugger::code_debugger::CodeDebugger;
use crate::gpu::gpu::GPU;
use crate::memory::mmu::MMU;
use crate::{io, main_window};

pub struct Emulator {
    pub cpu: CPU,
    gpu: Rc<RefCell<GPU>>,

    last_update: Instant,
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

            last_update: Instant::now(),
            code_debugger: CodeDebugger::new(),
        };

        let main_window = main_window::new("Mimic".to_string(), 2048, 1024);
        let func = move |_run: &mut bool, ui: &mut imgui::Ui| {
            em.update(ui);
        };

        main_window.main_loop(func);
    }

    fn update(&mut self, ui: &mut imgui::Ui) {
        let code_debugger = &mut self.code_debugger;
        code_debugger.draw(ui, &mut self.cpu);

        self.tick_cpu();
    }

    fn tick_cpu(&mut self) {
        const CPU_SPEED_HZ: u32 = 4194304;
        let delta = Instant::now() - self.last_update;

        let available_ticks = ((delta * CPU_SPEED_HZ).as_secs() as u64).min(100000);
        let mut ticks = 0;

        if !self.cpu.is_broken_to_debugger() {
            loop {
                ticks += self.cpu.tick();

                if self
                    .code_debugger
                    .breakpoints
                    .contains(&self.cpu.registers.pc())
                {
                    self.cpu.break_to_debugger();
                    break;
                }

                if ticks as u64 > available_ticks {
                    break;
                }
            }
        } else {
            if self.cpu.wants_single_step() {
                self.cpu.tick();

                // Reset the single step flag so it can be clicked again next time
                self.cpu.debug_single_step(false);
            }
        }

        self.last_update = Instant::now();
    }
}

use std::{cell::RefCell, rc::Rc};

use clap::Parser;
use cpu::cpu::CPU;
use memory::{mbc::MBC, memory::Memory, test_memory};

mod cartridge;
mod cpu;
mod emulator;
mod int_utils;
mod interruptable;
mod main_window;
mod memory;
mod tickable;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    boot_rom_path: Option<String>,

    #[arg(short, long)]
    rom_path: String,
}

fn main() {
    let mut memory = test_memory::TestMemory::new();
    memory.write8(0x00, 0x78);
    memory.write8(0x01, 0x4F);

    let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
    mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));

    let mut cpu = CPU::new(mmu);

    cpu.registers.set_a(1);
    cpu.registers.set_b(2);

    let main_window = main_window::new("Mimic".to_string(), 2048, 1024);
    main_window.main_loop(|run, ui| {
        let w = ui.window("hello world");
        w.build(|| {
            ui.text("Hello World!");
            if ui.button("Exit") {
                *run = false;
            }
        });
    });
    loop {
        cpu.tick();
    }
}

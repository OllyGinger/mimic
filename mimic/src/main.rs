use std::{cell::RefCell, rc::Rc};

use clap::Parser;
use cpu::cpu::CPU;
use memory::{mbc::MBC, memory::Memory, test_memory};

mod cartridge;
mod cpu;
mod emulator;
mod int_utils;
mod main_window;
mod memory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    boot_rom_path: Option<String>,

    #[arg(short, long)]
    rom_path: String,
}

fn main() {
    pretty_env_logger::init();
    let args: Args = Args::parse();

    let mut emulator = emulator::Emulator::from_rom_path(args.boot_rom_path, args.rom_path);
    emulator.run();
}

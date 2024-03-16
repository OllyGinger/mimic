#![allow(dead_code)]

use clap::Parser;

mod cartridge;
mod cpu;
mod debugger;
mod emulator;
mod gpu;
mod int_utils;
mod io;
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
    pretty_env_logger::init();
    let args: Args = Args::parse();

    emulator::Emulator::start(args.boot_rom_path, args.rom_path);
}

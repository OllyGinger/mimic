use crate::cartridge::{self, Cartridge};
use crate::cpu::cpu::CPU;
use crate::memory::mmu::MMU;

struct Emulator {
    cart: Cartridge,
    cpu: CPU,
    gpu: (),
}

impl Emulator {
    pub fn from_rom_path(rom_path: String) -> Self {
        let rom_bytes = std::fs::read(rom_path).unwrap();
        let cart = cartridge::new(rom_bytes);

        let mmu = MMU::new();

        let cpu = CPU::new(mmu);

        Emulator { cart, cpu, gpu: () }
    }
}

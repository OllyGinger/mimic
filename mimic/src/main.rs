use std::{cell::RefCell, rc::Rc};

use cpu::cpu::CPU;
use memory::{memory::Memory, test_memory};

mod cpu;
mod memory;

fn main() {
    let mut memory = test_memory::TestMemory::new();
    memory.write8(0x00, 0x78);
    memory.write8(0x01, 0x4F);

    let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
    mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));

    let mut cpu = CPU::new(mmu);

    cpu.registers.set_a(1);
    cpu.registers.set_b(2);

    loop {
        cpu.tick();
    }
}

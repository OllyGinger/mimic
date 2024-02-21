use cpu::cpu::CPU;
use memory::{memory::Memory, test_memory};

mod cpu;
mod memory;

fn main() {
    let mut memory = test_memory::TestMemory::new();
    memory.write8(0x00, 0x78);

    let mut cpu = CPU::new(Box::new(memory));

    cpu.registers.set_a(1);
    cpu.registers.set_b(2);

    loop {
        cpu.tick();
    }
}

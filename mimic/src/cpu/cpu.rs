use std::{fs, io};

use crate::{memory::mmu::MMU, tickable::Tickable};

use super::registers::{Flags, Registers};
use crate::int_utils::IntExt;
use std::io::Write;
use std::ops::Not;

pub struct CPU {
    pub registers: Registers,
    pub halt: bool,
    broken_in_debugger: bool,
    debug_single_step: bool,
    pub mmu: MMU,
    pub setdi: u8,
    pub setei: u8,

    log_enabled: bool,
    log_buf: io::BufWriter<fs::File>,
    log_counter: u32,
}

pub struct OpcodeAndPrefix {
    pub opcode: u8,
    pub prefix: Option<u8>,
    pub args: [u8; 2],
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        let has_boot_rom = mmu.read8(0xff50) == 0x00;
        let mut cpu = CPU {
            registers: Registers::new(),
            halt: false,
            broken_in_debugger: true, // For now, just always start broken in debugger
            debug_single_step: false,
            mmu: mmu,
            setdi: 0,
            setei: 0,

            log_enabled: true, // If Gameboy Doctor logging is enabled
            log_buf: io::BufWriter::new(
                fs::File::options()
                    .append(false)
                    .create(true)
                    .write(true)
                    .open(r"F:\repos\mimic\target\mimic_gameboy_doctor.txt")
                    .unwrap(),
            ),
            log_counter: 0,
        };
        if !has_boot_rom {
            cpu.registers.set_pc(0x0100);

            // Initialise for gameboy doctor
            cpu.registers.set_a(0x01);
            let mut f: crate::cpu::registers::Flags = crate::cpu::registers::Flags::empty();
            f.insert(crate::cpu::registers::Flags::ZERO);
            f.insert(crate::cpu::registers::Flags::CARRY);
            f.insert(crate::cpu::registers::Flags::HALF_CARRY);
            cpu.registers.set_flags(f);
            cpu.registers.set_b(0x00);
            cpu.registers.set_c(0x13);
            cpu.registers.set_d(0x00);
            cpu.registers.set_e(0xd8);
            cpu.registers.set_h(0x01);
            cpu.registers.set_l(0x4d);
            cpu.registers.set_sp(0xfffe);
        }
        cpu
    }

    pub fn halt(&mut self) {
        self.halt = true;
    }

    pub fn wake(&mut self) {
        self.halt = false;
    }

    pub fn is_halted(&self) -> bool {
        self.halt
    }

    pub fn break_to_debugger(&mut self) {
        self.broken_in_debugger = true;
    }

    pub fn resume_from_debugger(&mut self) {
        self.broken_in_debugger = false;
        self.debug_single_step(false);
    }

    pub fn is_broken_to_debugger(&self) -> bool {
        self.broken_in_debugger
    }

    pub fn debug_single_step(&mut self, single_step: bool) {
        self.debug_single_step = single_step;
        if single_step {
            self.break_to_debugger();
        }
    }

    pub fn wants_single_step(&self) -> bool {
        self.debug_single_step
    }

    pub fn push_stack(self: &mut CPU, value: u16) {
        self.mmu.write16(self.registers.sp().wrapping_sub(2), value);
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));
    }

    pub fn pop_stack(self: &mut CPU) -> u16 {
        let ret = self.mmu.read16(self.registers.sp());
        self.registers.set_sp(self.registers.sp().wrapping_add(2));
        ret
    }

    // TODO: This should probably check if we're about to read
    // past the end of valid memory
    pub fn read_next_opcode(&self) -> OpcodeAndPrefix {
        CPU::decode_opcode([
            self.mmu.read8(self.registers.pc()),
            self.mmu
                .try_read8(self.registers.pc() + 1)
                .or(Some(0x0))
                .unwrap(),
            self.mmu
                .try_read8(self.registers.pc() + 2)
                .or(Some(0x0))
                .unwrap(),
            self.mmu
                .try_read8(self.registers.pc() + 2)
                .or(Some(0x0))
                .unwrap(),
        ])
    }

    // Currently only supports 0xCB prefix (fine for SM83 CPU)
    pub fn decode_opcode(data: [u8; 4]) -> OpcodeAndPrefix {
        let prefix_or_opcode = data[0];
        let mut prefix: Option<u8> = None;
        let mut args = data[1..3].try_into().unwrap();
        let opcode;
        if prefix_or_opcode == 0xCB {
            opcode = data[1];
            prefix = Some(0xCB);
            args = data[2..4].try_into().unwrap();
        } else {
            opcode = prefix_or_opcode;
        }

        OpcodeAndPrefix {
            opcode,
            prefix,
            args,
        }
    }

    pub fn pre_tick(&mut self) {
        self.handle_dma_transfer();

        self.log_gameboy_doctor_state();
    }

    pub fn post_tick(&mut self, mcycles: u32) {
        let cycles = (mcycles * 4) as u8;
        self.mmu.tick(cycles);
    }

    fn handle_dma_transfer(&mut self) {
        let dma = self.mmu.read8(0xFF46);

        if dma == 0 {
            return;
        }

        let source_address = (dma as u16) << 8;
        let dest_address = 0xFE00;

        for i in 0..160 as u16 {
            let src = self.mmu.read8(source_address + i);
            self.mmu.write8(dest_address + i, src);
        }

        self.mmu.write8(0xFF46, 0x0);
    }

    fn log_gameboy_doctor_state(&mut self) {
        //if !self.mmu.read8(0xff50) == 0x00 {
        self.log_counter += 1;
        writeln!(
            &mut self.log_buf,
            "{} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            self.registers,
            self.mmu.read8(self.registers.pc()),
            self.mmu.read8(self.registers.pc() + 1),
            self.mmu.read8(self.registers.pc() + 2),
            self.mmu.read8(self.registers.pc() + 3)
        )
        .unwrap();
        //if self.log_counter % 100 == 0 {
        self.log_buf.flush().unwrap();
        //}
        //}
    }
}

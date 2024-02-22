use crate::memory::{memory::Memory, mmu::MMU};

use super::registers::Registers;
use std::fmt;

pub struct CPU {
    pub registers: Registers,
    pub halt: bool,
    pub mmu: MMU,
}

pub struct NextOpcode {
    pub opcode: u8,
    pub prefix: Option<u8>,
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        CPU {
            registers: Registers::new(),
            halt: false,
            mmu: mmu,
        }
    }

    pub fn halt(&mut self) {
        self.halt = true;
    }
    pub fn is_halted(&self) -> bool {
        self.halt
    }

    // Currently only supports 0xCB prefix (fine for SM83 CPU)
    pub fn read_next_opcode(&mut self) -> NextOpcode {
        let prefix_or_opcode = self.mmu.read8(self.registers.pc());
        let prefix: Option<u8> = None;
        let opcode;
        if prefix_or_opcode == 0xCB {
            opcode = self.mmu.read8(self.registers.pc() + 1);
        } else {
            opcode = prefix_or_opcode;
        }

        NextOpcode { opcode, prefix }
    }

    pub fn alu_add(&mut self, val: u8) {
        let (result, carry) = self.registers.a().overflowing_add(val);
        let half_carry = (val & 0x0f).checked_add(val | 0xf0).is_none();
        self.registers.set_flag_z(result == 0u8);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(half_carry);
        self.registers.set_flag_c(carry);
        self.registers.set_a(result);
    }

    pub fn alu_adc(&mut self, val: u8) {
        self.alu_add(val + self.registers.flag_c() as u8);
    }

    fn alu_subcp(&mut self, val: u8, carry: bool, update_a: bool) {
        let c = carry as u8;
        let result = self.registers.a().wrapping_sub(val).wrapping_sub(c);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(true);
        self.registers.set_flag_h(
            (self.registers.a() & 0xf)
                .wrapping_sub(val & 0xf)
                .wrapping_sub(c)
                & (0xf + 1)
                != 0,
        );
        self.registers
            .set_flag_c((self.registers.a() as u16) < (val as u16) + (carry as u16));

        if update_a {
            self.registers.set_a(result);
        }
    }

    pub fn alu_sub(&mut self, val: u8) {
        self.alu_subcp(val, false, true);
    }
    pub fn alu_sbc(&mut self, val: u8) {
        self.alu_subcp(val, self.registers.flag_c(), true);
    }

    pub fn alu_and(&mut self, val: u8) {
        self.registers.set_a(self.registers.a() & val);
        self.registers.set_flag_z(val == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(true);
        self.registers.set_flag_c(false);
    }

    pub fn alu_xor(&mut self, val: u8) {
        self.registers.set_a(self.registers.a() ^ val);
        self.registers.set_flag_z(val == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }

    pub fn alu_or(&mut self, val: u8) {
        self.registers.set_a(self.registers.a() | val);
        self.registers.set_flag_z(val == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }

    pub fn alu_cp(&mut self, val: u8) {
        self.alu_subcp(val, false, false);
    }
}

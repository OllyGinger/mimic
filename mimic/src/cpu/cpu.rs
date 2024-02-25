use crate::memory::mmu::MMU;

use super::registers::{Flags, Registers};
use crate::int_utils::IntExt;

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

    pub fn inc8(&self, val: u8) -> (u8, Flags) {
        let mut flags = Flags::empty();
        let result = val.wrapping_add(1);
        flags.set(Flags::ZERO, result == 0);
        flags.set(Flags::N_SUBTRACT, false);
        flags.set(Flags::HALF_CARRY, (val & 0x0f) + 1 > 0x0f);
        (result, flags)
    }

    pub fn dec8(&self, val: u8) -> (u8, Flags) {
        let mut flags = Flags::empty();
        let result = val.wrapping_sub(1);
        flags.set(Flags::ZERO, result == 0);
        flags.set(Flags::N_SUBTRACT, true);
        flags.set(Flags::HALF_CARRY, (val & 0x0f) == 0);
        (result, flags)
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

    pub fn alu_add_hl(&mut self, val: u16) {
        let a = val;
        let b = self.registers.hl();
        let (result, carry) = b.overflowing_add(a);
        let half_carry = u16::test_add_carry_bit(11, a, b);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(half_carry);
        self.registers.set_flag_c(carry);
        self.registers.set_hl(result);
    }
}

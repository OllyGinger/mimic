use log::trace;

use crate::memory::mmu::MMU;

use super::registers::{Flags, Registers};
use crate::int_utils::IntExt;

pub struct CPU {
    pub registers: Registers,
    pub halt: bool,
    pub mmu: MMU,
}

pub struct OpcodeAndPrefix {
    pub opcode: u8,
    pub prefix: Option<u8>,
    pub args: [u8; 2],
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

    // TODO: This should probably check if we're about to read
    // past the end of valid memory
    pub fn read_next_opcode(&self) -> OpcodeAndPrefix {
        self.decode_opcode([
            self.mmu.read8(self.registers.pc()),
            self.mmu.read8(self.registers.pc() + 1),
            self.mmu.read8(self.registers.pc() + 2),
            self.mmu.read8(self.registers.pc() + 3),
        ])
    }

    // Currently only supports 0xCB prefix (fine for SM83 CPU)
    pub fn decode_opcode(&self, data: [u8; 4]) -> OpcodeAndPrefix {
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
        println!(
            "{:04X}: {}\t\t{}",
            self.registers.pc(),
            self.disassemble(self.read_next_opcode()),
            self.registers
        );

        if self.registers.pc() == 0x0099 {
            self.halt = true;
        }
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

    pub fn alu_rlca(&mut self) {
        let val = self.registers.a();
        let carry = val & 0x80;
        self.registers.set_a(val.rotate_left(1));
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
    }

    pub fn alu_rla(&mut self) {
        let val = self.registers.a();
        let carry = val & 0x80;
        self.registers
            .set_a((val << 1) | self.registers.flag_c() as u8);
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
    }

    pub fn alu_rrca(&mut self) {
        let val = self.registers.a();
        let carry = val & 0x01;
        self.registers.set_a(val.rotate_right(1));
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
    }

    pub fn alu_rra(&mut self) {
        let val = self.registers.a();
        let carry = val & 0x01;
        self.registers
            .set_a((val >> 1) | ((self.registers.flag_c() as u8) << 7));
        self.registers.set_flag_z(false);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
    }

    pub fn alu_daa(&mut self) {
        let mut a = self.registers.a();
        let mut ajd = if self.registers.flag_c() { 0x60 } else { 0x0 };
        if self.registers.flag_h() {
            ajd |= 0x06;
        }
        if !self.registers.flag_n() {
            if a & 0x0f > 0x09 {
                ajd |= 0x06;
            }
            if a > 0x99 {
                ajd |= 0x60;
            }
            a = a.wrapping_add(ajd);
        } else {
            a = a.wrapping_sub(ajd);
        }

        self.registers.set_a(a);
        self.registers.set_flag_z(a == 0x0);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(ajd >= 0x60);
    }

    pub fn alu_rlc(&mut self, mut val: u8) -> u8 {
        let carry = val & 0x80;
        val = val.rotate_left(1);
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
        val
    }

    pub fn alu_rrc(&mut self, mut val: u8) -> u8 {
        let carry = val & 0x01;
        val = val.rotate_right(1);
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
        val
    }

    pub fn alu_rl(&mut self, mut val: u8) -> u8 {
        let carry = val & 0x80;
        val <<= 1 | self.registers.flag_c() as u8;
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
        val
    }

    pub fn alu_rr(&mut self, mut val: u8) -> u8 {
        let carry = val & 0x01;
        val >>= 1 | ((self.registers.flag_c() as u8) << 7);
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
        val
    }

    pub fn alu_sla(&mut self, mut val: u8) -> u8 {
        let carry = (val & 0x80) == 0x80;
        val >>= 1;
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry);
        val
    }

    pub fn alu_srl(&mut self, mut val: u8) -> u8 {
        let carry = val & 0x01;
        val >>= 1;
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
        val
    }

    pub fn alu_sra(&mut self, val: u8) -> u8 {
        let bit7 = val & 0x80;
        self.alu_srl(val) | bit7
    }

    pub fn alu_swap(&mut self, mut val: u8) -> u8 {
        val = ((val & 0x0f) << 4) | ((val & 0xf0) >> 4);
        self.registers.set_flag_z(val == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
        val
    }

    pub fn cc_nz(&self) -> bool {
        !self.cc_z()
    }

    pub fn cc_z(&self) -> bool {
        self.registers.flag_z()
    }

    pub fn cc_nc(&self) -> bool {
        !self.cc_c()
    }

    pub fn cc_c(&self) -> bool {
        self.registers.flag_c()
    }
}

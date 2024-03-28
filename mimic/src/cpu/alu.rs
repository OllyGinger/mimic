use super::{cpu::CPU, registers::Flags};
use crate::int_utils::IntExt;

impl CPU {
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
        let half_carry = (self.registers.a() & 0x0f)
            .checked_add(val | 0xf0)
            .is_none();
        self.registers.set_flag_z(result == 0u8);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(half_carry);
        self.registers.set_flag_c(carry);
        self.registers.set_a(result);
    }

    pub fn alu_adc(&mut self, val: u8) {
        let cy = self.registers.flag_c() as u8;
        let result = self.registers.a().wrapping_add(val).wrapping_add(cy);
        self.registers.set_flag_z(result == 0);
        self.registers.set_flag_n(false);
        self.registers
            .set_flag_h((self.registers.a() & 0xf) + (val & 0xf) + cy > 0xf);
        self.registers
            .set_flag_c(self.registers.a() as u16 + val as u16 + cy as u16 > 0xff);
        self.registers.set_a(result);
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
        self.registers.set_flag_z(self.registers.a() == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(true);
        self.registers.set_flag_c(false);
    }

    pub fn alu_xor(&mut self, val: u8) {
        self.registers.set_a(self.registers.a() ^ val);
        self.registers.set_flag_z(self.registers.a() == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }

    pub fn alu_or(&mut self, val: u8) {
        self.registers.set_a(self.registers.a() | val);
        self.registers.set_flag_z(self.registers.a() == 0);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(false);
    }

    pub fn alu_cp(&mut self, val: u8) {
        self.alu_subcp(val, false, false);
    }

    pub fn alu_add_hl(&mut self, val: u16) {
        let hl = self.registers.hl();
        let result = hl.wrapping_add(val);
        let half_carry = u16::test_add_carry_bit(11, hl, val);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(half_carry);
        self.registers.set_flag_c(hl > 0xffff - val);
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

    pub fn alu_rr(&mut self, val: u8) -> u8 {
        let carry = val & 0x01;
        let newval = (val >> 1) | ((self.registers.flag_c() as u8) << 7);
        self.registers.set_flag_z(newval == 0x00);
        self.registers.set_flag_n(false);
        self.registers.set_flag_h(false);
        self.registers.set_flag_c(carry != 0);
        newval
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

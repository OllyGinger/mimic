#![allow(unused, dead_code)]

use std::fmt;

use bitflags::bitflags;

bitflags! (
    pub struct Flags: u8 {
        const ZERO         = 0b_1000_0000;
        const N_SUBTRACT   = 0b_0100_0000;
        const HALF_CARRY   = 0b_0010_0000;
        const CARRY        = 0b_0001_0000;
    }
);

#[derive(Clone)]
pub struct Registers {
    pc: u16, // Program Counter
    sp: u16, // Stack Pointer
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    // Interrupts
    ime: u8, // Interrupt master enable
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum RegisterName {
    AF,
    A,
    F,
    BC,
    B,
    C,
    DE,
    D,
    E,
    HL,
    H,
    L,
    SP,
    PC,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A:{:02X} F:{:02X} \
           B:{:02X} C:{:02X} D:{:02X} E:{:02X} \
           H:{:02X} L:{:02X} SP:{:04X} PC:{:04X}",
            self.a(),
            self.f.bits,
            self.b(),
            self.c(),
            self.d(),
            self.e(),
            self.h(),
            self.l(),
            self.sp(),
            self.pc()
        )
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0,
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            ime: 0,
        }
    }

    // AF
    pub fn af(self: &Registers) -> u16 {
        ((self.a as u16) << 8 | self.f.bits() as u16)
    }
    pub fn a(self: &Registers) -> u8 {
        self.a
    }
    pub fn set_a(self: &mut Registers, a: u8) {
        self.a = a;
    }
    pub fn set_af(self: &mut Registers, af: u16) {
        self.a = (af >> 8) as u8;
        self.f = Flags::from_bits_truncate(af as u8);
    }

    // BC
    pub fn bc(self: &Registers) -> u16 {
        ((self.b as u16) << 8 | self.c as u16)
    }
    pub fn b(self: &Registers) -> u8 {
        self.b
    }
    pub fn c(self: &Registers) -> u8 {
        self.c
    }
    pub fn set_bc(self: &mut Registers, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = bc as u8
    }
    pub fn set_c(self: &mut Registers, c: u8) {
        self.c = c;
    }
    pub fn set_b(self: &mut Registers, b: u8) {
        self.b = b
    }

    // DE
    pub fn de(self: &Registers) -> u16 {
        ((self.d as u16) << 8 | self.e as u16)
    }
    pub fn d(self: &Registers) -> u8 {
        self.d
    }
    pub fn e(self: &Registers) -> u8 {
        self.e
    }
    pub fn set_de(self: &mut Registers, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = de as u8
    }
    pub fn set_e(self: &mut Registers, e: u8) {
        self.e = e;
    }
    pub fn set_d(self: &mut Registers, d: u8) {
        self.d = d
    }

    // HL
    pub fn hl(self: &Registers) -> u16 {
        ((self.h as u16) << 8 | self.l as u16)
    }
    pub fn h(self: &Registers) -> u8 {
        self.h
    }
    pub fn l(self: &Registers) -> u8 {
        self.l
    }
    pub fn set_hl(self: &mut Registers, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = hl as u8
    }
    pub fn set_l(self: &mut Registers, l: u8) {
        self.l = l
    }
    pub fn set_h(self: &mut Registers, h: u8) {
        self.h = h
    }

    // SP
    pub fn sp(self: &Registers) -> u16 {
        self.sp
    }
    pub fn set_sp(self: &mut Registers, sp: u16) {
        self.sp = sp;
    }

    // PC
    pub fn pc(self: &Registers) -> u16 {
        self.pc
    }
    pub fn set_pc(self: &mut Registers, pc: u16) {
        self.pc = pc;
    }
    pub fn inc_pc(self: &mut Registers, n: u16) {
        self.pc = self.pc + n;
    }

    // Flags
    pub fn flag_z(self: &Registers) -> bool {
        self.f.contains(Flags::ZERO)
    }
    pub fn set_flag_z(self: &mut Registers, flag: bool) {
        self.f.set(Flags::ZERO, flag);
    }
    pub fn flag_n(self: &Registers) -> bool {
        self.f.contains(Flags::N_SUBTRACT)
    }
    pub fn set_flag_n(self: &mut Registers, flag: bool) {
        self.f.set(Flags::N_SUBTRACT, flag);
    }
    pub fn flag_h(self: &Registers) -> bool {
        self.f.contains(Flags::HALF_CARRY)
    }
    pub fn set_flag_h(self: &mut Registers, flag: bool) {
        self.f.set(Flags::HALF_CARRY, flag);
    }
    pub fn flag_c(self: &Registers) -> bool {
        self.f.contains(Flags::CARRY)
    }
    pub fn set_flag_c(self: &mut Registers, flag: bool) {
        self.f.set(Flags::CARRY, flag);
    }

    // Interrupts
    pub fn set_ime(self: &mut Registers, enabled: bool) {
        self.ime = if enabled { 0x1 } else { 0x0 };
    }
    pub fn ime(self: &Registers) -> bool {
        self.ime == 0x1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_flag() {
        let mut r = Registers::new();
        assert_eq!(r.flag_z(), false);
        assert_eq!(r.af(), 0x0000);

        r.set_flag_z(true);
        assert_eq!(r.af(), 0x0080);
        assert_eq!(r.flag_z(), true);

        r.set_flag_z(false);
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_z(), false);

        r.set_flag_z(true);
        assert_eq!(r.af(), 0x0080);
    }

    #[test]
    fn test_n_flag() {
        let mut r = Registers::new();
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_n(), false);

        r.set_flag_n(true);
        assert_eq!(r.af(), 0x0040);
        assert_eq!(r.flag_n(), true);

        r.set_flag_n(false);
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_n(), false);

        r.set_flag_n(true);
        assert_eq!(r.af(), 0x0040);
    }

    #[test]
    fn test_h_flag() {
        let mut r = Registers::new();
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_h(), false);

        r.set_flag_h(true);
        assert_eq!(r.af(), 0x0020);
        assert_eq!(r.flag_h(), true);

        r.set_flag_h(false);
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_h(), false);

        r.set_flag_h(true);
        assert_eq!(r.af(), 0x0020);
    }

    #[test]
    fn test_c_flag() {
        let mut r = Registers::new();
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_c(), false);

        r.set_flag_c(true);
        assert_eq!(r.af(), 0x0010);
        assert_eq!(r.flag_c(), true);

        r.set_flag_c(false);
        assert_eq!(r.af(), 0x0000);
        assert_eq!(r.flag_c(), false);

        r.set_flag_c(true);
        assert_eq!(r.af(), 0x0010);
    }

    #[test]
    fn test_a_reg() {
        let mut r = Registers::new();
        r.set_a(0xff);
        assert_eq!(r.af(), 0xff00);
        assert_eq!(r.a(), 0xff);

        r.set_flag_z(true);
        assert_eq!(r.af(), 0xff80);
        assert_eq!(r.a(), 0xff);

        r.set_flag_n(true);
        assert_eq!(r.af(), 0xffC0);
        assert_eq!(r.a(), 0xff);
    }

    #[test]
    fn test_flag_combi() {
        let mut r = Registers::new();
        r.set_flag_z(true);
        assert_eq!(r.af(), 0x0080);
        r.set_flag_n(true);
        assert_eq!(r.af(), 0x00C0);
        r.set_flag_h(true);
        assert_eq!(r.af(), 0x00E0);
        r.set_flag_c(true);
        assert_eq!(r.af(), 0x00F0);
    }
}

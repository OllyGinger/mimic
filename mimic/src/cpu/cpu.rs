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
}

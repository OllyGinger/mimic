use super::registers::Registers;
use std::fmt;

pub struct CPU {
    pub registers: Registers,
    pub halt: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            halt: false,
        }
    }

    pub fn halt(&mut self) {
        self.halt = true;
    }
    pub fn is_halted(&self) -> bool {
        self.halt
    }
}

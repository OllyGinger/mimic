use std::ops::RangeInclusive;

use crate::memory::mmu::MMU;

use super::cpu::CPU;

pub struct DisassemblyLine {
    pub address_range: RangeInclusive<u16>,
    pub bytes: Vec<u8>,
    pub instruction: String,
    pub comment: Option<String>,
}

pub struct Disassembler {}

impl Disassembler {
    pub fn new() -> Disassembler {
        Disassembler {}
    }

    pub fn disassemble(&self, mmu: &MMU) -> Vec<DisassemblyLine> {
        let mut lines: Vec<DisassemblyLine> = vec![];
        let mut address = 0x0000u16;
        for _ in 0x0000u16..=0xffffu16 {
            // Read up to 4 bytes to work out what we're looking at
            let mut bytes: [Option<u8>; 4] = [None, None, None, None];
            let mut raw_bytes: [u8; 4] = [0u8, 0u8, 0u8, 0u8];
            bytes[0] = mmu.try_read8(address);
            if let Some(b) = bytes[0] {
                raw_bytes[0] = b
            }
            for offset in 1..=std::cmp::min(0xffff - address, 3) {
                if address + offset < 0xffffu16 {
                    bytes[offset as usize] = mmu.try_read8(address + offset);
                    if let Some(b) = bytes[offset as usize] {
                        raw_bytes[offset as usize] = b
                    }
                }
            }

            let op = CPU::decode_opcode(raw_bytes);
            let line: DisassemblyLine;

            // First check if the first byte isn't mapped. If not, we can't process any further
            if let None = bytes[0] {
                line = DisassemblyLine {
                    address_range: address..=address,
                    bytes: raw_bytes[..1].to_vec(),
                    instruction: "??".to_string(),
                    comment: None,
                };

                if address == 0xffff {
                    lines.push(line);
                    break;
                }
                address += 1;
            }
            // Then handle the main instruction
            else if let Some(instruction_len) = CPU::instruction_length(&op) {
                line = DisassemblyLine {
                    address_range: address..=address + instruction_len as u16,
                    bytes: raw_bytes[..instruction_len as usize].to_vec(),
                    instruction: CPU::disassemble(&op)
                        .expect("Unknown opcode") // This should never happen as it's the same check as `CPU::instruction_length` above
                        .replace(&"n8".to_string(), &format!("{:#04x}", op.args[0]))
                        .replace(
                            &"n16".to_string(),
                            &format!("{:#06x}", (op.args[0] as u16) | (op.args[1] as u16) << 8),
                        ),
                    comment: None,
                };

                if address == 0xffff {
                    lines.push(line);
                    break;
                }
                address += instruction_len as u16;
            }
            // Otherwise the address is mapped, but we don't recognise the opcode
            else {
                // Unknown opcode
                line = DisassemblyLine {
                    address_range: address..=address,
                    bytes: raw_bytes[..1 as usize].to_vec(),
                    instruction: "<unknown opcode>".to_string(),
                    comment: None,
                };

                if address == 0xffff {
                    lines.push(line);
                    break;
                }
                address += 1u16;
            }

            lines.push(line);
        }
        lines
    }
}

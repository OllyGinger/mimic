use crate::EncodingPattern;

#[derive(Default)]
pub struct Op {
    pub opcode: u32,
    pub mnemonic: String,
}

pub fn from_encoding_pattern(opcode: u32, encoding_pattern: &EncodingPattern) -> Op {
    Op {
        opcode: opcode,
        mnemonic: encoding_pattern.encoding.to_string(),
    }
}

impl Op {}

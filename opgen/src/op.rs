use proc_macro2::TokenStream;

use crate::EncodingPattern;

#[derive(Default)]
pub struct Op {
    pub opcode: u32,
    pub mnemonic: String,
    pub code: TokenStream,
}

pub fn from_encoding_pattern(opcode: u32, encoding_pattern: &EncodingPattern) -> Op {
    let new_code = handle_action_replacements(opcode, &encoding_pattern.action);
    Op {
        opcode: opcode,
        mnemonic: encoding_pattern.encoding.to_string(),
        code: new_code.parse().unwrap(),
    }
}

fn handle_action_replacements(opcode: u32, action: &String) -> String {
    let encoding_params: EncodingParams = opcode.into();

    action
        .replace("$RY", &get_register_variable(encoding_params.y))
        .replace("$RZ", &get_register_variable(encoding_params.z))
}

fn get_register_variable(idx: u8) -> String {
    const REGISTER: &'static [&'static str] = &[
        "cpu.registers.b",
        "cpu.registers.c",
        "cpu.registers.d",
        "cpu.registers.e",
        "(H)",
        "(L)",
        "",
        "cpu.registers.a",
    ];

    return REGISTER[idx as usize].to_string();
}

pub(crate) struct EncodingParams {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub p: u8,
    pub q: u8,
}

impl Into<EncodingParams> for u32 {
    fn into(self) -> EncodingParams {
        let code = self as u8;
        let y = ((code >> 3) & 7) as u8;
        EncodingParams {
            x: (code >> 6) as u8,
            y: y,
            z: y & 7,
            p: y >> 1,
            q: y & 1,
        }
    }
}

impl Op {}

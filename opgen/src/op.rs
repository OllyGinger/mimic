use proc_macro2::TokenStream;

use crate::EncodingPattern;

#[derive(Default)]
pub struct Op {
    pub opcode: u32,
    pub mnemonic: String,
    pub code: TokenStream,
    pub mcycle_duration: u8,
    pub mcycle_conditional_duration: Option<u8>,
}

pub fn from_encoding_pattern(opcode: u32, encoding_pattern: &EncodingPattern) -> Op {
    let new_code = handle_action_replacements(opcode, &encoding_pattern.action);
    let new_mnemonic = handle_mnemonic_replacements(opcode, &encoding_pattern.mnemonic);

    Op {
        opcode: opcode,
        mnemonic: new_mnemonic,
        code: new_code.parse().unwrap(),
        mcycle_duration: encoding_pattern.mcycle_duration,
        mcycle_conditional_duration: encoding_pattern.mcycle_conditional_duration,
    }
}

fn handle_action_replacements(opcode: u32, action: &String) -> String {
    let encoding_params: EncodingParams = opcode.into();

    action
        .replace("$RY", &get_register_variable(encoding_params.y))
        .replace("$RZ", &get_register_variable(encoding_params.z))
        .replace("$ALU", &get_alu_function(encoding_params.y))
}

fn handle_mnemonic_replacements(opcode: u32, mnemonic: &String) -> String {
    let encoding_params: EncodingParams = opcode.into();

    mnemonic
        .replace("$RY", &get_register_description(encoding_params.y))
        .replace("$RZ", &get_register_description(encoding_params.z))
        .replace("$ALU", &get_alu_function_description(encoding_params.y))
}

fn get_register_variable(idx: u8) -> String {
    const REGISTER: &'static [&'static str] = &["b", "c", "d", "e", "h", "l", "XX", "a"];
    REGISTER[idx as usize].to_string()
}

fn get_register_description(idx: u8) -> String {
    const REGISTER: &'static [&'static str] = &["B", "C", "D", "E", "H", "L", "(HL)", "A"];
    REGISTER[idx as usize].to_string()
}

fn get_alu_function(idx: u8) -> String {
    const FUNC: &'static [&'static str] = &["add", "adc", "sub", "sbc", "and", "xor", "or", "cp"];
    FUNC[idx as usize].to_string()
}

fn get_alu_function_description(idx: u8) -> String {
    get_alu_function(idx).to_uppercase()
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
            z: code & 7,
            p: y >> 1,
            q: y & 1,
        }
    }
}

impl Op {}

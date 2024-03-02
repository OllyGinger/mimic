use evalexpr::Node;
use serde::Deserialize;
use std::collections::HashMap;

mod errors;
pub mod generator;
mod op;
mod parser;

#[derive(Deserialize, PartialEq, Debug, Default)]
#[serde(default)]
pub struct EncodingPattern {
    pub encoding: String,
    pub action: String,
    #[serde(default = "default_serde_length")]
    pub length: u8,
    #[serde(default = "default_serde_duration")]
    pub mcycle_duration: u8,
    #[serde(default = "default_serde_conditional_duration")]
    pub conditional_duration: bool,
    pub tests: Option<Vec<EncodingTest>>,
    #[serde(default = "default_serde_prefix")]
    pub prefix: Option<u8>,

    #[serde(skip)]
    pub compiled_encoding: Option<Node>,
    #[serde(skip)]
    pub mnemonic: String,
}

fn default_serde_length() -> u8 {
    1
}

fn default_serde_duration() -> u8 {
    1
}

fn default_serde_conditional_duration() -> bool {
    false
}

fn default_serde_prefix() -> Option<u8> {
    None
}

#[derive(Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(default)]
pub struct EncodingTest {
    pub set: Vec<String>,
    pub expect: Vec<(String, u16)>,
}

#[derive(PartialEq, Debug)]
pub struct Encoding {
    pub ops: HashMap<String, EncodingPattern>,
}

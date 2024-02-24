use evalexpr::Node;
use serde::{Deserialize, Deserializer};
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
    pub mcycle_conditional_duration: Option<u8>,
    pub tests: Option<Vec<EncodingTest>>,

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

#[derive(Deserialize, PartialEq, Debug, Default, Clone)]
#[serde(default)]
pub struct EncodingTest {
    pub set: Vec<(String, u8)>,
    pub expect: Vec<(String, u8)>,
}

#[derive(PartialEq, Debug)]
pub struct Encoding {
    pub ops: HashMap<String, EncodingPattern>,
}

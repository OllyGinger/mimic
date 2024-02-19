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
    pub duration: u8,
    pub conditional_duration: Option<u8>,

    #[serde(skip)]
    pub compiled_encoding: Option<Node>,
    #[serde(skip)]
    pub mnemonic: String,
}

fn default_serde_length() -> u8 {
    1
}

#[derive(PartialEq, Debug)]
pub struct Encoding {
    pub ops: HashMap<String, EncodingPattern>,
}

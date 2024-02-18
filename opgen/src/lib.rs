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

    #[serde(skip)]
    pub compiled_encoding: Option<Node>,
}

#[derive(PartialEq, Debug)]
pub struct Encoding {
    pub ops: HashMap<String, EncodingPattern>,
}

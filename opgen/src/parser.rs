use evalexpr::build_operator_tree;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::EncodingPattern;

pub fn from_path(
    path: PathBuf,
    encodings: &mut HashMap<String, EncodingPattern>,
) -> std::io::Result<()> {
    let encoding_file = File::open(path)?;
    let reader = BufReader::new(encoding_file);
    let mut op_encoding: HashMap<String, EncodingPattern> =
        serde_yaml::from_reader(reader).unwrap();

    // Pre compile all of the evals
    for op in &mut op_encoding {
        let eval = build_operator_tree(&op.1.encoding);
        if let Err(e) = eval {
            panic!("Error parsing encoding for {}. Error: {}", op.0, e)
        }

        op.1.compiled_encoding = Some(eval.unwrap());
        op.1.mnemonic = op.0.clone();
    }

    *encodings = op_encoding;

    std::io::Result::Ok(())
}

use crate::{errors::BuildError, op, op::Op, parser, EncodingPattern};
use evalexpr::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::path::Path;
use std::{io::Write, path::PathBuf};

pub struct Generator {
    encodings: HashMap<String, EncodingPattern>,
    ops: BTreeMap<u32, Op>,
}

pub fn new() -> Generator {
    Generator {
        encodings: HashMap::<String, EncodingPattern>::new(),
        ops: BTreeMap::<u32, Op>::new(),
    }
}

impl Generator {
    pub fn generate(
        &mut self,
        encoding_path: impl Into<String>,
        output_path: impl Into<String>,
    ) -> Result<(), BuildError> {
        // Parse the yaml into our encoding map
        parser::from_path(PathBuf::from(encoding_path.into()), &mut self.encodings).unwrap();

        // Build all of the opcodes based on the specified rules
        self.process_opcodes();

        let out_path = PathBuf::from(output_path.into());
        let mut out_file = File::create(&out_path).unwrap();
        let stream = self.generate_file(&mut out_file);

        #[cfg(feature = "rustfmt")]
        rustfmt(&out_path)
    }

    fn process_opcodes(&mut self) {
        for opcode in 0..255 {
            let encoding_params: op::EncodingParams = opcode.into();

            // Loop through all of our opcode encodings to try and find one that matches
            let mut any_found = false;
            for op_encoding in &self.encodings {
                let eval_context = context_map! {
                    "x" => Value::Int(encoding_params.x as i64),
                    "y" => Value::Int(encoding_params.y as i64),
                    "z" => Value::Int(encoding_params.z as i64),
                    "p" => Value::Int(encoding_params.p as i64),
                    "q" => Value::Int(encoding_params.q as i64)
                }
                .unwrap();

                // Check if this encoding matches, otherwise skip
                match &op_encoding.1.compiled_encoding {
                    Some(eval) => match eval.eval_with_context(&eval_context) {
                        Ok(result) => {
                            if result != Value::from(true) {
                                continue;
                            }

                            if any_found {
                                panic!("An opcode ({:#06x}) has already matched, when processing '{}'. ", opcode, op_encoding.0);
                            }
                            any_found = true;
                        }
                        Err(e) => {
                            panic!(
                                "Error evaluating encoding for '{}'. Error: {}",
                                op_encoding.0, e
                            );
                        }
                    },
                    None => {}
                }

                self.ops.insert(
                    opcode.try_into().unwrap(),
                    op::from_encoding_pattern(opcode.try_into().unwrap(), op_encoding.1),
                );

                if any_found {
                    break;
                }
            }
        }
    }

    fn generate_file(&self, file: &mut File) {
        writeln!(
            file,
            "#[derive(Default)]
            struct Registers{{
                pc: u16, // Program Counter
                sp: u16, // Stack Pointer
                a: u8,
                f: Flags,
                b: u8,
                c: u8,
                d: u8,
                e: u8,
                h: u8,
                l: u8,

                // Interrupts
                ime: u8, // Interrupt master enable
            }}
            struct CPU {{
                pub registers: Registers
            }}"
        )
        .unwrap();

        writeln!(
            file,
            "
            pub fn opcode(op: u32, cpu: &mut CPU) {{
                match(op) {{"
        )
        .unwrap();

        for op in &self.ops {
            self.generate_op_match_arm_token_stream(file, &op.1);
        }

        writeln!(file, "}}").unwrap();
    }

    fn generate_op_match_arm_token_stream(&self, file: &mut File, op: &Op) {
        let action = &op.code;
        let opcode = op.opcode;

        writeln!(
            file,
            "
        ({}) => {{
            {}
        }}
        ",
            opcode,
            action.to_string()
        )
        .unwrap();
    }
}

#[cfg(feature = "rustfmt")]
fn rustfmt(path: &Path) -> Result<(), BuildError> {
    use std::{env, process::Command};

    Command::new(env::var("RUSTFMT").unwrap_or_else(|_| "rustfmt".to_string()))
        .args(&["--emit", "files"])
        .arg(path)
        .output()
        .map_err(BuildError::Fmt)?;

    Ok(())
}

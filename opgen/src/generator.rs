use crate::{errors::BuildError, op, op::Op, parser, EncodingPattern};
use evalexpr::*;
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
        let _stream = self.generate_file(&mut out_file);

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
            "use super::cpu::CPU;
            impl CPU {{"
        )
        .unwrap();

        writeln!(
            file,
            "
            /// Each opcode is calculated as a machine-cycle (time it takes to complete a sub-operation, eg a fetch)
            /// Returns: Duration of the tick in T-States, which is machine cycles * 4. 
            pub fn tick(&mut self) -> u32 {{
                let next_opcode = self.read_next_opcode();
                let mcycles;
                match next_opcode.opcode {{"
        )
        .unwrap();

        for op in &self.ops {
            self.generate_op_match_arm_token_stream(file, &op.1);
        }

        writeln!(file, "_ => unreachable!()").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, "mcycles").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, "}}").unwrap();

        self.generate_tests(file);
    }

    fn generate_op_match_arm_token_stream(&self, file: &mut File, op: &Op) {
        let action = &op.code;
        let opcode = op.opcode;

        writeln!(
            file,
            "
        // {}
        // ({:o} octal) - {}t
        {:#06x} => {{
            {}
            mcycles = {};
        }}
        ",
            op.mnemonic,
            opcode,
            op.mcycle_duration,
            opcode,
            action.to_string(),
            op.mcycle_duration,
        )
        .unwrap();
    }

    fn generate_tests(&self, file: &mut File) {
        writeln!(
            file,
            "#[cfg(test)]
            mod tests {{
                use crate::cpu::cpu::CPU;
                use crate::memory::{{mmu::MMU, memory::Memory, test_memory}};
                use std::rc::Rc;
                use std::cell::RefCell;"
        )
        .unwrap();

        for op in &self.ops {
            self.generate_op_test(file, &op.1);
        }

        writeln!(file, "}}").unwrap();
    }

    fn generate_op_test(&self, file: &mut File, op: &Op) {
        let op_name = op
            .mnemonic
            .replace(" ", "_")
            .replace(",", "")
            .replace("(", "ind_")
            .replace(")", "_ind")
            .to_lowercase();
        writeln!(file, "#[test]").unwrap();
        writeln!(file, "fn test_op_{op_name}() {{").unwrap();
        if let Some(tests) = &op.tests {
            for test in tests {
                writeln!(file, "let mut memory = test_memory::TestMemory::new();").unwrap();
                writeln!(
                    file,
                    "memory.write8(0x00, {:#04x}); // Write the opcode into test memory",
                    op.opcode
                )
                .unwrap();

                writeln!(file, "let mut mmu: MMU = MMU::new();").unwrap();
                writeln!(
                    file,
                    "mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));"
                )
                .unwrap();
                writeln!(file, "let mut cpu = CPU::new(mmu);").unwrap();
                for (reg, val) in &test.set {
                    writeln!(file, "cpu.registers.set_{}({:#04x});", reg, val).unwrap();
                }

                writeln!(file, "cpu.tick();").unwrap();

                for (reg, val) in &test.expect {
                    writeln!(file, "assert_eq!(cpu.registers.{}(), {:#04x});", reg, val).unwrap();
                }
            }
        }
        writeln!(file, "}}").unwrap();
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

use crate::{errors::BuildError, op, op::Op, parser, EncodingPattern};
use evalexpr::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::{io::Write, path::PathBuf};

pub struct Generator {
    encodings: HashMap<String, EncodingPattern>,
    ops: BTreeMap<u32, Op>,
    prefixes: HashSet<u8>,
}

pub fn new() -> Generator {
    Generator {
        encodings: HashMap::<String, EncodingPattern>::new(),
        ops: BTreeMap::<u32, Op>::new(),
        prefixes: HashSet::<u8>::new(),
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
        // Add any prefixes to a list for processing later
        for op_encoding in &self.encodings {
            if let Some(prefix) = op_encoding.1.prefix {
                self.prefixes.insert(prefix);
            }
        }

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
            /// Auto Generated - Do not modify
            ///
            /// Each opcode is calculated as a machine-cycle (time it takes to complete a sub-operation, eg a fetch)
            /// Returns: Duration of the tick in T-States. Machine cycles are t states * 4. 
            pub fn tick(&mut self) -> u32 {{
                let mut next_opcode = self.read_next_opcode();
                let mcycles;
                match next_opcode.prefix {{"
        )
        .unwrap();

        // No prefix
        writeln!(file, "None => {{").unwrap();
        writeln!(file, "match next_opcode.opcode {{").unwrap();
        for opcode in 0..256 {
            if self.ops.contains_key(&opcode) && self.ops[&opcode].prefix == None {
                self.generate_op_match_arm_token_stream(file, &self.ops[&opcode]);
            } else {
                writeln!(
                    file,
                    "
                    // ({:o} octal)
                    {:#04X} => {{
                        unreachable!();
                    }}
                    ",
                    opcode, opcode,
                )
                .unwrap();
            }
        }
        writeln!(file, "}}").unwrap();
        writeln!(file, "}}").unwrap();

        // Prefix
        writeln!(file, "Some(prefix) => {{").unwrap();
        writeln!(file, "self.registers.inc_pc(1);").unwrap();
        writeln!(file, "next_opcode = self.read_next_opcode();").unwrap();
        writeln!(file, "match prefix {{").unwrap();
        for prefix in &self.prefixes {
            writeln!(file, "{:#04X} => {{", prefix).unwrap();
            writeln!(file, "match next_opcode.opcode {{").unwrap();
            for opcode in 0..256 {
                if self.ops.contains_key(&opcode) && self.ops[&opcode].prefix == Some(*prefix) {
                    self.generate_op_match_arm_token_stream(file, &self.ops[&opcode]);
                }
            }
            writeln!(
                file,
                "
                _ => {{
                    unreachable!();
                }}
                "
            )
            .unwrap();
            writeln!(file, "}}").unwrap();
            writeln!(file, "}}").unwrap();
        }
        writeln!(
            file,
            "
            _ => {{
                unreachable!();
            }}
            "
        )
        .unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, "}}").unwrap();

        writeln!(file, "}}").unwrap();
        writeln!(file, "mcycles").unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, "}}").unwrap();

        self.generate_tests(file);
    }

    fn generate_op_match_arm_token_stream(&self, file: &mut File, op: &Op) {
        let action = &op.code;
        let opcode = op.opcode;

        if let Some(prefix) = op.prefix {
            write!(file, "// {:#04X}{:02X}", prefix, op.opcode).unwrap();
        }

        write!(
            file,
            "
        // {}
        // ({:o} octal) - {}t
        {:#04X} => {{
            {}
        ",
            op.mnemonic,
            opcode,
            op.mcycle_duration,
            opcode,
            action.to_string()
        )
        .unwrap();

        if !op.conditional_duration {
            write!(file, "mcycles = {};", op.mcycle_duration).unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    fn generate_tests(&self, file: &mut File) {
        writeln!(
            file,
            "#[cfg(test)]
            mod tests {{
                use crate::cpu::cpu::CPU;
                use crate::memory::{{mmu::MMU, memory::Memory, test_memory}};
                use std::rc::Rc;
                use std::cell::RefCell;
                
                fn create_test_cpu() -> CPU {{
                    let mut memory = test_memory::TestMemory::new();
                    memory.write8(0x00, 0x74); // Write the opcode into test memory
                    let mut mmu: MMU = MMU::new();
                    mmu.add_interface([0x0000..0xFFFF], Rc::new(RefCell::new(memory)));
                    CPU::new(mmu)
                }}"
        )
        .unwrap();

        for op in &self.ops {
            self.generate_op_test(file, &op.1);
        }

        writeln!(file, "}}").unwrap();
    }

    fn generate_op_test(&self, file: &mut File, op: &Op) {
        if let Some(tests) = &op.tests {
            let op_name = op
                .mnemonic
                .replace(" ", "_")
                .replace(",", "")
                .replace("(", "ind_")
                .replace(")", "_ind")
                .replace("+", "_inc_")
                .replace("-", "_dec_")
                .to_lowercase();
            for (idx, test) in tests.iter().enumerate() {
                writeln!(
                    file,
                    "#[test]
                #[allow(non_snake_case)]"
                )
                .unwrap();

                let mut test_name = format!("test_op_{:#04X}_{op_name}_test{idx}", op.opcode);
                if let Some(prefix) = op.prefix {
                    test_name =
                        format!("test_op_{prefix:#04X}{:02X}_{op_name}_test{idx}", op.opcode);
                }

                writeln!(file, "fn {test_name}(){{").unwrap();
                writeln!(file, "let mut cpu = create_test_cpu();").unwrap();
                if let Some(prefix) = op.prefix {
                    writeln!(file, "cpu.mmu.write8(0x00, {:#04X});", prefix).unwrap();
                    writeln!(file, "cpu.mmu.write8(0x01, {:#04X});", op.opcode).unwrap();
                } else {
                    writeln!(file, "cpu.mmu.write8(0x00, {:#04X});", op.opcode).unwrap();
                }
                for set in &test.set {
                    writeln!(file, "cpu.{};", set).unwrap();
                }

                writeln!(file, "cpu.tick();").unwrap();

                for (reg, val) in &test.expect {
                    let mut cast: String = "".to_string();
                    if reg.contains("flag_") {
                        cast = " != 0".to_string();
                    }
                    writeln!(file, "assert_eq!(cpu.{}, {:#04x}{cast});", reg, val).unwrap();
                }
                writeln!(file, "}}").unwrap();
            }
        }
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

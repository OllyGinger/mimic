use crate::{errors::BuildError, op, op::Op, parser, EncodingPattern};
use evalexpr::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::{io::Write, path::PathBuf};

pub struct Generator {
    encodings: HashMap<String, EncodingPattern>,
    ops: BTreeMap<(Option<u8>, u32), Op>,
    prefixes: HashSet<u8>,
}

pub fn new() -> Generator {
    Generator {
        encodings: HashMap::<String, EncodingPattern>::new(),
        ops: BTreeMap::<(Option<u8>, u32), Op>::new(),
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

        for opcode in 0..256 {
            let encoding_params: op::EncodingParams = opcode.into();
            let eval_context = context_map! {
                "x" => Value::Int(encoding_params.x as i64),
                "y" => Value::Int(encoding_params.y as i64),
                "z" => Value::Int(encoding_params.z as i64),
                "p" => Value::Int(encoding_params.p as i64),
                "q" => Value::Int(encoding_params.q as i64)
            }
            .unwrap();

            // Loop through all of our opcode encodings to try and find one that matches
            for op_encoding in &self.encodings {
                // Check if this encoding matches, otherwise skip
                match &op_encoding.1.compiled_encoding {
                    Some(eval) => match eval.eval_with_context(&eval_context) {
                        Ok(result) => {
                            if result != Value::from(true) {
                                continue;
                            }
                        }
                        Err(e) => {
                            panic!(
                                "Error evaluating encoding for '{}'. Error: {}",
                                op_encoding.0, e
                            );
                        }
                    },
                    None => {
                        panic!("Missing pre-compiled encoding for: {}", op_encoding.0);
                    }
                }

                self.ops.insert(
                    (op_encoding.1.prefix, opcode.try_into().unwrap()),
                    op::from_encoding_pattern(opcode.try_into().unwrap(), op_encoding.1),
                );
            }
        }
    }

    fn generate_file(&self, file: &mut File) {
        writeln!(
            file,
            "use super::cpu::CPU;
            use crate::int_utils::IntExt;
            use super::cpu::OpcodeAndPrefix;
            impl CPU {{"
        )
        .unwrap();

        self.generate_tick_fn(file);
        self.generate_disassemble_fn(file);
        self.generate_instruction_len_fn(file);
        writeln!(file, "}}").unwrap();

        self.generate_tests(file);
    }

    fn generate_tick_fn(&self, file: &mut File) {
        writeln!(
            file,
            "
            /// Auto Generated - Do not modify
            ///
            /// Each opcode is calculated as a machine-cycle (time it takes to complete a sub-operation, eg a fetch)
            /// Returns: Duration of the tick in T-States. Machine cycles are t states * 4. 
            pub fn tick(&mut self) -> u32 {{
                let next_opcode = self.read_next_opcode();
                let mcycles;
                self.pre_tick();
                match self.handle_interrupts() {{
                    0 => {{}}
                    n => {{
                        self.post_tick(n);
                        return n;
                    }}
                }}
                match next_opcode.prefix {{"
        )
        .unwrap();

        // No prefix
        writeln!(file, "None => {{").unwrap();
        writeln!(file, "match next_opcode.opcode {{").unwrap();
        for opcode in 0..256 {
            if self.ops.contains_key(&(None, opcode)) {
                self.generate_op_match_arm_token_stream(file, &self.ops[&(None, opcode)]);
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
        writeln!(file, "match prefix {{").unwrap();
        for prefix in &self.prefixes {
            writeln!(file, "{:#04X} => {{", prefix).unwrap();
            writeln!(file, "match next_opcode.opcode {{").unwrap();
            for opcode in 0..256 {
                if self.ops.contains_key(&(Some(*prefix), opcode)) {
                    self.generate_op_match_arm_token_stream(
                        file,
                        &self.ops[&(Some(*prefix), opcode)],
                    );
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

        writeln!(
            file,
            "}}
        self.post_tick(mcycles);"
        )
        .unwrap();
        writeln!(file, "mcycles").unwrap();
        writeln!(file, "}}").unwrap();
    }

    fn generate_disassemble_fn(&self, file: &mut File) {
        writeln!(
            file,
            "pub fn disassemble(opcode: &OpcodeAndPrefix) -> Option<String> {{
                match opcode.prefix {{"
        )
        .unwrap();

        // No Prefix
        writeln!(
            file,
            "None => {{
                match opcode.opcode {{"
        )
        .unwrap();
        for opcode in 0..256 {
            if self.ops.contains_key(&(None, opcode)) {
                self.generate_disassembly_op_match_arm(file, &self.ops[&(None, opcode)]);
            }
        }
        writeln!(
            file,
            "_ => {{None}}
        }}
    }}"
        )
        .unwrap();

        // Prefixes
        for prefix in &self.prefixes {
            writeln!(
                file,
                "Some({:#04X}) => {{
                match opcode.opcode {{",
                prefix
            )
            .unwrap();
            for opcode in 0..256 {
                if self.ops.contains_key(&(Some(*prefix), opcode)) {
                    self.generate_disassembly_op_match_arm(
                        file,
                        &self.ops[&(Some(*prefix), opcode)],
                    );
                }
            }
            writeln!(
                file,
                "_ => {{None}}
                }}
            }}",
            )
            .unwrap();
        }
        writeln!(
            file,
            "_ => {{None}}
        }}
            "
        )
        .unwrap();

        writeln!(file, "}}").unwrap();
    }

    fn generate_disassembly_op_match_arm(&self, file: &mut File, op: &Op) {
        write!(
            file,
            "
        {:#04X} => {{
            Some(\"{}\".to_string())
        }}",
            op.opcode as u8, op.mnemonic
        )
        .unwrap();
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

    fn generate_instruction_len_fn(&self, file: &mut File) {
        writeln!(
            file,
            "pub fn instruction_length(opcode: &OpcodeAndPrefix) -> Option<u8> {{
                match opcode.prefix {{"
        )
        .unwrap();

        // No Prefix
        writeln!(
            file,
            "None => {{
                match opcode.opcode {{"
        )
        .unwrap();
        for opcode in 0..256 {
            if self.ops.contains_key(&(None, opcode)) {
                self.generate_instruction_len_match_arm(file, &self.ops[&(None, opcode)]);
            }
        }
        writeln!(
            file,
            "_ => {{None}}
        }}
    }}"
        )
        .unwrap();

        // Prefixes
        for prefix in &self.prefixes {
            writeln!(
                file,
                "Some({:#04X}) => {{
                match opcode.opcode {{",
                prefix
            )
            .unwrap();
            for opcode in 0..256 {
                if self.ops.contains_key(&(Some(*prefix), opcode)) {
                    self.generate_instruction_len_match_arm(
                        file,
                        &self.ops[&(Some(*prefix), opcode)],
                    );
                }
            }
            writeln!(
                file,
                "_ => {{None}}
                }}
            }}",
            )
            .unwrap();
        }
        writeln!(
            file,
            "_ => {{None}}
        }}
            "
        )
        .unwrap();

        writeln!(file, "}}").unwrap();
    }

    fn generate_instruction_len_match_arm(&self, file: &mut File, op: &Op) {
        let mut len = op.length;

        // If this op has a prefix, add another byte to the length
        if let Some(_) = op.prefix {
            len += 1;
        }

        write!(
            file,
            "
        {:#04X} => {{
            Some({})
        }}",
            op.opcode as u8, len
        )
        .unwrap();
    }

    fn generate_tests(&self, file: &mut File) {
        writeln!(
            file,
            "#[cfg(test)]
            mod tests {{
                use crate::cpu::cpu::CPU;
                use crate::memory::{{mmu::MMU, test_memory}};
                use std::rc::Rc;
                use std::cell::RefCell;
                
                fn create_test_cpu() -> CPU {{
                    let memory = test_memory::TestMemory::new();
                    let mut mmu: MMU = MMU::new();
                    mmu.add_interface(Rc::new(RefCell::new(memory)));
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
                    writeln!(file, "{};", set).unwrap();
                }

                writeln!(file, "cpu.tick();").unwrap();

                for (reg, val) in &test.expect {
                    let mut cast: String = "".to_string();
                    if reg.contains("flag_") {
                        cast = " != 0".to_string();
                    }
                    writeln!(file, "assert_eq!({}, {:#04x}{cast});", reg, val).unwrap();
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

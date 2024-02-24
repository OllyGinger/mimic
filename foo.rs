use super::cpu::CPU;
impl CPU {
    /// Each opcode is calculated as a machine-cycle (time it takes to complete a sub-operation, eg a fetch)
    /// Returns: Duration of the tick in T-States, which is machine cycles * 4.
    pub fn tick(&mut self) -> u32 {
        let next_opcode = self.read_next_opcode();
        let mcycles;
        match next_opcode.opcode {
            // NOP
            // (0 octal) - 1t
            0x0000 => {
                self.registers.inc_pc(1);
                mcycles = 1;
            }

            // LD BC, n16
            // (1 octal) - 3t
            0x0001 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read16(self.registers.pc());
                self.registers.inc_pc(2);
                self.registers.set_hl(imm);
                mcycles = 3;
            }

            // LD (BC), A
            // (2 octal) - 2t
            0x0002 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.bc(), self.registers.a());
                mcycles = 2;
            }

            // INC BC
            // (3 octal) - 2t
            0x0003 => {
                self.registers.inc_pc(1);
                self.registers.set_bc(self.registers.bc().wrapping_add(1));
                mcycles = 2;
            }

            // INC B
            // (4 octal) - 1t
            0x0004 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.b().wrapping_add(1));
                mcycles = 1;
            }

            // DEC B
            // (5 octal) - 1t
            0x0005 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.b().wrapping_sub(1));
                mcycles = 1;
            }

            // LD B, n8
            // (6 octal) - 2t
            0x0006 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_b(imm);
                mcycles = 2;
            }

            // ADD HL, BC
            // (11 octal) - 2t
            0x0009 => {
                self.registers.inc_pc(1);
                self.alu_add_hl(self.registers.bc());
                mcycles = 2;
            }

            // LD A, (BC)
            // (12 octal) - 2t
            0x000a => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.mmu.read8(self.registers.bc()));
                mcycles = 2;
            }

            // DEC BC
            // (13 octal) - 2t
            0x000b => {
                self.registers.inc_pc(1);
                self.registers.set_bc(self.registers.bc().wrapping_sub(1));
                mcycles = 2;
            }

            // INC C
            // (14 octal) - 1t
            0x000c => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.c().wrapping_add(1));
                mcycles = 1;
            }

            // DEC C
            // (15 octal) - 1t
            0x000d => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.c().wrapping_sub(1));
                mcycles = 1;
            }

            // LD C, n8
            // (16 octal) - 2t
            0x000e => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_c(imm);
                mcycles = 2;
            }

            // LD DE, n16
            // (21 octal) - 3t
            0x0011 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read16(self.registers.pc());
                self.registers.inc_pc(2);
                self.registers.set_hl(imm);
                mcycles = 3;
            }

            // LD (DE), A
            // (22 octal) - 2t
            0x0012 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.de(), self.registers.a());
                mcycles = 2;
            }

            // INC DE
            // (23 octal) - 2t
            0x0013 => {
                self.registers.inc_pc(1);
                self.registers.set_de(self.registers.de().wrapping_add(1));
                mcycles = 2;
            }

            // INC D
            // (24 octal) - 1t
            0x0014 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.d().wrapping_add(1));
                mcycles = 1;
            }

            // DEC D
            // (25 octal) - 1t
            0x0015 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.d().wrapping_sub(1));
                mcycles = 1;
            }

            // LD D, n8
            // (26 octal) - 2t
            0x0016 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_d(imm);
                mcycles = 2;
            }

            // ADD HL, DE
            // (31 octal) - 2t
            0x0019 => {
                self.registers.inc_pc(1);
                self.alu_add_hl(self.registers.de());
                mcycles = 2;
            }

            // LD A, (DE)
            // (32 octal) - 2t
            0x001a => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.mmu.read8(self.registers.de()));
                mcycles = 2;
            }

            // DEC DE
            // (33 octal) - 2t
            0x001b => {
                self.registers.inc_pc(1);
                self.registers.set_de(self.registers.de().wrapping_sub(1));
                mcycles = 2;
            }

            // INC E
            // (34 octal) - 1t
            0x001c => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.e().wrapping_add(1));
                mcycles = 1;
            }

            // DEC E
            // (35 octal) - 1t
            0x001d => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.e().wrapping_sub(1));
                mcycles = 1;
            }

            // LD E, n8
            // (36 octal) - 2t
            0x001e => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_e(imm);
                mcycles = 2;
            }

            // LD HL, n16
            // (41 octal) - 3t
            0x0021 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read16(self.registers.pc());
                self.registers.inc_pc(2);
                self.registers.set_hl(imm);
                mcycles = 3;
            }

            // INC HL
            // (43 octal) - 2t
            0x0023 => {
                self.registers.inc_pc(1);
                self.registers.set_hl(self.registers.hl().wrapping_add(1));
                mcycles = 2;
            }

            // INC H
            // (44 octal) - 1t
            0x0024 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.h().wrapping_add(1));
                mcycles = 1;
            }

            // DEC H
            // (45 octal) - 1t
            0x0025 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.h().wrapping_sub(1));
                mcycles = 1;
            }

            // LD H, n8
            // (46 octal) - 2t
            0x0026 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_h(imm);
                mcycles = 2;
            }

            // ADD HL, HL
            // (51 octal) - 2t
            0x0029 => {
                self.registers.inc_pc(1);
                self.alu_add_hl(self.registers.hl());
                mcycles = 2;
            }

            // DEC HL
            // (53 octal) - 2t
            0x002b => {
                self.registers.inc_pc(1);
                self.registers.set_hl(self.registers.hl().wrapping_sub(1));
                mcycles = 2;
            }

            // INC L
            // (54 octal) - 1t
            0x002c => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.l().wrapping_add(1));
                mcycles = 1;
            }

            // DEC L
            // (55 octal) - 1t
            0x002d => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.l().wrapping_sub(1));
                mcycles = 1;
            }

            // LD L, n8
            // (56 octal) - 2t
            0x002e => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_l(imm);
                mcycles = 2;
            }

            // LD SP, n16
            // (61 octal) - 3t
            0x0031 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read16(self.registers.pc());
                self.registers.inc_pc(2);
                self.registers.set_hl(imm);
                mcycles = 3;
            }

            // INC SP
            // (63 octal) - 2t
            0x0033 => {
                self.registers.inc_pc(1);
                self.registers.set_sp(self.registers.sp().wrapping_add(1));
                mcycles = 2;
            }

            // INC (HL)
            // (64 octal) - 3t
            0x0034 => {
                self.registers.inc_pc(1);
                self.mmu.write8(
                    self.registers.hl(),
                    self.mmu.read8(self.registers.hl()).wrapping_add(1),
                );
                mcycles = 3;
            }

            // DEC (HL)
            // (65 octal) - 3t
            0x0035 => {
                self.registers.inc_pc(1);
                self.mmu.write8(
                    self.registers.hl(),
                    self.mmu.read8(self.registers.hl()).wrapping_sub(1),
                );
                mcycles = 3;
            }

            // LD (HL), n8
            // (66 octal) - 3t
            0x0036 => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), imm);
                mcycles = 3;
            }

            // ADD HL, SP
            // (71 octal) - 2t
            0x0039 => {
                self.registers.inc_pc(1);
                self.alu_add_hl(self.registers.sp());
                mcycles = 2;
            }

            // DEC SP
            // (73 octal) - 2t
            0x003b => {
                self.registers.inc_pc(1);
                self.registers.set_sp(self.registers.sp().wrapping_sub(1));
                mcycles = 2;
            }

            // INC A
            // (74 octal) - 1t
            0x003c => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.a().wrapping_add(1));
                mcycles = 1;
            }

            // DEC A
            // (75 octal) - 1t
            0x003d => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.a().wrapping_sub(1));
                mcycles = 1;
            }

            // LD A, n8
            // (76 octal) - 2t
            0x003e => {
                self.registers.inc_pc(1);
                let imm = self.mmu.read8(self.registers.pc());
                self.registers.inc_pc(1);
                self.registers.set_a(imm);
                mcycles = 2;
            }

            // LD B, B
            // (100 octal) - 1t
            0x0040 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.b());
                mcycles = 1;
            }

            // LD B, C
            // (101 octal) - 1t
            0x0041 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.c());
                mcycles = 1;
            }

            // LD B, D
            // (102 octal) - 1t
            0x0042 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.d());
                mcycles = 1;
            }

            // LD B, E
            // (103 octal) - 1t
            0x0043 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.e());
                mcycles = 1;
            }

            // LD B, H
            // (104 octal) - 1t
            0x0044 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.h());
                mcycles = 1;
            }

            // LD B, L
            // (105 octal) - 1t
            0x0045 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.l());
                mcycles = 1;
            }

            // LD B, (HL)
            // (106 octal) - 2t
            0x0046 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD B, A
            // (107 octal) - 1t
            0x0047 => {
                self.registers.inc_pc(1);
                self.registers.set_b(self.registers.a());
                mcycles = 1;
            }

            // LD C, B
            // (110 octal) - 1t
            0x0048 => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.b());
                mcycles = 1;
            }

            // LD C, C
            // (111 octal) - 1t
            0x0049 => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.c());
                mcycles = 1;
            }

            // LD C, D
            // (112 octal) - 1t
            0x004a => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.d());
                mcycles = 1;
            }

            // LD C, E
            // (113 octal) - 1t
            0x004b => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.e());
                mcycles = 1;
            }

            // LD C, H
            // (114 octal) - 1t
            0x004c => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.h());
                mcycles = 1;
            }

            // LD C, L
            // (115 octal) - 1t
            0x004d => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.l());
                mcycles = 1;
            }

            // LD C, (HL)
            // (116 octal) - 2t
            0x004e => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD C, A
            // (117 octal) - 1t
            0x004f => {
                self.registers.inc_pc(1);
                self.registers.set_c(self.registers.a());
                mcycles = 1;
            }

            // LD D, B
            // (120 octal) - 1t
            0x0050 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.b());
                mcycles = 1;
            }

            // LD D, C
            // (121 octal) - 1t
            0x0051 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.c());
                mcycles = 1;
            }

            // LD D, D
            // (122 octal) - 1t
            0x0052 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.d());
                mcycles = 1;
            }

            // LD D, E
            // (123 octal) - 1t
            0x0053 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.e());
                mcycles = 1;
            }

            // LD D, H
            // (124 octal) - 1t
            0x0054 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.h());
                mcycles = 1;
            }

            // LD D, L
            // (125 octal) - 1t
            0x0055 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.l());
                mcycles = 1;
            }

            // LD D, (HL)
            // (126 octal) - 2t
            0x0056 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD D, A
            // (127 octal) - 1t
            0x0057 => {
                self.registers.inc_pc(1);
                self.registers.set_d(self.registers.a());
                mcycles = 1;
            }

            // LD E, B
            // (130 octal) - 1t
            0x0058 => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.b());
                mcycles = 1;
            }

            // LD E, C
            // (131 octal) - 1t
            0x0059 => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.c());
                mcycles = 1;
            }

            // LD E, D
            // (132 octal) - 1t
            0x005a => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.d());
                mcycles = 1;
            }

            // LD E, E
            // (133 octal) - 1t
            0x005b => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.e());
                mcycles = 1;
            }

            // LD E, H
            // (134 octal) - 1t
            0x005c => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.h());
                mcycles = 1;
            }

            // LD E, L
            // (135 octal) - 1t
            0x005d => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.l());
                mcycles = 1;
            }

            // LD E, (HL)
            // (136 octal) - 2t
            0x005e => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD E, A
            // (137 octal) - 1t
            0x005f => {
                self.registers.inc_pc(1);
                self.registers.set_e(self.registers.a());
                mcycles = 1;
            }

            // LD H, B
            // (140 octal) - 1t
            0x0060 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.b());
                mcycles = 1;
            }

            // LD H, C
            // (141 octal) - 1t
            0x0061 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.c());
                mcycles = 1;
            }

            // LD H, D
            // (142 octal) - 1t
            0x0062 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.d());
                mcycles = 1;
            }

            // LD H, E
            // (143 octal) - 1t
            0x0063 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.e());
                mcycles = 1;
            }

            // LD H, H
            // (144 octal) - 1t
            0x0064 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.h());
                mcycles = 1;
            }

            // LD H, L
            // (145 octal) - 1t
            0x0065 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.l());
                mcycles = 1;
            }

            // LD H, (HL)
            // (146 octal) - 2t
            0x0066 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD H, A
            // (147 octal) - 1t
            0x0067 => {
                self.registers.inc_pc(1);
                self.registers.set_h(self.registers.a());
                mcycles = 1;
            }

            // LD L, B
            // (150 octal) - 1t
            0x0068 => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.b());
                mcycles = 1;
            }

            // LD L, C
            // (151 octal) - 1t
            0x0069 => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.c());
                mcycles = 1;
            }

            // LD L, D
            // (152 octal) - 1t
            0x006a => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.d());
                mcycles = 1;
            }

            // LD L, E
            // (153 octal) - 1t
            0x006b => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.e());
                mcycles = 1;
            }

            // LD L, H
            // (154 octal) - 1t
            0x006c => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.h());
                mcycles = 1;
            }

            // LD L, L
            // (155 octal) - 1t
            0x006d => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.l());
                mcycles = 1;
            }

            // LD L, (HL)
            // (156 octal) - 2t
            0x006e => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD L, A
            // (157 octal) - 1t
            0x006f => {
                self.registers.inc_pc(1);
                self.registers.set_l(self.registers.a());
                mcycles = 1;
            }

            // LD (HL), B
            // (160 octal) - 2t
            0x0070 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.b());
                mcycles = 2;
            }

            // LD (HL), C
            // (161 octal) - 2t
            0x0071 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.c());
                mcycles = 2;
            }

            // LD (HL), D
            // (162 octal) - 2t
            0x0072 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.d());
                mcycles = 2;
            }

            // LD (HL), E
            // (163 octal) - 2t
            0x0073 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.e());
                mcycles = 2;
            }

            // LD (HL), H
            // (164 octal) - 2t
            0x0074 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.h());
                mcycles = 2;
            }

            // LD (HL), L
            // (165 octal) - 2t
            0x0075 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.l());
                mcycles = 2;
            }

            // HALT
            // (166 octal) - 1t
            0x0076 => {
                self.halt();
                mcycles = 1;
            }

            // LD (HL), A
            // (167 octal) - 2t
            0x0077 => {
                self.registers.inc_pc(1);
                self.mmu.write8(self.registers.hl(), self.registers.a());
                mcycles = 2;
            }

            // LD A, B
            // (170 octal) - 1t
            0x0078 => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.b());
                mcycles = 1;
            }

            // LD A, C
            // (171 octal) - 1t
            0x0079 => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.c());
                mcycles = 1;
            }

            // LD A, D
            // (172 octal) - 1t
            0x007a => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.d());
                mcycles = 1;
            }

            // LD A, E
            // (173 octal) - 1t
            0x007b => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.e());
                mcycles = 1;
            }

            // LD A, H
            // (174 octal) - 1t
            0x007c => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.h());
                mcycles = 1;
            }

            // LD A, L
            // (175 octal) - 1t
            0x007d => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.l());
                mcycles = 1;
            }

            // LD A, (HL)
            // (176 octal) - 2t
            0x007e => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // LD A, A
            // (177 octal) - 1t
            0x007f => {
                self.registers.inc_pc(1);
                self.registers.set_a(self.registers.a());
                mcycles = 1;
            }

            // ADD B
            // (200 octal) - 1t
            0x0080 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.b());
                mcycles = 1;
            }

            // ADD C
            // (201 octal) - 1t
            0x0081 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.c());
                mcycles = 1;
            }

            // ADD D
            // (202 octal) - 1t
            0x0082 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.d());
                mcycles = 1;
            }

            // ADD E
            // (203 octal) - 1t
            0x0083 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.e());
                mcycles = 1;
            }

            // ADD H
            // (204 octal) - 1t
            0x0084 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.h());
                mcycles = 1;
            }

            // ADD L
            // (205 octal) - 1t
            0x0085 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.l());
                mcycles = 1;
            }

            // ADD (HL)
            // (206 octal) - 2t
            0x0086 => {
                self.registers.inc_pc(1);
                self.alu_add(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // ADD A
            // (207 octal) - 1t
            0x0087 => {
                self.registers.inc_pc(1);
                self.alu_add(self.registers.a());
                mcycles = 1;
            }

            // ADC B
            // (210 octal) - 1t
            0x0088 => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.b());
                mcycles = 1;
            }

            // ADC C
            // (211 octal) - 1t
            0x0089 => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.c());
                mcycles = 1;
            }

            // ADC D
            // (212 octal) - 1t
            0x008a => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.d());
                mcycles = 1;
            }

            // ADC E
            // (213 octal) - 1t
            0x008b => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.e());
                mcycles = 1;
            }

            // ADC H
            // (214 octal) - 1t
            0x008c => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.h());
                mcycles = 1;
            }

            // ADC L
            // (215 octal) - 1t
            0x008d => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.l());
                mcycles = 1;
            }

            // ADC (HL)
            // (216 octal) - 2t
            0x008e => {
                self.registers.inc_pc(1);
                self.alu_adc(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // ADC A
            // (217 octal) - 1t
            0x008f => {
                self.registers.inc_pc(1);
                self.alu_adc(self.registers.a());
                mcycles = 1;
            }

            // SUB B
            // (220 octal) - 1t
            0x0090 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.b());
                mcycles = 1;
            }

            // SUB C
            // (221 octal) - 1t
            0x0091 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.c());
                mcycles = 1;
            }

            // SUB D
            // (222 octal) - 1t
            0x0092 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.d());
                mcycles = 1;
            }

            // SUB E
            // (223 octal) - 1t
            0x0093 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.e());
                mcycles = 1;
            }

            // SUB H
            // (224 octal) - 1t
            0x0094 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.h());
                mcycles = 1;
            }

            // SUB L
            // (225 octal) - 1t
            0x0095 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.l());
                mcycles = 1;
            }

            // SUB (HL)
            // (226 octal) - 2t
            0x0096 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // SUB A
            // (227 octal) - 1t
            0x0097 => {
                self.registers.inc_pc(1);
                self.alu_sub(self.registers.a());
                mcycles = 1;
            }

            // SBC B
            // (230 octal) - 1t
            0x0098 => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.b());
                mcycles = 1;
            }

            // SBC C
            // (231 octal) - 1t
            0x0099 => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.c());
                mcycles = 1;
            }

            // SBC D
            // (232 octal) - 1t
            0x009a => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.d());
                mcycles = 1;
            }

            // SBC E
            // (233 octal) - 1t
            0x009b => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.e());
                mcycles = 1;
            }

            // SBC H
            // (234 octal) - 1t
            0x009c => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.h());
                mcycles = 1;
            }

            // SBC L
            // (235 octal) - 1t
            0x009d => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.l());
                mcycles = 1;
            }

            // SBC (HL)
            // (236 octal) - 2t
            0x009e => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // SBC A
            // (237 octal) - 1t
            0x009f => {
                self.registers.inc_pc(1);
                self.alu_sbc(self.registers.a());
                mcycles = 1;
            }

            // AND B
            // (240 octal) - 1t
            0x00a0 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.b());
                mcycles = 1;
            }

            // AND C
            // (241 octal) - 1t
            0x00a1 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.c());
                mcycles = 1;
            }

            // AND D
            // (242 octal) - 1t
            0x00a2 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.d());
                mcycles = 1;
            }

            // AND E
            // (243 octal) - 1t
            0x00a3 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.e());
                mcycles = 1;
            }

            // AND H
            // (244 octal) - 1t
            0x00a4 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.h());
                mcycles = 1;
            }

            // AND L
            // (245 octal) - 1t
            0x00a5 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.l());
                mcycles = 1;
            }

            // AND (HL)
            // (246 octal) - 2t
            0x00a6 => {
                self.registers.inc_pc(1);
                self.alu_and(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // AND A
            // (247 octal) - 1t
            0x00a7 => {
                self.registers.inc_pc(1);
                self.alu_and(self.registers.a());
                mcycles = 1;
            }

            // XOR B
            // (250 octal) - 1t
            0x00a8 => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.b());
                mcycles = 1;
            }

            // XOR C
            // (251 octal) - 1t
            0x00a9 => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.c());
                mcycles = 1;
            }

            // XOR D
            // (252 octal) - 1t
            0x00aa => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.d());
                mcycles = 1;
            }

            // XOR E
            // (253 octal) - 1t
            0x00ab => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.e());
                mcycles = 1;
            }

            // XOR H
            // (254 octal) - 1t
            0x00ac => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.h());
                mcycles = 1;
            }

            // XOR L
            // (255 octal) - 1t
            0x00ad => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.l());
                mcycles = 1;
            }

            // XOR (HL)
            // (256 octal) - 2t
            0x00ae => {
                self.registers.inc_pc(1);
                self.alu_xor(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // XOR A
            // (257 octal) - 1t
            0x00af => {
                self.registers.inc_pc(1);
                self.alu_xor(self.registers.a());
                mcycles = 1;
            }

            // OR B
            // (260 octal) - 1t
            0x00b0 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.b());
                mcycles = 1;
            }

            // OR C
            // (261 octal) - 1t
            0x00b1 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.c());
                mcycles = 1;
            }

            // OR D
            // (262 octal) - 1t
            0x00b2 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.d());
                mcycles = 1;
            }

            // OR E
            // (263 octal) - 1t
            0x00b3 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.e());
                mcycles = 1;
            }

            // OR H
            // (264 octal) - 1t
            0x00b4 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.h());
                mcycles = 1;
            }

            // OR L
            // (265 octal) - 1t
            0x00b5 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.l());
                mcycles = 1;
            }

            // OR (HL)
            // (266 octal) - 2t
            0x00b6 => {
                self.registers.inc_pc(1);
                self.alu_or(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // OR A
            // (267 octal) - 1t
            0x00b7 => {
                self.registers.inc_pc(1);
                self.alu_or(self.registers.a());
                mcycles = 1;
            }

            // CP B
            // (270 octal) - 1t
            0x00b8 => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.b());
                mcycles = 1;
            }

            // CP C
            // (271 octal) - 1t
            0x00b9 => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.c());
                mcycles = 1;
            }

            // CP D
            // (272 octal) - 1t
            0x00ba => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.d());
                mcycles = 1;
            }

            // CP E
            // (273 octal) - 1t
            0x00bb => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.e());
                mcycles = 1;
            }

            // CP H
            // (274 octal) - 1t
            0x00bc => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.h());
                mcycles = 1;
            }

            // CP L
            // (275 octal) - 1t
            0x00bd => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.l());
                mcycles = 1;
            }

            // CP (HL)
            // (276 octal) - 2t
            0x00be => {
                self.registers.inc_pc(1);
                self.alu_cp(self.mmu.read8(self.registers.hl()));
                mcycles = 2;
            }

            // CP A
            // (277 octal) - 1t
            0x00bf => {
                self.registers.inc_pc(1);
                self.alu_cp(self.registers.a());
                mcycles = 1;
            }

            _ => unreachable!(),
        }
        mcycles
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cpu::cpu::CPU;
    use memory::{memory::Memory, test_memory};
    #[test]
    fn test_op_nop() {}
    #[test]
    fn test_op_ld_bc_n16() {}
    #[test]
    fn test_op_ld_ind_bc_ind_a() {}
    #[test]
    fn test_op_inc_bc() {}
    #[test]
    fn test_op_inc_b() {}
    #[test]
    fn test_op_dec_b() {}
    #[test]
    fn test_op_ld_b_n8() {}
    #[test]
    fn test_op_add_hl_bc() {}
    #[test]
    fn test_op_ld_a_ind_bc_ind() {}
    #[test]
    fn test_op_dec_bc() {}
    #[test]
    fn test_op_inc_c() {}
    #[test]
    fn test_op_dec_c() {}
    #[test]
    fn test_op_ld_c_n8() {}
    #[test]
    fn test_op_ld_de_n16() {}
    #[test]
    fn test_op_ld_ind_de_ind_a() {}
    #[test]
    fn test_op_inc_de() {}
    #[test]
    fn test_op_inc_d() {}
    #[test]
    fn test_op_dec_d() {}
    #[test]
    fn test_op_ld_d_n8() {}
    #[test]
    fn test_op_add_hl_de() {}
    #[test]
    fn test_op_ld_a_ind_de_ind() {}
    #[test]
    fn test_op_dec_de() {}
    #[test]
    fn test_op_inc_e() {}
    #[test]
    fn test_op_dec_e() {}
    #[test]
    fn test_op_ld_e_n8() {}
    #[test]
    fn test_op_ld_hl_n16() {}
    #[test]
    fn test_op_inc_hl() {}
    #[test]
    fn test_op_inc_h() {}
    #[test]
    fn test_op_dec_h() {}
    #[test]
    fn test_op_ld_h_n8() {}
    #[test]
    fn test_op_add_hl_hl() {}
    #[test]
    fn test_op_dec_hl() {}
    #[test]
    fn test_op_inc_l() {}
    #[test]
    fn test_op_dec_l() {}
    #[test]
    fn test_op_ld_l_n8() {}
    #[test]
    fn test_op_ld_sp_n16() {}
    #[test]
    fn test_op_inc_sp() {}
    #[test]
    fn test_op_inc_ind_hl_ind() {}
    #[test]
    fn test_op_dec_ind_hl_ind() {}
    #[test]
    fn test_op_ld_ind_hl_ind_n8() {}
    #[test]
    fn test_op_add_hl_sp() {}
    #[test]
    fn test_op_dec_sp() {}
    #[test]
    fn test_op_inc_a() {}
    #[test]
    fn test_op_dec_a() {}
    #[test]
    fn test_op_ld_a_n8() {}
    #[test]
    fn test_op_ld_b_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_b_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_b_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_b_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_b_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_b_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_b_ind_hl_ind() {}
    #[test]
    fn test_op_ld_b_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_ld_c_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_c_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_c_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_c_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_c_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_c_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_c_ind_hl_ind() {}
    #[test]
    fn test_op_ld_c_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_ld_d_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_d_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_d_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_d_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_d_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_d_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_d_ind_hl_ind() {}
    #[test]
    fn test_op_ld_d_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_ld_e_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_e_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_e_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_e_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_e_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_e_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_e_ind_hl_ind() {}
    #[test]
    fn test_op_ld_e_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_ld_h_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_h_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_h_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_h_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_h_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_h_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_h_ind_hl_ind() {}
    #[test]
    fn test_op_ld_h_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_ld_l_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_l_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_l_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_l_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_l_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_l_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_l_ind_hl_ind() {}
    #[test]
    fn test_op_ld_l_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_ld_ind_hl_ind_b() {}
    #[test]
    fn test_op_ld_ind_hl_ind_c() {}
    #[test]
    fn test_op_ld_ind_hl_ind_d() {}
    #[test]
    fn test_op_ld_ind_hl_ind_e() {}
    #[test]
    fn test_op_ld_ind_hl_ind_h() {}
    #[test]
    fn test_op_ld_ind_hl_ind_l() {}
    #[test]
    fn test_op_halt() {}
    #[test]
    fn test_op_ld_ind_hl_ind_a() {}
    #[test]
    fn test_op_ld_a_b() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_b(0);
    }
    #[test]
    fn test_op_ld_a_c() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_c(0);
    }
    #[test]
    fn test_op_ld_a_d() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_d(0);
    }
    #[test]
    fn test_op_ld_a_e() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_e(0);
    }
    #[test]
    fn test_op_ld_a_h() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_h(0);
    }
    #[test]
    fn test_op_ld_a_l() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_l(0);
    }
    #[test]
    fn test_op_ld_a_ind_hl_ind() {}
    #[test]
    fn test_op_ld_a_a() {
        let mut memory = test_memory::TestMemory::new();
        let mut mmu: memory::mmu::MMU = memory::mmu::MMU::new();
        mmu.add_interface([0x00..0xFF], Rc::new(RefCell::new(memory)));
        let mut cpu = CPU::new(mmu);
        cpu.registers.set_a(0);
    }
    #[test]
    fn test_op_add_b() {}
    #[test]
    fn test_op_add_c() {}
    #[test]
    fn test_op_add_d() {}
    #[test]
    fn test_op_add_e() {}
    #[test]
    fn test_op_add_h() {}
    #[test]
    fn test_op_add_l() {}
    #[test]
    fn test_op_add_ind_hl_ind() {}
    #[test]
    fn test_op_add_a() {}
    #[test]
    fn test_op_adc_b() {}
    #[test]
    fn test_op_adc_c() {}
    #[test]
    fn test_op_adc_d() {}
    #[test]
    fn test_op_adc_e() {}
    #[test]
    fn test_op_adc_h() {}
    #[test]
    fn test_op_adc_l() {}
    #[test]
    fn test_op_adc_ind_hl_ind() {}
    #[test]
    fn test_op_adc_a() {}
    #[test]
    fn test_op_sub_b() {}
    #[test]
    fn test_op_sub_c() {}
    #[test]
    fn test_op_sub_d() {}
    #[test]
    fn test_op_sub_e() {}
    #[test]
    fn test_op_sub_h() {}
    #[test]
    fn test_op_sub_l() {}
    #[test]
    fn test_op_sub_ind_hl_ind() {}
    #[test]
    fn test_op_sub_a() {}
    #[test]
    fn test_op_sbc_b() {}
    #[test]
    fn test_op_sbc_c() {}
    #[test]
    fn test_op_sbc_d() {}
    #[test]
    fn test_op_sbc_e() {}
    #[test]
    fn test_op_sbc_h() {}
    #[test]
    fn test_op_sbc_l() {}
    #[test]
    fn test_op_sbc_ind_hl_ind() {}
    #[test]
    fn test_op_sbc_a() {}
    #[test]
    fn test_op_and_b() {}
    #[test]
    fn test_op_and_c() {}
    #[test]
    fn test_op_and_d() {}
    #[test]
    fn test_op_and_e() {}
    #[test]
    fn test_op_and_h() {}
    #[test]
    fn test_op_and_l() {}
    #[test]
    fn test_op_and_ind_hl_ind() {}
    #[test]
    fn test_op_and_a() {}
    #[test]
    fn test_op_xor_b() {}
    #[test]
    fn test_op_xor_c() {}
    #[test]
    fn test_op_xor_d() {}
    #[test]
    fn test_op_xor_e() {}
    #[test]
    fn test_op_xor_h() {}
    #[test]
    fn test_op_xor_l() {}
    #[test]
    fn test_op_xor_ind_hl_ind() {}
    #[test]
    fn test_op_xor_a() {}
    #[test]
    fn test_op_or_b() {}
    #[test]
    fn test_op_or_c() {}
    #[test]
    fn test_op_or_d() {}
    #[test]
    fn test_op_or_e() {}
    #[test]
    fn test_op_or_h() {}
    #[test]
    fn test_op_or_l() {}
    #[test]
    fn test_op_or_ind_hl_ind() {}
    #[test]
    fn test_op_or_a() {}
    #[test]
    fn test_op_cp_b() {}
    #[test]
    fn test_op_cp_c() {}
    #[test]
    fn test_op_cp_d() {}
    #[test]
    fn test_op_cp_e() {}
    #[test]
    fn test_op_cp_h() {}
    #[test]
    fn test_op_cp_l() {}
    #[test]
    fn test_op_cp_ind_hl_ind() {}
    #[test]
    fn test_op_cp_a() {}
}

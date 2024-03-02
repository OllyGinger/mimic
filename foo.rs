use super::cpu::CPU;
            impl CPU {

            /// Auto Generated - Do not modify
            ///
            /// Each opcode is calculated as a machine-cycle (time it takes to complete a sub-operation, eg a fetch)
            /// Returns: Duration of the tick in T-States. Machine cycles are t states * 4. 
            pub fn tick(&mut self) -> u32 {
                let next_opcode = self.read_next_opcode();
                let mcycles;
                match next_opcode.opcode {

        // NOP
        // (0 octal) - 1t
        0x0000 => {
            self . registers . inc_pc (1) ;
            mcycles = 1;
        }
        

        // LD BC, n16
        // (1 octal) - 3t
        0x0001 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read16 (self . registers . pc ()) ; self . registers . inc_pc (2) ; self . registers . set_bc (imm) ;
            mcycles = 3;
        }
        

        // LD (BC), A
        // (2 octal) - 2t
        0x0002 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . bc () , self . registers . a ()) ;
            mcycles = 2;
        }
        

        // INC BC
        // (3 octal) - 2t
        0x0003 => {
            self . registers . inc_pc (1) ; self . registers . set_bc (self . registers . bc () . wrapping_add (1)) ;
            mcycles = 2;
        }
        

        // INC B
        // (4 octal) - 1t
        0x0004 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers . b ()) ; self . registers . set_b (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC B
        // (5 octal) - 1t
        0x0005 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers . b ()) ; self . registers . set_b (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD B, n8
        // (6 octal) - 2t
        0x0006 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_b (imm) ;
            mcycles = 2;
        }
        

        // RLCA
        // (7 octal) - 1t
        0x0007 => {
            self . registers . inc_pc (1) ; self . alu_rlca () ;
            mcycles = 1;
        }
        

        // LD (n16), SP
        // (10 octal) - 5t
        0x0008 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read16 (self . registers . pc ()) ; self . registers . inc_pc (2) ; self . mmu . write16 (imm , self . registers . sp ()) ;
            mcycles = 5;
        }
        

        // ADD HL, BC
        // (11 octal) - 2t
        0x0009 => {
            self . registers . inc_pc (1) ; self . alu_add_hl (self . registers . bc ()) ;
            mcycles = 2;
        }
        

        // LD A, (BC)
        // (12 octal) - 2t
        0x000a => {
            self . registers . inc_pc (1) ; self . registers . set_a (self . mmu . read8 (self . registers . bc ())) ;
            mcycles = 2;
        }
        

        // DEC BC
        // (13 octal) - 2t
        0x000b => {
            self . registers . inc_pc (1) ; self . registers . set_bc (self . registers . bc () . wrapping_sub (1)) ;
            mcycles = 2;
        }
        

        // INC C
        // (14 octal) - 1t
        0x000c => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers . c ()) ; self . registers . set_c (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC C
        // (15 octal) - 1t
        0x000d => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers . c ()) ; self . registers . set_c (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD C, n8
        // (16 octal) - 2t
        0x000e => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_c (imm) ;
            mcycles = 2;
        }
        

        // RRCA
        // (17 octal) - 1t
        0x000f => {
            self . registers . inc_pc (1) ; self . alu_rrca () ;
            mcycles = 1;
        }
        

                // (20 octal)
                0x0010 => {
                    unreachable!();
                }
                

        // LD DE, n16
        // (21 octal) - 3t
        0x0011 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read16 (self . registers . pc ()) ; self . registers . inc_pc (2) ; self . registers . set_de (imm) ;
            mcycles = 3;
        }
        

        // LD (DE), A
        // (22 octal) - 2t
        0x0012 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . de () , self . registers . a ()) ;
            mcycles = 2;
        }
        

        // INC DE
        // (23 octal) - 2t
        0x0013 => {
            self . registers . inc_pc (1) ; self . registers . set_de (self . registers . de () . wrapping_add (1)) ;
            mcycles = 2;
        }
        

        // INC D
        // (24 octal) - 1t
        0x0014 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers . d ()) ; self . registers . set_d (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC D
        // (25 octal) - 1t
        0x0015 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers . d ()) ; self . registers . set_d (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD D, n8
        // (26 octal) - 2t
        0x0016 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_d (imm) ;
            mcycles = 2;
        }
        

        // RLA
        // (27 octal) - 1t
        0x0017 => {
            self . registers . inc_pc (1) ; self . alu_rla () ;
            mcycles = 1;
        }
        

        // JR n8
        // (30 octal) - 1t
        0x0018 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) as i8 ; self . registers . inc_pc (1) ; self . registers . set_pc (self . registers . pc () . wrapping_add (imm as u16)) ;
            mcycles = 1;
        }
        

        // ADD HL, DE
        // (31 octal) - 2t
        0x0019 => {
            self . registers . inc_pc (1) ; self . alu_add_hl (self . registers . de ()) ;
            mcycles = 2;
        }
        

        // LD A, (DE)
        // (32 octal) - 2t
        0x001a => {
            self . registers . inc_pc (1) ; self . registers . set_a (self . mmu . read8 (self . registers . de ())) ;
            mcycles = 2;
        }
        

        // DEC DE
        // (33 octal) - 2t
        0x001b => {
            self . registers . inc_pc (1) ; self . registers . set_de (self . registers . de () . wrapping_sub (1)) ;
            mcycles = 2;
        }
        

        // INC E
        // (34 octal) - 1t
        0x001c => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers . e ()) ; self . registers . set_e (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC E
        // (35 octal) - 1t
        0x001d => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers . e ()) ; self . registers . set_e (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD E, n8
        // (36 octal) - 2t
        0x001e => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_e (imm) ;
            mcycles = 2;
        }
        

        // RRA
        // (37 octal) - 1t
        0x001f => {
            self . registers . inc_pc (1) ; self . alu_rra () ;
            mcycles = 1;
        }
        

        // JR -4, n8
        // (40 octal) - 3t
        0x0020 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) as i8 ; self . registers . inc_pc (1) ; if self . cc_ $ CC () { sef . registers . set_pc (self . registers . pc () . wrapping_add (imm as u16)) ; }
            mcycles = 3;
        }
        

        // LD HL, n16
        // (41 octal) - 3t
        0x0021 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read16 (self . registers . pc ()) ; self . registers . inc_pc (2) ; self . registers . set_ $ RRP (imm) ;
            mcycles = 3;
        }
        

        // LD (HL+), A
        // (42 octal) - 2t
        0x0022 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers . a ()) ; self . registers . set_hl (self . registers . hl () . wrapping_add (1)) ;
            mcycles = 2;
        }
        

        // INC HL
        // (43 octal) - 2t
        0x0023 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RRP (self . registers .$ RRP () . wrapping_add (1)) ;
            mcycles = 2;
        }
        

        // INC H
        // (44 octal) - 1t
        0x0024 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers .$ RY ()) ; self . registers . set_ $ RY (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC H
        // (45 octal) - 1t
        0x0025 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers .$ RY ()) ; self . registers . set_ $ RY (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD H, n8
        // (46 octal) - 2t
        0x0026 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_ $ RY (imm) ;
            mcycles = 2;
        }
        

                // (47 octal)
                0x0027 => {
                    unreachable!();
                }
                

        // JR -4, n8
        // (50 octal) - 3t
        0x0028 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) as i8 ; self . registers . inc_pc (1) ; if self . cc_ $ CC () { sef . registers . set_pc (self . registers . pc () . wrapping_add (imm as u16)) ; }
            mcycles = 3;
        }
        

        // ADD HL, HL
        // (51 octal) - 2t
        0x0029 => {
            self . registers . inc_pc (1) ; self . alu_add_hl (self . registers . hl ()) ;
            mcycles = 2;
        }
        

        // LD A, (HL+)
        // (52 octal) - 2t
        0x002a => {
            self . registers . inc_pc (1) ; self . registers . set_a (self . mmu . read8 (self . registers . hl ())) ; self . registers . set_hl (self . registers . hl () . wrapping_add (1)) ;
            mcycles = 2;
        }
        

        // DEC HL
        // (53 octal) - 2t
        0x002b => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RRP (self . registers .$ RRP () . wrapping_sub (1)) ;
            mcycles = 2;
        }
        

        // INC L
        // (54 octal) - 1t
        0x002c => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers .$ RY ()) ; self . registers . set_ $ RY (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC L
        // (55 octal) - 1t
        0x002d => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers .$ RY ()) ; self . registers . set_ $ RY (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD L, n8
        // (56 octal) - 2t
        0x002e => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_ $ RY (imm) ;
            mcycles = 2;
        }
        

                // (57 octal)
                0x002f => {
                    unreachable!();
                }
                

        // JR -4, n8
        // (60 octal) - 3t
        0x0030 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) as i8 ; self . registers . inc_pc (1) ; if self . cc_ $ CC () { sef . registers . set_pc (self . registers . pc () . wrapping_add (imm as u16)) ; }
            mcycles = 3;
        }
        

        // LD SP, n16
        // (61 octal) - 3t
        0x0031 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read16 (self . registers . pc ()) ; self . registers . inc_pc (2) ; self . registers . set_ $ RRP (imm) ;
            mcycles = 3;
        }
        

        // LD (HL-), A
        // (62 octal) - 2t
        0x0032 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers . a ()) ; self . registers . set_hl (self . registers . hl () . wrapping_sub (1)) ;
            mcycles = 2;
        }
        

        // INC SP
        // (63 octal) - 2t
        0x0033 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RRP (self . registers .$ RRP () . wrapping_add (1)) ;
            mcycles = 2;
        }
        

        // INC (HL)
        // (64 octal) - 3t
        0x0034 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . mmu . read8 (self . registers . hl ())) ; self . registers . set_flags (flags) ; self . mmu . write8 (self . registers . hl () , val) ;
            mcycles = 3;
        }
        

        // DEC (HL)
        // (65 octal) - 3t
        0x0035 => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . mmu . read8 (self . registers . hl ())) ; self . registers . set_flags (flags) ; self . mmu . write8 (self . registers . hl () , val) ;
            mcycles = 3;
        }
        

        // LD (HL), n8
        // (66 octal) - 3t
        0x0036 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , imm) ;
            mcycles = 3;
        }
        

                // (67 octal)
                0x0037 => {
                    unreachable!();
                }
                

        // JR -4, n8
        // (70 octal) - 3t
        0x0038 => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) as i8 ; self . registers . inc_pc (1) ; if self . cc_ $ CC () { sef . registers . set_pc (self . registers . pc () . wrapping_add (imm as u16)) ; }
            mcycles = 3;
        }
        

        // ADD HL, SP
        // (71 octal) - 2t
        0x0039 => {
            self . registers . inc_pc (1) ; self . alu_add_hl (self . registers .$ RRP ()) ;
            mcycles = 2;
        }
        

        // LD A, (HL-)
        // (72 octal) - 2t
        0x003a => {
            self . registers . inc_pc (1) ; self . registers . set_a (self . mmu . read8 (self . registers . hl ())) ; self . registers . set_hl (self . registers . hl () . wrapping_sub (1)) ;
            mcycles = 2;
        }
        

        // DEC SP
        // (73 octal) - 2t
        0x003b => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RRP (self . registers .$ RRP () . wrapping_sub (1)) ;
            mcycles = 2;
        }
        

        // INC A
        // (74 octal) - 1t
        0x003c => {
            self . registers . inc_pc (1) ; let (val , flags) = self . inc8 (self . registers .$ RY ()) ; self . registers . set_ $ RY (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // DEC A
        // (75 octal) - 1t
        0x003d => {
            self . registers . inc_pc (1) ; let (val , flags) = self . dec8 (self . registers .$ RY ()) ; self . registers . set_ $ RY (val) ; self . registers . set_flags (flags) ;
            mcycles = 1;
        }
        

        // LD A, n8
        // (76 octal) - 2t
        0x003e => {
            self . registers . inc_pc (1) ; let imm = self . mmu . read8 (self . registers . pc ()) ; self . registers . inc_pc (1) ; self . registers . set_ $ RY (imm) ;
            mcycles = 2;
        }
        

                // (77 octal)
                0x003f => {
                    unreachable!();
                }
                

        // LD B, B
        // (100 octal) - 1t
        0x0040 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // LD B, C
        // (101 octal) - 1t
        0x0041 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // LD B, D
        // (102 octal) - 1t
        0x0042 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // LD B, E
        // (103 octal) - 1t
        0x0043 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // LD B, H
        // (104 octal) - 1t
        0x0044 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // LD B, L
        // (105 octal) - 1t
        0x0045 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // LD B, (HL)
        // (106 octal) - 2t
        0x0046 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD B, A
        // (107 octal) - 1t
        0x0047 => {
            self . registers . inc_pc (1) ; self . registers . set_b (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // LD C, B
        // (110 octal) - 1t
        0x0048 => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // LD C, C
        // (111 octal) - 1t
        0x0049 => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // LD C, D
        // (112 octal) - 1t
        0x004a => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // LD C, E
        // (113 octal) - 1t
        0x004b => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // LD C, H
        // (114 octal) - 1t
        0x004c => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // LD C, L
        // (115 octal) - 1t
        0x004d => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // LD C, (HL)
        // (116 octal) - 2t
        0x004e => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD C, A
        // (117 octal) - 1t
        0x004f => {
            self . registers . inc_pc (1) ; self . registers . set_c (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // LD D, B
        // (120 octal) - 1t
        0x0050 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // LD D, C
        // (121 octal) - 1t
        0x0051 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // LD D, D
        // (122 octal) - 1t
        0x0052 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // LD D, E
        // (123 octal) - 1t
        0x0053 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // LD D, H
        // (124 octal) - 1t
        0x0054 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // LD D, L
        // (125 octal) - 1t
        0x0055 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // LD D, (HL)
        // (126 octal) - 2t
        0x0056 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD D, A
        // (127 octal) - 1t
        0x0057 => {
            self . registers . inc_pc (1) ; self . registers . set_d (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // LD E, B
        // (130 octal) - 1t
        0x0058 => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // LD E, C
        // (131 octal) - 1t
        0x0059 => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // LD E, D
        // (132 octal) - 1t
        0x005a => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // LD E, E
        // (133 octal) - 1t
        0x005b => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // LD E, H
        // (134 octal) - 1t
        0x005c => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // LD E, L
        // (135 octal) - 1t
        0x005d => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // LD E, (HL)
        // (136 octal) - 2t
        0x005e => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD E, A
        // (137 octal) - 1t
        0x005f => {
            self . registers . inc_pc (1) ; self . registers . set_e (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // LD H, B
        // (140 octal) - 1t
        0x0060 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD H, C
        // (141 octal) - 1t
        0x0061 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD H, D
        // (142 octal) - 1t
        0x0062 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD H, E
        // (143 octal) - 1t
        0x0063 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD H, H
        // (144 octal) - 1t
        0x0064 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD H, L
        // (145 octal) - 1t
        0x0065 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD H, (HL)
        // (146 octal) - 2t
        0x0066 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD H, A
        // (147 octal) - 1t
        0x0067 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, B
        // (150 octal) - 1t
        0x0068 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, C
        // (151 octal) - 1t
        0x0069 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, D
        // (152 octal) - 1t
        0x006a => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, E
        // (153 octal) - 1t
        0x006b => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, H
        // (154 octal) - 1t
        0x006c => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, L
        // (155 octal) - 1t
        0x006d => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD L, (HL)
        // (156 octal) - 2t
        0x006e => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD L, A
        // (157 octal) - 1t
        0x006f => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD (HL), B
        // (160 octal) - 2t
        0x0070 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // LD (HL), C
        // (161 octal) - 2t
        0x0071 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // LD (HL), D
        // (162 octal) - 2t
        0x0072 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // LD (HL), E
        // (163 octal) - 2t
        0x0073 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // LD (HL), H
        // (164 octal) - 2t
        0x0074 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // LD (HL), L
        // (165 octal) - 2t
        0x0075 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // HALT
        // (166 octal) - 1t
        0x0076 => {
            self . halt () ;
            mcycles = 1;
        }
        

        // LD (HL), A
        // (167 octal) - 2t
        0x0077 => {
            self . registers . inc_pc (1) ; self . mmu . write8 (self . registers . hl () , self . registers .$ RZ ()) ;
            mcycles = 2;
        }
        

        // LD A, B
        // (170 octal) - 1t
        0x0078 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD A, C
        // (171 octal) - 1t
        0x0079 => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD A, D
        // (172 octal) - 1t
        0x007a => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD A, E
        // (173 octal) - 1t
        0x007b => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD A, H
        // (174 octal) - 1t
        0x007c => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD A, L
        // (175 octal) - 1t
        0x007d => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // LD A, (HL)
        // (176 octal) - 2t
        0x007e => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // LD A, A
        // (177 octal) - 1t
        0x007f => {
            self . registers . inc_pc (1) ; self . registers . set_ $ RY (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // ADD B
        // (200 octal) - 1t
        0x0080 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // ADD C
        // (201 octal) - 1t
        0x0081 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // ADD D
        // (202 octal) - 1t
        0x0082 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // ADD E
        // (203 octal) - 1t
        0x0083 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // ADD H
        // (204 octal) - 1t
        0x0084 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // ADD L
        // (205 octal) - 1t
        0x0085 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // ADD (HL)
        // (206 octal) - 2t
        0x0086 => {
            self . registers . inc_pc (1) ; self . alu_add (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // ADD A
        // (207 octal) - 1t
        0x0087 => {
            self . registers . inc_pc (1) ; self . alu_add (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // ADC B
        // (210 octal) - 1t
        0x0088 => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // ADC C
        // (211 octal) - 1t
        0x0089 => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // ADC D
        // (212 octal) - 1t
        0x008a => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // ADC E
        // (213 octal) - 1t
        0x008b => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // ADC H
        // (214 octal) - 1t
        0x008c => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // ADC L
        // (215 octal) - 1t
        0x008d => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // ADC (HL)
        // (216 octal) - 2t
        0x008e => {
            self . registers . inc_pc (1) ; self . alu_adc (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // ADC A
        // (217 octal) - 1t
        0x008f => {
            self . registers . inc_pc (1) ; self . alu_adc (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // SUB B
        // (220 octal) - 1t
        0x0090 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // SUB C
        // (221 octal) - 1t
        0x0091 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // SUB D
        // (222 octal) - 1t
        0x0092 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // SUB E
        // (223 octal) - 1t
        0x0093 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // SUB H
        // (224 octal) - 1t
        0x0094 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // SUB L
        // (225 octal) - 1t
        0x0095 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // SUB (HL)
        // (226 octal) - 2t
        0x0096 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // SUB A
        // (227 octal) - 1t
        0x0097 => {
            self . registers . inc_pc (1) ; self . alu_sub (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // SBC B
        // (230 octal) - 1t
        0x0098 => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . b ()) ;
            mcycles = 1;
        }
        

        // SBC C
        // (231 octal) - 1t
        0x0099 => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . c ()) ;
            mcycles = 1;
        }
        

        // SBC D
        // (232 octal) - 1t
        0x009a => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . d ()) ;
            mcycles = 1;
        }
        

        // SBC E
        // (233 octal) - 1t
        0x009b => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . e ()) ;
            mcycles = 1;
        }
        

        // SBC H
        // (234 octal) - 1t
        0x009c => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . h ()) ;
            mcycles = 1;
        }
        

        // SBC L
        // (235 octal) - 1t
        0x009d => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . l ()) ;
            mcycles = 1;
        }
        

        // SBC (HL)
        // (236 octal) - 2t
        0x009e => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // SBC A
        // (237 octal) - 1t
        0x009f => {
            self . registers . inc_pc (1) ; self . alu_sbc (self . registers . a ()) ;
            mcycles = 1;
        }
        

        // AND B
        // (240 octal) - 1t
        0x00a0 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // AND C
        // (241 octal) - 1t
        0x00a1 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // AND D
        // (242 octal) - 1t
        0x00a2 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // AND E
        // (243 octal) - 1t
        0x00a3 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // AND H
        // (244 octal) - 1t
        0x00a4 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // AND L
        // (245 octal) - 1t
        0x00a5 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // AND (HL)
        // (246 octal) - 2t
        0x00a6 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // AND A
        // (247 octal) - 1t
        0x00a7 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR B
        // (250 octal) - 1t
        0x00a8 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR C
        // (251 octal) - 1t
        0x00a9 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR D
        // (252 octal) - 1t
        0x00aa => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR E
        // (253 octal) - 1t
        0x00ab => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR H
        // (254 octal) - 1t
        0x00ac => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR L
        // (255 octal) - 1t
        0x00ad => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // XOR (HL)
        // (256 octal) - 2t
        0x00ae => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // XOR A
        // (257 octal) - 1t
        0x00af => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR B
        // (260 octal) - 1t
        0x00b0 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR C
        // (261 octal) - 1t
        0x00b1 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR D
        // (262 octal) - 1t
        0x00b2 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR E
        // (263 octal) - 1t
        0x00b3 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR H
        // (264 octal) - 1t
        0x00b4 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR L
        // (265 octal) - 1t
        0x00b5 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // OR (HL)
        // (266 octal) - 2t
        0x00b6 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // OR A
        // (267 octal) - 1t
        0x00b7 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP B
        // (270 octal) - 1t
        0x00b8 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP C
        // (271 octal) - 1t
        0x00b9 => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP D
        // (272 octal) - 1t
        0x00ba => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP E
        // (273 octal) - 1t
        0x00bb => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP H
        // (274 octal) - 1t
        0x00bc => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP L
        // (275 octal) - 1t
        0x00bd => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

        // CP (HL)
        // (276 octal) - 2t
        0x00be => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . mmu . read8 (self . registers . hl ())) ;
            mcycles = 2;
        }
        

        // CP A
        // (277 octal) - 1t
        0x00bf => {
            self . registers . inc_pc (1) ; self . alu_ $ ALU (self . registers .$ RZ ()) ;
            mcycles = 1;
        }
        

                // (300 octal)
                0x00c0 => {
                    unreachable!();
                }
                

                // (301 octal)
                0x00c1 => {
                    unreachable!();
                }
                

                // (302 octal)
                0x00c2 => {
                    unreachable!();
                }
                

                // (303 octal)
                0x00c3 => {
                    unreachable!();
                }
                

                // (304 octal)
                0x00c4 => {
                    unreachable!();
                }
                

                // (305 octal)
                0x00c5 => {
                    unreachable!();
                }
                

                // (306 octal)
                0x00c6 => {
                    unreachable!();
                }
                

                // (307 octal)
                0x00c7 => {
                    unreachable!();
                }
                

                // (310 octal)
                0x00c8 => {
                    unreachable!();
                }
                

                // (311 octal)
                0x00c9 => {
                    unreachable!();
                }
                

                // (312 octal)
                0x00ca => {
                    unreachable!();
                }
                

                // (313 octal)
                0x00cb => {
                    unreachable!();
                }
                

                // (314 octal)
                0x00cc => {
                    unreachable!();
                }
                

                // (315 octal)
                0x00cd => {
                    unreachable!();
                }
                

                // (316 octal)
                0x00ce => {
                    unreachable!();
                }
                

                // (317 octal)
                0x00cf => {
                    unreachable!();
                }
                

                // (320 octal)
                0x00d0 => {
                    unreachable!();
                }
                

                // (321 octal)
                0x00d1 => {
                    unreachable!();
                }
                

                // (322 octal)
                0x00d2 => {
                    unreachable!();
                }
                

                // (323 octal)
                0x00d3 => {
                    unreachable!();
                }
                

                // (324 octal)
                0x00d4 => {
                    unreachable!();
                }
                

                // (325 octal)
                0x00d5 => {
                    unreachable!();
                }
                

                // (326 octal)
                0x00d6 => {
                    unreachable!();
                }
                

                // (327 octal)
                0x00d7 => {
                    unreachable!();
                }
                

                // (330 octal)
                0x00d8 => {
                    unreachable!();
                }
                

                // (331 octal)
                0x00d9 => {
                    unreachable!();
                }
                

                // (332 octal)
                0x00da => {
                    unreachable!();
                }
                

                // (333 octal)
                0x00db => {
                    unreachable!();
                }
                

                // (334 octal)
                0x00dc => {
                    unreachable!();
                }
                

                // (335 octal)
                0x00dd => {
                    unreachable!();
                }
                

                // (336 octal)
                0x00de => {
                    unreachable!();
                }
                

                // (337 octal)
                0x00df => {
                    unreachable!();
                }
                

                // (340 octal)
                0x00e0 => {
                    unreachable!();
                }
                

                // (341 octal)
                0x00e1 => {
                    unreachable!();
                }
                

                // (342 octal)
                0x00e2 => {
                    unreachable!();
                }
                

                // (343 octal)
                0x00e3 => {
                    unreachable!();
                }
                

                // (344 octal)
                0x00e4 => {
                    unreachable!();
                }
                

                // (345 octal)
                0x00e5 => {
                    unreachable!();
                }
                

                // (346 octal)
                0x00e6 => {
                    unreachable!();
                }
                

                // (347 octal)
                0x00e7 => {
                    unreachable!();
                }
                

                // (350 octal)
                0x00e8 => {
                    unreachable!();
                }
                

                // (351 octal)
                0x00e9 => {
                    unreachable!();
                }
                

                // (352 octal)
                0x00ea => {
                    unreachable!();
                }
                

                // (353 octal)
                0x00eb => {
                    unreachable!();
                }
                

                // (354 octal)
                0x00ec => {
                    unreachable!();
                }
                

                // (355 octal)
                0x00ed => {
                    unreachable!();
                }
                

                // (356 octal)
                0x00ee => {
                    unreachable!();
                }
                

                // (357 octal)
                0x00ef => {
                    unreachable!();
                }
                

                // (360 octal)
                0x00f0 => {
                    unreachable!();
                }
                

                // (361 octal)
                0x00f1 => {
                    unreachable!();
                }
                

                // (362 octal)
                0x00f2 => {
                    unreachable!();
                }
                

                // (363 octal)
                0x00f3 => {
                    unreachable!();
                }
                

                // (364 octal)
                0x00f4 => {
                    unreachable!();
                }
                

                // (365 octal)
                0x00f5 => {
                    unreachable!();
                }
                

                // (366 octal)
                0x00f6 => {
                    unreachable!();
                }
                

                // (367 octal)
                0x00f7 => {
                    unreachable!();
                }
                

                // (370 octal)
                0x00f8 => {
                    unreachable!();
                }
                

                // (371 octal)
                0x00f9 => {
                    unreachable!();
                }
                

                // (372 octal)
                0x00fa => {
                    unreachable!();
                }
                

                // (373 octal)
                0x00fb => {
                    unreachable!();
                }
                

                // (374 octal)
                0x00fc => {
                    unreachable!();
                }
                

                // (375 octal)
                0x00fd => {
                    unreachable!();
                }
                

                // (376 octal)
                0x00fe => {
                    unreachable!();
                }
                

                // (377 octal)
                0x00ff => {
                    unreachable!();
                }
                
}
mcycles
}
}
#[cfg(test)]
            mod tests {
                use crate::cpu::cpu::CPU;
                use crate::memory::{mmu::MMU, memory::Memory, test_memory};
                use std::rc::Rc;
                use std::cell::RefCell;
                
                fn create_test_cpu() -> CPU {
                    let mut memory = test_memory::TestMemory::new();
                    memory.write8(0x00, 0x74); // Write the opcode into test memory
                    let mut mmu: MMU = MMU::new();
                    mmu.add_interface([0x0000..0xFFFF], Rc::new(RefCell::new(memory)));
                    CPU::new(mmu)
                }
#[test]
                #[allow(non_snake_case)]
fn test_op_0x00_nop_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x00);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x00_nop_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x00);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x01_ld_bc_n16_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x01);
cpu.mmu.write16(0x0001, 0x1234);
cpu.tick();
assert_eq!(cpu.registers.bc(), 0x1234);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x03);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x01_ld_bc_n16_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x01);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x02_ld_ind_bc_ind_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x02);
cpu.registers.set_bc(0x0F00);
cpu.registers.set_a(0xFF);
cpu.tick();
assert_eq!(cpu.mmu.read8(0x0F00), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x02_ld_ind_bc_ind_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x02);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x03_inc_bc_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x03);
cpu.registers.set_bc(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.bc(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x03_inc_bc_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x03);
cpu.registers.set_bc(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.bc(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x04_inc_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x04);
cpu.registers.set_b(0xFE);
cpu.tick();
assert_eq!(cpu.registers.b(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x04_inc_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x04);
cpu.registers.set_b(0xFF);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x04_inc_b_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x04);
cpu.registers.set_b(0xEF);
cpu.tick();
assert_eq!(cpu.registers.b(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x05_dec_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x05);
cpu.registers.set_b(0xFF);
cpu.tick();
assert_eq!(cpu.registers.b(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x05_dec_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x05);
cpu.registers.set_b(0x00);
cpu.tick();
assert_eq!(cpu.registers.b(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x05_dec_b_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x05);
cpu.registers.set_b(0x01);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x06_ld_b_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x06);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.b(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x06_ld_b_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x06);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x07_rlca_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x07);
cpu.registers.set_a(0x01);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x02);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x07_rlca_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x07);
cpu.registers.set_a(0x7f);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x07_rlca_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x07);
cpu.registers.set_a(0xFE);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xfd);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x08_ld_ind_n16_ind_sp_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x08);
cpu.mmu.write16(0x1, 0xDEAF);
cpu.registers.set_sp(0x1234);
cpu.tick();
assert_eq!(cpu.mmu.read16(0xDEAF), 0x1234);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x08_ld_ind_n16_ind_sp_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x08);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x09_add_hl_bc_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x09);
cpu.registers.set_bc(0x1111);
cpu.registers.set_hl(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xbcde);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x09_add_hl_bc_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x09);
cpu.registers.set_bc(0x0001);
cpu.registers.set_hl(0x0FFF);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x1000);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x09_add_hl_bc_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x09);
cpu.registers.set_bc(0x0001);
cpu.registers.set_hl(0xFFFF);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0a_ld_a_ind_bc_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0a);
cpu.registers.set_bc(0x0F00);
cpu.mmu.write8(0x0F00, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0a_ld_a_ind_bc_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0a);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0b_dec_bc_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0b);
cpu.registers.set_bc(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.bc(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0b_dec_bc_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0b);
cpu.registers.set_bc(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.bc(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0c_inc_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0c);
cpu.registers.set_c(0xFE);
cpu.tick();
assert_eq!(cpu.registers.c(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0c_inc_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0c);
cpu.registers.set_c(0xFF);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0c_inc_c_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0c);
cpu.registers.set_c(0xEF);
cpu.tick();
assert_eq!(cpu.registers.c(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0d_dec_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0d);
cpu.registers.set_c(0xFF);
cpu.tick();
assert_eq!(cpu.registers.c(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0d_dec_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0d);
cpu.registers.set_c(0x00);
cpu.tick();
assert_eq!(cpu.registers.c(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0d_dec_c_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0d);
cpu.registers.set_c(0x01);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0e_ld_c_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0e);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.c(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0e_ld_c_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0e);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0f_rrca_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0f);
cpu.registers.set_a(0x01);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x80);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0f_rrca_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0f);
cpu.registers.set_a(0x7f);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xbf);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x0f_rrca_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x0f);
cpu.registers.set_a(0xFE);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x7f);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x11_ld_de_n16_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x11);
cpu.mmu.write16(0x0001, 0x1234);
cpu.tick();
assert_eq!(cpu.registers.de(), 0x1234);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x03);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x11_ld_de_n16_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x11);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x12_ld_ind_de_ind_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x12);
cpu.registers.set_de(0x0F00);
cpu.registers.set_a(0xFF);
cpu.tick();
assert_eq!(cpu.mmu.read8(0x0F00), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x12_ld_ind_de_ind_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x12);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x13_inc_de_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x13);
cpu.registers.set_de(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.de(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x13_inc_de_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x13);
cpu.registers.set_de(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.de(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x14_inc_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x14);
cpu.registers.set_d(0xFE);
cpu.tick();
assert_eq!(cpu.registers.d(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x14_inc_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x14);
cpu.registers.set_d(0xFF);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x14_inc_d_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x14);
cpu.registers.set_d(0xEF);
cpu.tick();
assert_eq!(cpu.registers.d(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x15_dec_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x15);
cpu.registers.set_d(0xFF);
cpu.tick();
assert_eq!(cpu.registers.d(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x15_dec_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x15);
cpu.registers.set_d(0x00);
cpu.tick();
assert_eq!(cpu.registers.d(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x15_dec_d_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x15);
cpu.registers.set_d(0x01);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x16_ld_d_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x16);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.d(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x16_ld_d_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x16);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x17_rla_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x17);
cpu.registers.set_a(0x01);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x02);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x17_rla_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x17);
cpu.registers.set_a(0x01);
cpu.registers.set_flag_c(true);;
cpu.tick();
assert_eq!(cpu.registers.a(), 0x03);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x17_rla_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x17);
cpu.registers.set_a(0x7f);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x17_rla_test3(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x17);
cpu.registers.set_a(0x7f);
cpu.registers.set_flag_c(true);;
cpu.tick();
assert_eq!(cpu.registers.a(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x17_rla_test4(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x17);
cpu.registers.set_a(0xFE);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xfc);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x17_rla_test5(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x17);
cpu.registers.set_a(0xFE);
cpu.registers.set_flag_c(true);;
cpu.tick();
assert_eq!(cpu.registers.a(), 0xfd);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x18_jr_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x18);
cpu.mmu.write8(0x0001, 0x12);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x14);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x18_jr_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x18);
cpu.mmu.write8(0x0001, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x19_add_hl_de_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x19);
cpu.registers.set_de(0x1111);
cpu.registers.set_hl(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xbcde);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x19_add_hl_de_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x19);
cpu.registers.set_de(0x0001);
cpu.registers.set_hl(0x0FFF);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x1000);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x19_add_hl_de_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x19);
cpu.registers.set_de(0x0001);
cpu.registers.set_hl(0xFFFF);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1a_ld_a_ind_de_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1a);
cpu.registers.set_de(0x0F00);
cpu.mmu.write8(0x0F00, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1a_ld_a_ind_de_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1a);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1b_dec_de_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1b);
cpu.registers.set_de(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.de(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1b_dec_de_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1b);
cpu.registers.set_de(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.de(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1c_inc_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1c);
cpu.registers.set_e(0xFE);
cpu.tick();
assert_eq!(cpu.registers.e(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1c_inc_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1c);
cpu.registers.set_e(0xFF);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1c_inc_e_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1c);
cpu.registers.set_e(0xEF);
cpu.tick();
assert_eq!(cpu.registers.e(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1d_dec_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1d);
cpu.registers.set_e(0xFF);
cpu.tick();
assert_eq!(cpu.registers.e(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1d_dec_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1d);
cpu.registers.set_e(0x00);
cpu.tick();
assert_eq!(cpu.registers.e(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1d_dec_e_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1d);
cpu.registers.set_e(0x01);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1e_ld_e_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1e);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.e(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1e_ld_e_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1e);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1f_rra_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1f);
cpu.registers.set_a(0x01);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1f_rra_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1f);
cpu.registers.set_a(0x01);
cpu.registers.set_flag_c(true);;
cpu.tick();
assert_eq!(cpu.registers.a(), 0x80);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1f_rra_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1f);
cpu.registers.set_a(0x7f);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x3f);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1f_rra_test3(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1f);
cpu.registers.set_a(0x7f);
cpu.registers.set_flag_c(true);;
cpu.tick();
assert_eq!(cpu.registers.a(), 0xbf);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1f_rra_test4(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1f);
cpu.registers.set_a(0xFE);
cpu.tick();
assert_eq!(cpu.registers.a(), 0x7f);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x1f_rra_test5(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x1f);
cpu.registers.set_a(0xFE);
cpu.registers.set_flag_c(true);;
cpu.tick();
assert_eq!(cpu.registers.a(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x20_jr__dec_4_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x20);
cpu.mmu.write8(0x0001, 0x12);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x14);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x20_jr__dec_4_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x20);
cpu.mmu.write8(0x0001, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x21_ld_hl_n16_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x21);
cpu.mmu.write16(0x0001, 0x1234);
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0x1234);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x03);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x21_ld_hl_n16_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x21);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x22_ld_ind_hl_inc__ind_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x22);
cpu.registers.set_hl(0x0F00);
cpu.registers.set_a(0xFF);
cpu.tick();
assert_eq!(cpu.mmu.read8(0x0F00), 0xff);
assert_eq!(cpu.registers.hl(), 0xf01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x22_ld_ind_hl_inc__ind_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x22);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x23_inc_hl_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x23);
cpu.registers.set_$RRP(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x23_inc_hl_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x23);
cpu.registers.set_$RRP(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x24_inc_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x24);
cpu.registers.set_$RY(0xFE);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x24_inc_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x24);
cpu.registers.set_$RY(0xFF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x24_inc_h_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x24);
cpu.registers.set_$RY(0xEF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x25_dec_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x25);
cpu.registers.set_$RY(0xFF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x25_dec_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x25);
cpu.registers.set_$RY(0x00);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x25_dec_h_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x25);
cpu.registers.set_$RY(0x01);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x26_ld_h_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x26);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x26_ld_h_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x26);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x28_jr__dec_4_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x28);
cpu.mmu.write8(0x0001, 0x12);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x14);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x28_jr__dec_4_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x28);
cpu.mmu.write8(0x0001, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x29_add_hl_hl_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x29);
cpu.registers.set_hl(0x1234);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x2468);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2a_ld_a_ind_hl_inc__ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2a);
cpu.registers.set_hl(0x0F00);
cpu.mmu.write8(0x0F00, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xff);
assert_eq!(cpu.registers.hl(), 0xf01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2a_ld_a_ind_hl_inc__ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2a);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2b_dec_hl_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2b);
cpu.registers.set_$RRP(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2b_dec_hl_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2b);
cpu.registers.set_$RRP(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2c_inc_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2c);
cpu.registers.set_$RY(0xFE);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2c_inc_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2c);
cpu.registers.set_$RY(0xFF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2c_inc_l_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2c);
cpu.registers.set_$RY(0xEF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2d_dec_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2d);
cpu.registers.set_$RY(0xFF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2d_dec_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2d);
cpu.registers.set_$RY(0x00);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2d_dec_l_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2d);
cpu.registers.set_$RY(0x01);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2e_ld_l_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2e);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x2e_ld_l_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x2e);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x30_jr__dec_4_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x30);
cpu.mmu.write8(0x0001, 0x12);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x14);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x30_jr__dec_4_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x30);
cpu.mmu.write8(0x0001, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x31_ld_sp_n16_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x31);
cpu.mmu.write16(0x0001, 0x1234);
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0x1234);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x03);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x31_ld_sp_n16_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x31);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x32_ld_ind_hl_dec__ind_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x32);
cpu.registers.set_hl(0x0F00);
cpu.registers.set_a(0xFF);
cpu.tick();
assert_eq!(cpu.mmu.read8(0x0F00), 0xff);
assert_eq!(cpu.registers.hl(), 0xeff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x32_ld_ind_hl_dec__ind_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x32);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x33_inc_sp_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x33);
cpu.registers.set_$RRP(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x33_inc_sp_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x33);
cpu.registers.set_$RRP(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabce);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x34_inc_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x34);
cpu.mmu.write8(0xff00, 0xDE);
cpu.registers.set_hl(0xff00);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xff00);
assert_eq!(cpu.mmu.read8(0xff00), 0xdf);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x34_inc_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x34);
cpu.mmu.write8(0xff00, 0xEF);
cpu.registers.set_hl(0xff00);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xff00);
assert_eq!(cpu.mmu.read8(0xff00), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x35_dec_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x35);
cpu.mmu.write8(0xff00, 0xDF);
cpu.registers.set_hl(0xff00);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xff00);
assert_eq!(cpu.mmu.read8(0xff00), 0xde);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x35_dec_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x35);
cpu.mmu.write8(0xff00, 0xF0);
cpu.registers.set_hl(0xff00);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xff00);
assert_eq!(cpu.mmu.read8(0xff00), 0xef);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x35_dec_ind_hl_ind_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x35);
cpu.mmu.write8(0xff00, 0x01);
cpu.registers.set_hl(0xff00);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xff00);
assert_eq!(cpu.mmu.read8(0xff00), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x36_ld_ind_hl_ind_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x36);
cpu.registers.set_hl(0x0F00);
cpu.mmu.write8(0x01, 0xFF);
cpu.tick();
assert_eq!(cpu.mmu.read8(0x0F00), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x36_ld_ind_hl_ind_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x36);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x38_jr__dec_4_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x38);
cpu.mmu.write8(0x0001, 0x12);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x14);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x38_jr__dec_4_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x38);
cpu.mmu.write8(0x0001, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.pc(), 0x01);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x39_add_hl_sp_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x39);
cpu.registers.set_$RRP(0x1111);
cpu.registers.set_hl(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0xbcde);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x39_add_hl_sp_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x39);
cpu.registers.set_$RRP(0x0001);
cpu.registers.set_hl(0x0FFF);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x1000);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x39_add_hl_sp_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x39);
cpu.registers.set_$RRP(0x0001);
cpu.registers.set_hl(0xFFFF);
cpu.tick();
assert_eq!(cpu.registers.hl(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3a_ld_a_ind_hl_dec__ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3a);
cpu.registers.set_hl(0x0F00);
cpu.mmu.write8(0x0F00, 0xFF);
cpu.tick();
assert_eq!(cpu.registers.a(), 0xff);
assert_eq!(cpu.registers.hl(), 0xeff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3a_ld_a_ind_hl_dec__ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3a);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3b_dec_sp_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3b);
cpu.registers.set_$RRP(0xABCD);
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3b_dec_sp_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3b);
cpu.registers.set_$RRP(0xABCD);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RRP(), 0xabcc);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3c_inc_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3c);
cpu.registers.set_$RY(0xFE);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3c_inc_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3c);
cpu.registers.set_$RY(0xFF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3c_inc_a_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3c);
cpu.registers.set_$RY(0xEF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xf0);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3d_dec_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3d);
cpu.registers.set_$RY(0xFF);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xfe);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3d_dec_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3d);
cpu.registers.set_$RY(0x00);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3d_dec_a_test2(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3d);
cpu.registers.set_$RY(0x01);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x00);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3e_ld_a_n8_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3e);
cpu.mmu.write8(0x01, 0xAB);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x02);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x3e_ld_a_n8_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x3e);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x40_ld_b_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x40);
cpu.registers.set_b(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x40_ld_b_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x40);
cpu.registers.set_b(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x41_ld_b_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x41);
cpu.registers.set_c(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x41_ld_b_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x41);
cpu.registers.set_c(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x42_ld_b_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x42);
cpu.registers.set_d(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x42_ld_b_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x42);
cpu.registers.set_d(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x43_ld_b_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x43);
cpu.registers.set_e(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x43_ld_b_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x43);
cpu.registers.set_e(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x44_ld_b_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x44);
cpu.registers.set_h(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x44_ld_b_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x44);
cpu.registers.set_h(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x45_ld_b_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x45);
cpu.registers.set_l(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x45_ld_b_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x45);
cpu.registers.set_l(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x46_ld_b_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x46);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.b(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x46_ld_b_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x46);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x47_ld_b_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x47);
cpu.registers.set_a(0x66);
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x47_ld_b_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x47);
cpu.registers.set_a(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.b(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x48_ld_c_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x48);
cpu.registers.set_b(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x48_ld_c_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x48);
cpu.registers.set_b(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x49_ld_c_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x49);
cpu.registers.set_c(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x49_ld_c_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x49);
cpu.registers.set_c(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4a_ld_c_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4a);
cpu.registers.set_d(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4a_ld_c_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4a);
cpu.registers.set_d(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4b_ld_c_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4b);
cpu.registers.set_e(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4b_ld_c_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4b);
cpu.registers.set_e(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4c_ld_c_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4c);
cpu.registers.set_h(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4c_ld_c_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4c);
cpu.registers.set_h(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4d_ld_c_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4d);
cpu.registers.set_l(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4d_ld_c_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4d);
cpu.registers.set_l(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4e_ld_c_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.c(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4e_ld_c_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4f_ld_c_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4f);
cpu.registers.set_a(0x66);
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x4f_ld_c_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x4f);
cpu.registers.set_a(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.c(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x50_ld_d_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x50);
cpu.registers.set_b(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x50_ld_d_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x50);
cpu.registers.set_b(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x51_ld_d_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x51);
cpu.registers.set_c(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x51_ld_d_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x51);
cpu.registers.set_c(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x52_ld_d_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x52);
cpu.registers.set_d(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x52_ld_d_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x52);
cpu.registers.set_d(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x53_ld_d_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x53);
cpu.registers.set_e(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x53_ld_d_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x53);
cpu.registers.set_e(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x54_ld_d_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x54);
cpu.registers.set_h(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x54_ld_d_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x54);
cpu.registers.set_h(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x55_ld_d_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x55);
cpu.registers.set_l(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x55_ld_d_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x55);
cpu.registers.set_l(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x56_ld_d_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x56);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.d(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x56_ld_d_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x56);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x57_ld_d_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x57);
cpu.registers.set_a(0x66);
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x57_ld_d_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x57);
cpu.registers.set_a(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.d(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x58_ld_e_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x58);
cpu.registers.set_b(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x58_ld_e_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x58);
cpu.registers.set_b(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x59_ld_e_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x59);
cpu.registers.set_c(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x59_ld_e_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x59);
cpu.registers.set_c(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5a_ld_e_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5a);
cpu.registers.set_d(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5a_ld_e_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5a);
cpu.registers.set_d(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5b_ld_e_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5b);
cpu.registers.set_e(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5b_ld_e_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5b);
cpu.registers.set_e(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5c_ld_e_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5c);
cpu.registers.set_h(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5c_ld_e_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5c);
cpu.registers.set_h(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5d_ld_e_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5d);
cpu.registers.set_l(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5d_ld_e_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5d);
cpu.registers.set_l(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5e_ld_e_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.e(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5e_ld_e_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5f_ld_e_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5f);
cpu.registers.set_a(0x66);
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x5f_ld_e_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x5f);
cpu.registers.set_a(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.e(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x60_ld_h_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x60);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x60_ld_h_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x60);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x61_ld_h_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x61);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x61_ld_h_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x61);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x62_ld_h_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x62);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x62_ld_h_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x62);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x63_ld_h_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x63);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x63_ld_h_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x63);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x64_ld_h_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x64);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x64_ld_h_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x64);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x65_ld_h_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x65);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x65_ld_h_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x65);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x66_ld_h_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x66);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x66_ld_h_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x66);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x67_ld_h_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x67);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x67_ld_h_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x67);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x68_ld_l_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x68);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x68_ld_l_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x68);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x69_ld_l_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x69);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x69_ld_l_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x69);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6a_ld_l_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6a);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6a_ld_l_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6a);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6b_ld_l_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6b);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6b_ld_l_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6b);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6c_ld_l_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6c);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6c_ld_l_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6c);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6d_ld_l_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6d);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6d_ld_l_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6d);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6e_ld_l_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6e_ld_l_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6f_ld_l_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6f);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x6f_ld_l_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x6f);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x70_ld_ind_hl_ind_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x70);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x70_ld_ind_hl_ind_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x70);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x71_ld_ind_hl_ind_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x71);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x71_ld_ind_hl_ind_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x71);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x72_ld_ind_hl_ind_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x72);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x72_ld_ind_hl_ind_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x72);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x73_ld_ind_hl_ind_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x73);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x73_ld_ind_hl_ind_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x73);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x74_ld_ind_hl_ind_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x74);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x74_ld_ind_hl_ind_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x74);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x75_ld_ind_hl_ind_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x75);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x75_ld_ind_hl_ind_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x75);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x77_ld_ind_hl_ind_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x77);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x77_ld_ind_hl_ind_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x77);
cpu.registers.set_$RZ(0xff);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.mmu.read8(0xffff), 0xff);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x78_ld_a_b_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x78);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x78_ld_a_b_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x78);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x79_ld_a_c_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x79);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x79_ld_a_c_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x79);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7a_ld_a_d_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7a);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7a_ld_a_d_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7a);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7b_ld_a_e_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7b);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7b_ld_a_e_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7b);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7c_ld_a_h_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7c);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7c_ld_a_h_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7c);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7d_ld_a_l_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7d);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7d_ld_a_l_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7d);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7e_ld_a_ind_hl_ind_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7e_ld_a_ind_hl_ind_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7e);
cpu.mmu.write8(0xffff, 0xAB);
cpu.registers.set_hl(0xffff);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0xab);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7f_ld_a_a_test0(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7f);
cpu.registers.set_$RZ(0x66);
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x00 != 0);
assert_eq!(cpu.registers.flag_n(), 0x00 != 0);
assert_eq!(cpu.registers.flag_h(), 0x00 != 0);
assert_eq!(cpu.registers.flag_c(), 0x00 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
#[test]
                #[allow(non_snake_case)]
fn test_op_0x7f_ld_a_a_test1(){
let mut cpu = create_test_cpu();
cpu.mmu.write8(0x00, 0x7f);
cpu.registers.set_$RZ(0x66);
cpu.registers.enable_all_flags();
cpu.tick();
assert_eq!(cpu.registers.$RY(), 0x66);
assert_eq!(cpu.registers.flag_z(), 0x01 != 0);
assert_eq!(cpu.registers.flag_n(), 0x01 != 0);
assert_eq!(cpu.registers.flag_h(), 0x01 != 0);
assert_eq!(cpu.registers.flag_c(), 0x01 != 0);
assert_eq!(cpu.registers.pc(), 0x01);
}
}

#[derive(Default)]
            struct Registers{
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
            }
            struct CPU {
                pub registers: Registers
            }

            pub fn opcode(op: u32, cpu: &mut CPU) {
                match(op) {

        (0) => {
            
        }
        

        (1) => {
            
        }
        

        (2) => {
            
        }
        

        (3) => {
            
        }
        

        (4) => {
            
        }
        

        (5) => {
            
        }
        

        (6) => {
            
        }
        

        (7) => {
            
        }
        

        (64) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (65) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (66) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (67) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (68) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (69) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (70) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (71) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (80) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (81) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (82) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (83) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (84) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (85) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (86) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (87) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (88) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (89) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (90) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (91) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (92) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (93) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (94) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (95) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (96) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (97) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (98) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (99) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (100) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (101) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (102) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (103) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (104) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (105) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (106) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (107) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (108) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (109) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (110) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (111) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (112) => {
            HLT
        }
        

        (113) => {
            HLT
        }
        

        (114) => {
            HLT
        }
        

        (115) => {
            HLT
        }
        

        (116) => {
            HLT
        }
        

        (117) => {
            HLT
        }
        

        (118) => {
            HLT
        }
        

        (119) => {
            HLT
        }
        

        (120) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (121) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (122) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (123) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (124) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (125) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (126) => {
            $ REG_Y = $ REG_Z ;
        }
        

        (127) => {
            $ REG_Y = $ REG_Z ;
        }
        
}

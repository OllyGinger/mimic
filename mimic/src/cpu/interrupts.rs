use super::cpu::CPU;
use bitflags::bitflags;

bitflags!(
    pub struct InterruptFlag: u8 {
        const VBLANK = 0b_0000_0001;
        const LCD = 0b_0000_0010;
        const TIMER = 0b_0000_0100;
        const SERIAL = 0b_0000_1000;
        const JOYPAD = 0b_0001_0000;
    }
);

impl CPU {
    fn update_ime(&mut self) {
        self.setdi = match self.setdi {
            2 => 1,
            1 => {
                self.registers.set_ime(false);
                0
            }
            _ => 0,
        };
        self.setei = match self.setei {
            2 => 1,
            1 => {
                self.registers.set_ime(true);
                0
            }
            _ => 0,
        };
    }

    pub fn handle_interrupts(&mut self) -> u32 {
        self.update_ime();
        let triggered_int =
            self.mmu.interrupt_flag & self.mmu.read8(0xffff /* interrupt enable */);
        if triggered_int == 0 {
            return 0;
        }

        if self.registers.ime() == false && !self.halt {
            return 0;
        }
        self.registers.set_ime(false);

        let int = triggered_int.trailing_zeros();
        if int >= 5 {
            panic!("Invalid interrupt: {}", 0x0040 | ((int as u16) << 3));
        }

        self.mmu.interrupt_flag = self.mmu.interrupt_flag & !(1 << int);
        self.push_stack(self.registers.pc());
        self.registers.set_pc(0x0040 | ((int as u16) << 3));

        self.halt = false;

        1
    }
}

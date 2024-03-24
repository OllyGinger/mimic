use bitflags::bitflags;
use gilrs::Button;
use imgui::Ui;

use crate::{
    memory::memory::{Memory, MemoryRangeInclusive},
    tickable::Tickable,
};

bitflags! (
    struct ButtonState : u8 {
        const RIGHT_A =         0b_0000_0001;
        const LEFT_B =          0b_0000_0010;
        const UP_SELECT =       0b_0000_0100;
        const DOWN_START =      0b_0000_1000;
        const SELECT_DIR =      0b_0001_0000;
        const SELECT_ACTION =   0b_0010_0000;
        const WRITABLE =        ButtonState::SELECT_DIR.bits | ButtonState::SELECT_ACTION.bits;
        const INITIAL_STATE =   ButtonState::WRITABLE.bits;
    }
);

pub struct Keypad {
    mapped_ranges: Vec<MemoryRangeInclusive>,

    gamepad_lib: Box<gilrs::Gilrs>,
    active_pad: Option<gilrs::GamepadId>,
    state: ButtonState,
    dir_state: ButtonState,
    action_state: ButtonState,
    pub interrupt: u8,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            mapped_ranges: vec![0xFF00..=0xFF00],
            gamepad_lib: Box::new(gilrs::Gilrs::new().unwrap()),
            active_pad: None,
            state: ButtonState::INITIAL_STATE,
            dir_state: ButtonState::empty(),
            action_state: ButtonState::empty(),
            interrupt: 0x0,
        }
    }

    fn update_state(&mut self) {
        self.state &= ButtonState::WRITABLE;
        if self.state.contains(ButtonState::SELECT_DIR) {
            self.state.insert(self.dir_state);
        }
        if self.state.contains(ButtonState::SELECT_ACTION) {
            self.state.insert(self.action_state);
        }
    }

    pub fn draw_window(&mut self, ui: &mut Ui) {
        ui.window("Keypad").build(|| {
            ui.text(format!("dir_state: {:04b}", self.dir_state.bits & 0x30));
            ui.text(format!("act_state: {:04b}", self.action_state.bits & 0x30));
            ui.separator();
            ui.text("UP: ");
            ui.same_line();
            ui.text(if self.dir_state.contains(ButtonState::UP_SELECT) {
                "Pressed"
            } else {
                ""
            });

            ui.text("DOWN: ");
            ui.same_line();
            ui.text(if self.dir_state.contains(ButtonState::DOWN_START) {
                "Pressed"
            } else {
                ""
            });

            ui.text("LEFT: ");
            ui.same_line();
            ui.text(if self.dir_state.contains(ButtonState::LEFT_B) {
                "Pressed"
            } else {
                ""
            });

            ui.text("RIGHT: ");
            ui.same_line();
            ui.text(if self.dir_state.contains(ButtonState::RIGHT_A) {
                "Pressed"
            } else {
                ""
            });

            ui.text("A: ");
            ui.same_line();
            ui.text(if self.action_state.contains(ButtonState::RIGHT_A) {
                "Pressed"
            } else {
                ""
            });

            ui.text("B: ");
            ui.same_line();
            ui.text(if self.action_state.contains(ButtonState::LEFT_B) {
                "Pressed"
            } else {
                ""
            });

            ui.text("START: ");
            ui.same_line();
            ui.text(if self.action_state.contains(ButtonState::DOWN_START) {
                "Pressed"
            } else {
                ""
            });

            ui.text("SELECT: ");
            ui.same_line();
            ui.text(if self.action_state.contains(ButtonState::UP_SELECT) {
                "Pressed"
            } else {
                ""
            });
        });
    }
}

impl Memory for Keypad {
    fn read8(&self, address: u16) -> u8 {
        self.try_read8(address)
            .expect(&format!("Unmapped address: {:#06X}", address))
    }

    fn try_read8(&self, address: u16) -> Option<u8> {
        match address {
            0xFF00 => Some(!self.state.bits),
            _ => panic!("Un-mapped timer address: {:06x}", address),
        }
    }

    fn write8(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => {
                self.state = ButtonState::from_bits_truncate(!value);
                self.update_state();
            }
            _ => panic!("Un-mapped keypad address: {:06x}", address),
        };
    }

    fn mapped_ranges(&self) -> &Vec<MemoryRangeInclusive> {
        &self.mapped_ranges
    }
}

impl Tickable for Keypad {
    fn tick(&mut self, ticks: u8) {
        // Needed to read input
        while let Some(ev) = self.gamepad_lib.next_event() {
            self.active_pad = Some(ev.id);
            match ev.event {
                gilrs::EventType::ButtonPressed(btn, _) => {
                    match btn {
                        gilrs::Button::DPadRight => self.dir_state.insert(ButtonState::RIGHT_A),
                        gilrs::Button::DPadLeft => self.dir_state.insert(ButtonState::LEFT_B),
                        gilrs::Button::DPadUp => self.dir_state.insert(ButtonState::UP_SELECT),
                        gilrs::Button::DPadDown => self.dir_state.insert(ButtonState::DOWN_START),
                        gilrs::Button::East => self.action_state.insert(ButtonState::RIGHT_A),
                        gilrs::Button::South => self.action_state.insert(ButtonState::LEFT_B),
                        gilrs::Button::Start => self.action_state.insert(ButtonState::DOWN_START),
                        gilrs::Button::Select => self.action_state.insert(ButtonState::UP_SELECT),
                        _ => {}
                    };
                    self.update_state();
                    self.interrupt = 0x10;
                }
                gilrs::EventType::ButtonReleased(btn, _) => {
                    match btn {
                        gilrs::Button::DPadRight => self.dir_state.remove(ButtonState::RIGHT_A),
                        gilrs::Button::DPadLeft => self.dir_state.remove(ButtonState::LEFT_B),
                        gilrs::Button::DPadUp => self.dir_state.remove(ButtonState::UP_SELECT),
                        gilrs::Button::DPadDown => self.dir_state.remove(ButtonState::DOWN_START),
                        gilrs::Button::East => self.action_state.remove(ButtonState::RIGHT_A),
                        gilrs::Button::South => self.action_state.remove(ButtonState::LEFT_B),
                        gilrs::Button::Start => self.action_state.remove(ButtonState::DOWN_START),
                        gilrs::Button::Select => self.action_state.remove(ButtonState::UP_SELECT),
                        _ => {}
                    };
                    self.update_state();
                }
                _ => {}
            }
        }
    }
}

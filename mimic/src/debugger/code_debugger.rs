use std::collections::BTreeSet;

use imgui::{
    ListClipper, SelectableFlags, StyleColor, TableBgTarget, TableColumnFlags, TableColumnSetup,
    TableFlags, WindowFlags,
};

use super::colours::*;
use crate::cpu::{
    cpu::CPU,
    disassembler::{Disassembler, DisassemblyLine},
};
use material_symbols::Symbol::*;

pub struct CodeDebugger {
    disassembly: Vec<DisassemblyLine>,

    pub breakpoints: BTreeSet<u16>,
}

impl CodeDebugger {
    pub fn new() -> CodeDebugger {
        CodeDebugger {
            disassembly: Vec::new(),

            breakpoints: BTreeSet::new(),
        }
    }

    pub fn draw(&mut self, ui: &mut imgui::Ui, cpu: &mut CPU) {
        let window = ui.window("Code Debugger");
        window
            .position([0.0, 0.0], imgui::Condition::Always)
            .flags(
                WindowFlags::NO_SCROLLBAR
                    | WindowFlags::NO_SCROLL_WITH_MOUSE
                    | WindowFlags::NO_COLLAPSE
                    | WindowFlags::NO_RESIZE,
            )
            //.size([560.0, ui.window_size()[1] * 2.5], imgui::Condition::Always)
            .build(|| {
                // Create a table here, so we can reisze various parts of the UI
                if let Some(_t) = ui.begin_table_with_flags(
                    "two_column",
                    2,
                    TableFlags::RESIZABLE | TableFlags::BORDERS,
                ) {
                    ui.table_setup_column_with(TableColumnSetup {
                        init_width_or_weight: -1.0,
                        flags: TableColumnFlags::NO_SORT | TableColumnFlags::WIDTH_STRETCH,
                        ..TableColumnSetup::new("code_column")
                    });
                    ui.table_setup_column_with(TableColumnSetup {
                        init_width_or_weight: 200.0,
                        flags: TableColumnFlags::WIDTH_FIXED,
                        ..TableColumnSetup::new("registers_column")
                    });

                    ui.table_next_row();
                    ui.table_next_column();
                    self.draw_disassembly_window(ui, cpu);

                    ui.table_next_column();
                    self.draw_debugger_control_box(ui, cpu);
                    self.draw_registers(ui, cpu);
                    ui.text("hi");
                }
            });
    }

    fn draw_disassembly_window(&mut self, ui: &imgui::Ui, cpu: &CPU) {
        if self.disassembly.is_empty() {
            self.disassembly = Disassembler::new().disassemble(&cpu.mmu);
        }

        let flags = TableFlags::BORDERS_V | TableFlags::SCROLL_Y; //| TableFlags::RESIZABLE;

        if let Some(_t) = ui.begin_table_with_sizing("disassembly", 4, flags, [480.0, -1.0], 0.0) {
            ui.table_setup_column_with(TableColumnSetup {
                init_width_or_weight: 20.0,
                flags: TableColumnFlags::WIDTH_FIXED,
                ..TableColumnSetup::new("")
            });
            ui.table_setup_column_with(TableColumnSetup {
                init_width_or_weight: 90.0,
                flags: TableColumnFlags::WIDTH_FIXED,
                ..TableColumnSetup::new("Address")
            });
            ui.table_setup_column_with(TableColumnSetup {
                init_width_or_weight: 100.0,
                flags: TableColumnFlags::WIDTH_FIXED,
                ..TableColumnSetup::new("Bytes")
            });
            ui.table_setup_column("Instruction");
            ui.table_setup_scroll_freeze(3, 1);
            ui.table_headers_row();

            let clipper = ListClipper::new(self.disassembly.len() as i32).begin(&ui);
            for row_num in clipper.iter() {
                let row_data = &self.disassembly[row_num as usize];
                let row_start_addr = row_data.address_range.start();

                ui.table_next_row();
                let address_id = ui.push_id(format!("{}", row_data.address_range.start()));

                // Highlight if this row is the current PC
                let is_crrent_instruction_row = cpu.registers.pc()
                    >= *row_data.address_range.start()
                    && cpu.registers.pc() < *row_data.address_range.end();
                if is_crrent_instruction_row {
                    ui.table_set_bg_color(TableBgTarget::ROW_BG0, COLOUR_DISASSEMBLY_ROW_PC);
                }

                // Highlight if this is a breakpoint
                if self.breakpoints.contains(row_start_addr) {
                    ui.table_set_bg_color(
                        TableBgTarget::ROW_BG0,
                        COLOUR_DEBUGGER_CODE_BREAKPOINT_ROW,
                    );
                }

                let bytes = row_data
                    .bytes
                    .iter()
                    .map(|&x| format!("{:02X}", x))
                    .collect::<Vec<String>>()
                    .join(" ");

                // Fill columns with data
                ui.table_next_column();

                let pos = ui.cursor_pos();
                let breakpoint_cell_clicked = ui
                    .selectable_config("##breakpoint_cell")
                    .allow_double_click(true)
                    .build();
                if breakpoint_cell_clicked {
                    if self.breakpoints.contains(row_start_addr) {
                        self.breakpoints.remove(row_start_addr);
                    } else {
                        self.breakpoints.insert(*row_start_addr);
                    }
                }
                ui.set_cursor_pos(pos);
                if is_crrent_instruction_row {
                    ui.text_colored(
                        COLOUR_LIGHT_YELLOW,
                        format!("{}", material_symbols::Symbol::ArrowRight.codepoint()),
                    );
                } else {
                    ui.text("");
                }
                ui.same_line();
                if self.breakpoints.contains(row_start_addr) {
                    ui.text_colored(
                        COLOUR_DEBUGGER_CODE_BREAKPOINT,
                        format!(
                            "{}",
                            material_symbols::Symbol::RadioButtonChecked.codepoint()
                        ),
                    );
                }

                ui.table_next_column();
                ui.text(format!("{:#06x}", row_data.address_range.start()));
                ui.table_next_column();
                ui.text(bytes);
                ui.table_next_column();
                ui.text(&row_data.instruction);

                address_id.pop();
            }
        }
    }

    fn draw_debugger_control_box(&mut self, ui: &imgui::Ui, cpu: &mut CPU) {
        // Run/Break
        if cpu.is_broken_to_debugger() {
            let play_button_style = ui.push_style_color(StyleColor::Text, COLOUR_DEBUGGER_CONTINUE);
            if ui.button(format!("{}", PlayArrow.codepoint())) {
                cpu.resume_from_debugger();
            }
            play_button_style.pop();
        } else {
            let break_button_style = ui.push_style_color(StyleColor::Text, COLOUR_DEBUGGER_STEP);
            if ui.button(format!("{}", Pause.codepoint())) {
                cpu.break_to_debugger();
            }
            break_button_style.pop();
        }
        ui.same_line();

        // Step
        let step_button_style = ui.push_style_color(StyleColor::Text, COLOUR_DEBUGGER_STEP);
        if ui.button(format!("{}", StepOver.codepoint())) {
            cpu.debug_single_step(true);
        }
        ui.same_line();
        ui.button(format!("{}", StepInto.codepoint()));
        ui.same_line();
        ui.button(format!("{}", StepOut.codepoint()));
        ui.same_line();
        step_button_style.pop();

        // Restart
        let restart_button_style = ui.push_style_color(StyleColor::Text, COLOUR_DEBUGGER_CONTINUE);
        ui.button(format!("{}", RestartAlt.codepoint()));
        restart_button_style.pop();
        ui.same_line();

        // Stop
        let stop_button_style = ui.push_style_color(StyleColor::Text, COLOUR_DEBUGGER_STOP);
        ui.button(format!("{}", Stop.codepoint()));
        stop_button_style.pop();
    }

    fn draw_registers(&mut self, ui: &imgui::Ui, cpu: &CPU) {
        let flags = TableFlags::BORDERS_V | TableFlags::BORDERS_V | TableFlags::SCROLL_Y;

        if let Some(_t) = ui.begin_table_with_sizing("cpu", 2, flags, [-1.0, 200.0], 0.0) {
            ui.table_setup_column("Register");
            ui.table_setup_column("Value");
            ui.table_headers_row();

            ui.table_next_row();
            ui.table_next_column();
            ui.text("A");
            ui.table_next_column();
            ui.text(format!("{:#04X}", cpu.registers.a()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("B");
            ui.table_next_column();
            ui.text(format!("{:#04X}", cpu.registers.b()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("C");
            ui.table_next_column();
            ui.text(format!("{:#04X}", cpu.registers.c()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("D");
            ui.table_next_column();
            ui.text(format!("{:#04X}", cpu.registers.d()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("E");
            ui.table_next_column();
            ui.text(format!("{:#04X}", cpu.registers.e()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("HL");
            ui.table_next_column();
            ui.text(format!("{:#06X}", cpu.registers.hl()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("SP");
            ui.table_next_column();
            ui.text(format!("{:#06X}", cpu.registers.sp()));

            ui.table_next_row();
            ui.table_next_column();
            ui.text("PC");
            ui.table_next_column();
            ui.text(format!("{:#06X}", cpu.registers.pc()));
        }
    }
}

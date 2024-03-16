use imgui::{ListClipper, TableFlags};
use material_icons::Icon;

use crate::cpu::{
    cpu::CPU,
    disassembler::{Disassembler, DisassemblyLine},
};

pub struct CodeDebugger {
    disassembly: Vec<DisassemblyLine>,
}

impl CodeDebugger {
    pub fn new() -> CodeDebugger {
        CodeDebugger {
            disassembly: Vec::new(),
        }
    }

    pub fn draw(&mut self, ui: &mut imgui::Ui, cpu: &CPU) {
        if self.disassembly.is_empty() {
            self.disassembly = Disassembler::new().disassemble(&cpu.mmu);
        }

        let window = ui.window("hi");
        window
            .position([0.0, 0.0], imgui::Condition::Always)
            .size([550.0, ui.window_size()[1] * 2.5], imgui::Condition::Always)
            .build(|| {
                let flags = TableFlags::BORDERS_V | TableFlags::SCROLL_Y;

                if let Some(_t) = ui.begin_table_with_sizing("cpu", 3, flags, [400.0, -1.0], 0.0) {
                    ui.table_setup_column(format!(
                        "Address {}",
                        material_icons::icon_to_char(Icon::Save)
                    ));
                    ui.table_setup_column("Bytes");
                    ui.table_setup_column("Instruction");
                    ui.table_setup_scroll_freeze(3, 1);
                    ui.table_headers_row();

                    let clipper = ListClipper::new(self.disassembly.len() as i32).begin(&ui);
                    for row_num in clipper.iter() {
                        ui.table_next_row();

                        let row_data = &self.disassembly[row_num as usize];
                        let bytes = row_data
                            .bytes
                            .iter()
                            .map(|&x| format!("{:02X}", x))
                            .collect::<Vec<String>>()
                            .join(" ");

                        // Fill columns with data
                        ui.table_next_column();
                        ui.text(format!("{:#06x}", row_data.address_range.start()));
                        ui.table_next_column();
                        ui.text(bytes);
                        ui.table_next_column();
                        ui.text(&row_data.instruction);
                    }
                }
            });
    }
}

// Virtual List Clipper
/*
let data: Vec<RowData> = (1..=100)
        .map(|i| RowData {
            id: i,
            name: format!("Name {}", i),
            age: i % 50 + 20, // Just for demonstration purposes
        })
        .collect();


         let window = imgui::Window::new(im_str!("ListClipper Table"))
            .position([10.0, 10.0], Condition::FirstUseEver)
            .size([400.0, 400.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.columns(3, im_str!("MyColumns"), true);

                // Table headers
                ui.text(im_str!("ID"));
                ui.next_column();
                ui.text(im_str!("Name"));
                ui.next_column();
                ui.text(im_str!("Age"));
                ui.next_column();

                ui.separator();

                // Calculate visible portion of the table based on scroll position
                let mut clipper = ListClipper::new(data.len() as i32).begin(&ui);
                while clipper.step() {
                    for i in clipper.display_start()..clipper.display_end() {
                        let row_data = &data[i as usize];

                        // Begin table row
                        ui.table_next_row(TableRowFlags::empty(), 0.0);

                        // Fill columns with data
                        ui.table_set_column_index(0);
                        ui.text(im_str!("{}", row_data.id));
                        ui.table_set_column_index(1);
                        ui.text(im_str!("{}", row_data.name));
                        ui.table_set_column_index(2);
                        ui.text(im_str!("{}", row_data.age));
                    }
                }
            });
*/

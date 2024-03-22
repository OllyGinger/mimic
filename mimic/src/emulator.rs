use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use imgui::Ui;
use imgui_glium_renderer::Renderer;

use crate::cartridge::{self};
use crate::cpu::cpu::CPU;
use crate::debugger::code_debugger::CodeDebugger;
use crate::debugger::tile_debug_view::TileDebugView;
use crate::debugger::tile_map_debug_view::TileMapDebugView;
use crate::gpu::gpu::GPU;
use crate::memory::mmu::MMU;
use crate::{io, main_window};

pub struct Emulator {
    pub cpu: CPU,
    gpu: Rc<RefCell<GPU>>,

    last_update: Instant,
    code_debugger: CodeDebugger,
    tile_debug_view: TileDebugView,
    tile_map_debug_view: TileMapDebugView,
}

impl Emulator {
    pub fn start(boot_rom_path: Option<String>, rom_path: String) {
        let rom_bytes = std::fs::read(rom_path).unwrap();
        let boot_rom_bytes: Option<Vec<u8>> = if let Some(path) = boot_rom_path {
            Some(std::fs::read(path).unwrap())
        } else {
            None
        };
        let cart = cartridge::new(boot_rom_bytes, rom_bytes);

        let mut mmu = MMU::new();
        mmu.map_cartridge(Rc::new(RefCell::new(cart)));

        let gpu = Rc::new(RefCell::new(GPU::new()));
        mmu.add_interface(gpu.clone());

        let audio = Rc::new(RefCell::new(io::audio::Audio::new()));
        mmu.add_interface(audio.clone());

        let cpu = CPU::new(mmu);
        let main_window = main_window::new("Mimic".to_string(), 2048, 1024);

        let mut em = Emulator {
            cpu,
            gpu: gpu.clone(),

            last_update: Instant::now(),
            code_debugger: CodeDebugger::new(),
            tile_debug_view: TileDebugView::new(
                main_window.display.clone(),
                main_window.renderer.clone(),
            ),
            tile_map_debug_view: TileMapDebugView::new(
                main_window.display.clone(),
                main_window.renderer.clone(),
            ),
        };

        main_window.main_loop(
            move |_keep_running: &mut bool,
                  ui: &mut Ui,
                  renderer: Rc<RefCell<Renderer>>,
                  display: Rc<RefCell<glium::Display>>| {
                em.update(ui, renderer, display);
            },
        );
    }

    fn update(
        &mut self,
        ui: &mut imgui::Ui,
        renderer: Rc<RefCell<Renderer>>,
        _display: Rc<RefCell<glium::Display>>,
    ) {
        if let Some(_menu_bar) = ui.begin_main_menu_bar() {
            if let Some(file_menu) = ui.begin_menu("File") {
                if ui.menu_item("Quit") {
                    std::process::exit(0);
                }
                file_menu.end();
            }

            if let Some(edit_menu) = ui.begin_menu("File") {
                if ui.menu_item("Save window layout") {}
                edit_menu.end();
            }
        }

        let code_debugger = &mut self.code_debugger;
        code_debugger.draw(ui, &mut self.cpu);

        let tile_debug_view = &mut self.tile_debug_view;
        tile_debug_view.draw(renderer.clone(), ui, &self.gpu.borrow());

        let tile_map_debug_view = &mut self.tile_map_debug_view;
        tile_map_debug_view.draw(renderer.clone(), ui, &self.gpu.borrow());

        self.tick_cpu();
    }

    fn tick_cpu(&mut self) {
        const CPU_SPEED_HZ: u32 = 4194304;
        let delta = Instant::now() - self.last_update;

        let available_ticks = ((delta * CPU_SPEED_HZ).as_secs() as u64).min(100000);
        let mut ticks = 0;

        if !self.cpu.is_broken_to_debugger() {
            loop {
                ticks += self.cpu.tick();

                if self
                    .code_debugger
                    .breakpoints
                    .contains(&self.cpu.registers.pc())
                {
                    self.cpu.break_to_debugger();
                    break;
                }

                if ticks as u64 > available_ticks {
                    break;
                }
            }
        } else {
            if self.cpu.wants_single_step() {
                self.cpu.tick();

                // Reset the single step flag so it can be clicked again next time
                self.cpu.debug_single_step(false);
            }
        }

        self.last_update = Instant::now();
    }
}

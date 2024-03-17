use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::*;
use imgui::{Context, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use std::ops::IndexMut;
use std::time::Instant;

pub struct MainWindow {
    event_loop: EventLoop<()>,
    display: glium::Display,
    imgui: Context,
    platform: WinitPlatform,
    renderer: Renderer,
}

pub fn new(title: String, width: u32, height: u32) -> MainWindow {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_inner_size(glutin::dpi::LogicalSize::new(width as f64, height as f64));
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../resources/RobotoMono-Regular.ttf"),
            size_pixels: 18.0,
            config: Some(FontConfig {
                rasterizer_multiply: 1.5,
                oversample_h: 4,
                oversample_v: 4,
                glyph_offset: [0.0, -1.0],
                ..FontConfig::default()
            }),
        },
        //FontSource::TtfData {
        //    data: include_bytes!("../resources/MaterialIcons-Regular.ttf"),
        //    size_pixels: 18.0,
        //    config: Some(FontConfig {
        //        rasterizer_multiply: 1.5,
        //        oversample_h: 4,
        //        oversample_v: 4,
        //        glyph_offset: [0.0, 3.0],
        //        glyph_ranges: FontGlyphRanges::from_slice(&[0xe000, 0xf23b, 0]),
        //        ..FontConfig::default()
        //    }),
        //},
        FontSource::TtfData {
            data: include_bytes!("../resources/MaterialSymbols.ttf"),
            size_pixels: 20.0,
            config: Some(FontConfig {
                rasterizer_multiply: 1.5,
                oversample_h: 4,
                oversample_v: 4,
                glyph_offset: [0.0, 2.0],
                glyph_ranges: FontGlyphRanges::from_slice(&[0xe000, 0xf8ff, 0]),
                ..FontConfig::default()
            }),
        },
    ]);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        let dpi_mode = imgui_winit_support::HiDpiMode::Locked(1.0);
        platform.attach_window(imgui.io_mut(), window, dpi_mode);
    }
    let mut renderer = Renderer::init(&mut imgui, &display).unwrap();
    renderer.reload_font_texture(&mut imgui).unwrap();

    // Change some of the default colors - Dracula from: https://github.com/iddm/imgui_styles/blob/master/src/lib.rs
    let style = imgui.style_mut();
    let colors = style.colors.as_mut();
    colors[imgui::StyleColor::WindowBg as usize] = [0.1f32, 0.1f32, 0.13f32, 1.0f32];
    colors[imgui::StyleColor::MenuBarBg as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

    // Border
    colors[imgui::StyleColor::Border as usize] = [0.44f32, 0.37f32, 0.61f32, 0.29f32];
    colors[imgui::StyleColor::BorderShadow as usize] = [0.0f32, 0.0f32, 0.0f32, 0.24f32];

    // Text
    colors[imgui::StyleColor::Text as usize] = [1.0f32, 1.0f32, 1.0f32, 1.0f32];
    colors[imgui::StyleColor::TextDisabled as usize] = [0.5f32, 0.5f32, 0.5f32, 1.0f32];

    // Headers
    colors[imgui::StyleColor::Header as usize] = [0.13f32, 0.13f32, 0.17, 1.0f32];
    colors[imgui::StyleColor::HeaderHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
    colors[imgui::StyleColor::HeaderActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

    // Buttons
    colors[imgui::StyleColor::Button as usize] = [0.13f32, 0.13f32, 0.17, 1.0f32];
    colors[imgui::StyleColor::ButtonHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
    colors[imgui::StyleColor::ButtonActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
    colors[imgui::StyleColor::CheckMark as usize] = [0.74f32, 0.58f32, 0.98f32, 1.0f32];

    // Popups
    colors[imgui::StyleColor::PopupBg as usize] = [0.1f32, 0.1f32, 0.13f32, 0.92f32];

    // Slider
    colors[imgui::StyleColor::SliderGrab as usize] = [0.44f32, 0.37f32, 0.61f32, 0.54f32];
    colors[imgui::StyleColor::SliderGrabActive as usize] = [0.74f32, 0.58f32, 0.98f32, 0.54f32];

    // Frame BG
    colors[imgui::StyleColor::FrameBg as usize] = [0.13f32, 0.13, 0.17, 1.0f32];
    colors[imgui::StyleColor::FrameBgHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
    colors[imgui::StyleColor::FrameBgActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

    // Tabs
    colors[imgui::StyleColor::Tab as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
    colors[imgui::StyleColor::TabHovered as usize] = [0.24, 0.24f32, 0.32f32, 1.0f32];
    colors[imgui::StyleColor::TabActive as usize] = [0.2f32, 0.22f32, 0.27f32, 1.0f32];
    colors[imgui::StyleColor::TabUnfocused as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
    colors[imgui::StyleColor::TabUnfocusedActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

    // Title
    colors[imgui::StyleColor::TitleBg as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
    colors[imgui::StyleColor::TitleBgActive as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
    colors[imgui::StyleColor::TitleBgCollapsed as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];

    // Scrollbar
    colors[imgui::StyleColor::ScrollbarBg as usize] = [0.1f32, 0.1f32, 0.13f32, 1.0f32];
    colors[imgui::StyleColor::ScrollbarGrab as usize] = [0.16f32, 0.16f32, 0.21f32, 1.0f32];
    colors[imgui::StyleColor::ScrollbarGrabHovered as usize] = [0.19f32, 0.2f32, 0.25f32, 1.0f32];
    colors[imgui::StyleColor::ScrollbarGrabActive as usize] = [0.24f32, 0.24f32, 0.32f32, 1.0f32];

    // Seperator
    colors[imgui::StyleColor::Separator as usize] = [0.44f32, 0.37f32, 0.61f32, 1.0f32];
    colors[imgui::StyleColor::SeparatorHovered as usize] = [0.74f32, 0.58f32, 0.98f32, 1.0f32];
    colors[imgui::StyleColor::SeparatorActive as usize] = [0.84f32, 0.58f32, 1.0f32, 1.0f32];

    // Resize Grip
    colors[imgui::StyleColor::ResizeGrip as usize] = [0.44f32, 0.37f32, 0.61f32, 0.29f32];
    colors[imgui::StyleColor::ResizeGripHovered as usize] = [0.74f32, 0.58f32, 0.98f32, 0.29f32];
    colors[imgui::StyleColor::ResizeGripActive as usize] = [0.84f32, 0.58f32, 1.0f32, 0.29f32];

    // Docking
    //colors[imgui::StyleColor::DockingPreview as usize] = [0.44f32, 0.37f32, 0.61f32, 1.0f32];

    style.tab_rounding = 4.0f32;
    style.scrollbar_rounding = 9.0f32;
    style.window_rounding = 0.0f32;
    style.grab_rounding = 3.0f32;
    style.frame_rounding = 3.0f32;
    style.popup_rounding = 4.0f32;
    style.child_rounding = 4.0f32;

    MainWindow {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
    }
}

impl MainWindow {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let MainWindow {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                let mut run = true;
                run_ui(&mut run, ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color(0.05, 0.066, 0.09, 1.0);
                platform.prepare_render(ui, gl_window.window());
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}

use std::{borrow::Borrow, cell::RefCell, ops::Range, rc::Rc};

use glium::{pixel_buffer::PixelBuffer, Display, Texture2d};
use imgui::{TextureId, Ui};
use imgui_glium_renderer::Renderer;

use crate::{
    gpu::{self, gpu::GPU},
    memory::memory::Memory,
    utils,
};

pub struct TileMapDebugView {
    pixel_buffer: PixelBuffer<(u8, u8, u8)>,
    texture_id: Option<TextureId>,

    tile_map_base: u16,
    scyx_enabled: bool,
}

impl TileMapDebugView {
    const TEXTURE_WIDTH: usize = gpu::TILE_SIZE * gpu::BACK_BUFFER_TILES_WIDE;
    const TEXTURE_HEIGHT: usize = gpu::TILE_SIZE * gpu::BACK_BUFFER_TILES_HIGH;

    pub fn new(facade: Rc<RefCell<Display>>, renderer: Rc<RefCell<Renderer>>) -> TileMapDebugView {
        let facade_ref: &RefCell<Display> = facade.borrow();
        let facade_refref: &Display = &facade_ref.borrow();
        let mut view = TileMapDebugView {
            pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(
                facade_refref,
                Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize,
            ),
            texture_id: None,
            tile_map_base: 0x9800,
            scyx_enabled: true,
        };

        let tex2d = Texture2d::empty_with_format(
            facade_refref,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            Self::TEXTURE_WIDTH as u32,
            Self::TEXTURE_HEIGHT as u32,
        )
        .unwrap();

        view.texture_id = Some(utils::register_texture(Rc::new(tex2d), renderer));
        view
    }

    pub fn draw(
        self: &mut TileMapDebugView,
        renderer: Rc<RefCell<Renderer>>,
        ui: &mut Ui,
        gpu: &GPU,
    ) {
        let mut buffer: [gpu::PixelColour;
            Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize] =
            [(255u8, 255u8, 255u8); Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize];

        ui.window("VRam Tile Map Viewer").build(|| {
            // Draw out the tile map
            self.draw_tile_map(gpu, &mut buffer);
            self.pixel_buffer.write(&buffer[..]);

            let mut renderer_ref = renderer.borrow_mut();
            renderer_ref
                .textures()
                .get(self.texture_id.unwrap())
                .unwrap()
                .texture
                .mipmap(0)
                .unwrap()
                .raw_upload_from_pixel_buffer(
                    self.pixel_buffer.as_slice(),
                    0..Self::TEXTURE_WIDTH as u32,
                    0..Self::TEXTURE_HEIGHT as u32,
                    0..1,
                );

            ui.text("Map Base:");
            ui.same_line();
            if ui.radio_button_bool("0x9800", self.tile_map_base == 0x9800) {
                self.tile_map_base = 0x9800;
            }
            ui.same_line();
            if ui.radio_button_bool("0x9C00", self.tile_map_base == 0x9C00) {
                self.tile_map_base = 0x9C00;
            }
            ui.checkbox("scyc", &mut self.scyx_enabled);

            let img = imgui::Image::new(
                self.texture_id.unwrap(),
                [
                    Self::TEXTURE_WIDTH as f32 * 2.0,
                    Self::TEXTURE_HEIGHT as f32 * 2.0,
                ],
            );
            img.build(ui);
        });
    }

    fn draw_screen_rectangle(
        self: &TileMapDebugView,
        gpu: &GPU,
        buffer: &mut [gpu::PixelColour;
                 Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize],
    ) {
        // Draw rectangle around screen viewport represented by SCX and SCY
        let viewport_x = gpu.get_scx() as usize;
        let viewport_y = gpu.get_scy() as usize;
        let viewport_width = gpu::LCD_SCREEN_WIDTH;
        let viewport_height = gpu::LCD_SCREEN_HEIGHT;

        // Draw rectangle within texture bounds
        let rect_top = viewport_y;
        let rect_bottom = viewport_y + viewport_height - 1;
        let rect_left = viewport_x;
        let rect_right = viewport_x + viewport_width - 1;

        // Draw top and bottom lines within texture bounds
        for row in [rect_top, rect_bottom].iter() {
            let range = Self::get_texture_range(rect_left, *row, viewport_width, 1);
            let line: [gpu::PixelColour; gpu::LCD_SCREEN_WIDTH] =
                [(255u8, 0u8, 0u8); gpu::LCD_SCREEN_WIDTH];
            utils::superimpose(buffer, range, &line);
        }

        // Draw left and right lines within texture bounds
        for col in [rect_left, rect_right].iter() {
            for row in rect_top..=rect_bottom {
                let range = Self::get_texture_range(*col, row, 1, 1);
                let pixel: [gpu::PixelColour; 1] = [(255u8, 0u8, 0u8); 1];
                utils::superimpose(buffer, range, &pixel);
            }
        }

        // Draw rectangle portions that wrap around the edges of the texture
        if rect_top > rect_bottom {
            // Draw top portion
            let top_range = Self::get_texture_range(rect_left, 0, viewport_width, rect_top + 1);
            let top_line: [gpu::PixelColour; gpu::LCD_SCREEN_WIDTH] =
                [(255u8, 0u8, 0u8); gpu::LCD_SCREEN_WIDTH];
            utils::superimpose(buffer, top_range, &top_line);

            // Draw bottom portion
            let bottom_range = Self::get_texture_range(
                rect_left,
                rect_bottom,
                viewport_width,
                Self::TEXTURE_HEIGHT,
            );
            let bottom_line: [gpu::PixelColour; gpu::LCD_SCREEN_WIDTH] =
                [(255u8, 0u8, 0u8); gpu::LCD_SCREEN_WIDTH];
            utils::superimpose(buffer, bottom_range, &bottom_line);
        }

        if rect_left > rect_right {
            // Draw left portion
            let left_range = Self::get_texture_range(0, rect_top, rect_left + 1, viewport_height);
            for _row in rect_top..=rect_bottom {
                let pixel: [gpu::PixelColour; 1] = [(255u8, 0u8, 0u8); 1];
                utils::superimpose(buffer, left_range.clone(), &pixel);
            }

            // Draw right portion
            let right_range =
                Self::get_texture_range(rect_right, rect_top, Self::TEXTURE_WIDTH, viewport_height);
            for _row in rect_top..=rect_bottom {
                let pixel: [gpu::PixelColour; 1] = [(255u8, 0u8, 0u8); 1];
                utils::superimpose(buffer, right_range.clone(), &pixel);
            }
        }
    }

    fn draw_tile_map(
        self: &TileMapDebugView,
        gpu: &GPU,
        buffer: &mut [gpu::PixelColour;
                 Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize],
    ) {
        // Draw tiles
        for tile_y in 0..gpu::BACK_BUFFER_TILES_HIGH {
            for tile_x in 0..gpu::BACK_BUFFER_TILES_WIDE {
                let tile_index = gpu.read8(
                    self.tile_map_base
                        + (gpu::BACK_BUFFER_TILES_WIDE as u16 * tile_y as u16 + tile_x as u16),
                );
                let tile_buffer = gpu.get_bg_tile_as_pixels(tile_index as u16);

                // Copy the current tile into the buffer
                let x_pixel = tile_x * (gpu::TILE_SIZE);
                let y_pixel = tile_y * (gpu::TILE_SIZE);

                // Copy each row of the tile
                for row_num in 0..gpu::TILE_SIZE {
                    let row: [(u8, u8, u8); 8] = tile_buffer
                        [(row_num * gpu::TILE_SIZE)..(row_num * gpu::TILE_SIZE) + gpu::TILE_SIZE]
                        .try_into()
                        .unwrap();

                    let dest_range =
                        Self::get_texture_range(x_pixel, y_pixel + row_num, gpu::TILE_SIZE, 1);

                    utils::superimpose(buffer, dest_range, &row);
                }
            }
        }

        // Draw screen rectangle
        if self.scyx_enabled {
            self.draw_screen_rectangle(gpu, buffer);
        }
    }

    fn get_texture_range(x: usize, y: usize, width: usize, height: usize) -> Range<usize> {
        let start = Self::TEXTURE_WIDTH as usize * y as usize + x as usize;
        let end = start + (width * height);

        let max_index = Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize;
        let end_clamped = end.min(max_index); // Clamp end index to maximum length

        start..end_clamped
    }
}

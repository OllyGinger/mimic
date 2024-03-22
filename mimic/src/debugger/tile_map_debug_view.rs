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
    }

    fn get_texture_range(x: usize, y: usize, width: usize, height: usize) -> Range<usize> {
        let start = Self::TEXTURE_WIDTH as usize * y as usize + x as usize;
        let end = start + (width * height);

        start..end
    }
}

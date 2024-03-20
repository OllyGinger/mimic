use std::{borrow::Borrow, cell::RefCell, ops::Range, rc::Rc};

use glium::{pixel_buffer::PixelBuffer, Display, Texture2d};
use imgui::{TextureId, Ui};
use imgui_glium_renderer::Renderer;

use crate::{
    cpu::cpu::CPU,
    gpu::{self, gpu::GPU},
    utils,
};

pub struct TileDebugView {
    pixel_buffer: PixelBuffer<(u8, u8, u8)>,
    texture_id: Option<TextureId>,
}

impl TileDebugView {
    const TILES_WIDE: usize = 16;
    const TILES_HIGH: usize = 24;

    const TEXTURE_WIDTH: usize = gpu::TILE_SIZE * Self::TILES_WIDE;
    const TEXTURE_HEIGHT: usize = gpu::TILE_SIZE * Self::TILES_HIGH;

    pub fn new(facade: Rc<RefCell<Display>>, renderer: Rc<RefCell<Renderer>>) -> TileDebugView {
        let facade_ref: &RefCell<Display> = facade.borrow();
        let facade_refref: &Display = &facade_ref.borrow();
        let mut view = TileDebugView {
            pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(
                facade_refref,
                Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize,
            ),
            texture_id: None,
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

    pub fn draw(self: &mut TileDebugView, renderer: &mut Renderer, ui: &mut Ui, gpu: &GPU) {
        ui.window("VRam Tile Viewer").build(|| {
            let mut buffer: [gpu::PixelColour;
                Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize] =
                [(255u8, 255u8, 255u8);
                    Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize];

            // Draw out the tile map
            self.draw_tile_map(gpu, &mut buffer);
            self.pixel_buffer.write(&buffer[..]);

            renderer
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
        self: &TileDebugView,
        gpu: &GPU,
        buffer: &mut [gpu::PixelColour;
                 Self::TEXTURE_WIDTH as usize * Self::TEXTURE_HEIGHT as usize],
    ) {
        // Draw tiles
        let mut tile_num = 0u16;
        for tile_y in 0..Self::TILES_HIGH {
            for tile_x in 0..Self::TILES_WIDE {
                let tile_buffer = gpu.get_bg_tile_as_pixels(tile_num);

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

                // Keep track of the tile we're reading
                tile_num += 1;
            }
        }
    }

    fn get_texture_range(x: usize, y: usize, width: usize, height: usize) -> Range<usize> {
        let start = Self::TEXTURE_WIDTH as usize * y as usize + x as usize;
        let end = start + (width * height);

        start..end
    }
}

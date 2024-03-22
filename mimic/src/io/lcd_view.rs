use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use glium::{pixel_buffer::PixelBuffer, Display, Texture2d};
use imgui::{TextureId, Ui};
use imgui_glium_renderer::Renderer;

use crate::{
    gpu::{self, gpu::GPU},
    utils,
};

pub struct LCDView {
    frame_buffer_pixel_buffer: PixelBuffer<(u8, u8, u8)>,
    frame_buffer_texture_id: Option<TextureId>,

    tile_map_base: u16,
    scyx_enabled: bool,
}

impl LCDView {
    // Full back buffer
    const BACK_BUFFER_TILES_WIDE: usize = 32;
    const BACK_BUFFER_TILES_HIGH: usize = 32;

    // What we see
    const FRAME_BUFFER_TILES_WIDE: usize = 20;
    const FRAME_BUFFER_TILES_HIGH: usize = 18;

    const BACK_BUFFER_TEXTURE_WIDTH: usize = gpu::TILE_SIZE * Self::BACK_BUFFER_TILES_WIDE;
    const BACK_BUFFER_TEXTURE_HEIGHT: usize = gpu::TILE_SIZE * Self::BACK_BUFFER_TILES_HIGH;
    const FRAME_BUFFER_TEXTURE_WIDTH: usize = gpu::TILE_SIZE * Self::FRAME_BUFFER_TILES_WIDE;
    const FRAME_BUFFER_TEXTURE_HEIGHT: usize = gpu::TILE_SIZE * Self::FRAME_BUFFER_TILES_HIGH;

    pub fn new(facade: Rc<RefCell<Display>>, renderer: Rc<RefCell<Renderer>>) -> LCDView {
        let facade_ref: &RefCell<Display> = facade.borrow();
        let facade_refref: &Display = &facade_ref.borrow();
        let mut view = LCDView {
            frame_buffer_pixel_buffer: glium::texture::pixel_buffer::PixelBuffer::new_empty(
                facade_refref,
                Self::FRAME_BUFFER_TEXTURE_WIDTH as usize
                    * Self::FRAME_BUFFER_TEXTURE_HEIGHT as usize,
            ),
            frame_buffer_texture_id: None,
            tile_map_base: 0x9800,
            scyx_enabled: true,
        };

        // Frame buffer
        let frame_buffer_tex2d = Texture2d::empty_with_format(
            facade_refref,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            Self::FRAME_BUFFER_TEXTURE_WIDTH as u32,
            Self::FRAME_BUFFER_TEXTURE_HEIGHT as u32,
        )
        .unwrap();
        view.frame_buffer_texture_id = Some(utils::register_texture(
            Rc::new(frame_buffer_tex2d),
            renderer,
        ));

        view
    }

    pub fn draw(&mut self, renderer: Rc<RefCell<Renderer>>, ui: &mut Ui, gpu: &GPU) {
        // Upload the frame buffer directly from the GPU. This is built as a series of scanlines
        // during each HSync phase.
        self.frame_buffer_pixel_buffer.write(gpu.get_back_buffer());

        let mut renderer_ref = renderer.borrow_mut();
        renderer_ref
            .textures()
            .get(self.frame_buffer_texture_id.unwrap())
            .unwrap()
            .texture
            .mipmap(0)
            .unwrap()
            .raw_upload_from_pixel_buffer(
                self.frame_buffer_pixel_buffer.as_slice(),
                0..Self::FRAME_BUFFER_TEXTURE_WIDTH as u32,
                0..Self::FRAME_BUFFER_TEXTURE_HEIGHT as u32,
                0..1,
            );

        ui.window("LCD").build(|| {
            let img = imgui::Image::new(
                self.frame_buffer_texture_id.unwrap(),
                [
                    Self::FRAME_BUFFER_TEXTURE_WIDTH as f32 * 2.0,
                    Self::FRAME_BUFFER_TEXTURE_HEIGHT as f32 * 2.0,
                ],
            );
            img.build(ui);
        });
    }
}

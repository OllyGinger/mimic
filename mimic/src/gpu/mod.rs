pub mod gpu;
pub mod palette;

pub type PixelColour = (u8, u8, u8);

pub(crate) const TILE_SIZE: usize = 8;
pub(crate) const BACK_BUFFER_TILES_WIDE: usize = 32;
pub(crate) const BACK_BUFFER_TILES_HIGH: usize = 32;

pub(crate) const FRAME_BUFFER_TILES_WIDE: usize = 20;
pub(crate) const FRAME_BUFFER_TILES_HIGH: usize = 18;
pub(crate) const FRAME_BUFFER_TEXTURE_WIDTH: usize = TILE_SIZE * FRAME_BUFFER_TILES_WIDE;
pub(crate) const FRAME_BUFFER_TEXTURE_HEIGHT: usize = TILE_SIZE * FRAME_BUFFER_TILES_HIGH;

pub(crate) const LCD_SCREEN_WIDTH: usize = 160;
pub(crate) const LCD_SCREEN_HEIGHT: usize = 144;

pub(crate) type BackBufferTexture =
    [PixelColour; FRAME_BUFFER_TEXTURE_HEIGHT * FRAME_BUFFER_TEXTURE_WIDTH];

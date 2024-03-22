pub mod gpu;
pub mod palette;

pub type PixelColour = (u8, u8, u8);

pub(crate) const TILE_SIZE: usize = 8;
pub(crate) const BACK_BUFFER_TILES_WIDE: usize = 32;
pub(crate) const BACK_BUFFER_TILES_HIGH: usize = 32;

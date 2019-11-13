mod ctrl;
mod data;
mod mask;
mod oam;
mod status;

pub use crate::ppu::regs::ctrl::PPUCtrl;
pub use crate::ppu::regs::mask::PPUMask;
pub use crate::ppu::regs::status::PPUStatus;
pub use crate::ppu::regs::oam::OAM;
pub use crate::ppu::regs::data::PPUData;

pub use crate::ppu::regs::data::{
    BACKGROUND_PALETTE_ADDRESSES,
    SPRITE_PALETTE_ADDRESSES,
    PATTERN_TABLE_ADDRESSES,
};

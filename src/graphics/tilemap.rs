#![allow(dead_code)]

/// Tile 8x8@4bpp
/// 32 bytes = 8 u32
#[repr(C)]
pub struct Tile4 {
    data: [u32; 8]
}

/// Tile 8x8@8bpp
/// 64 bytes = 16 u32
#[repr(C)]
pub struct Tile8 {
    data: [u32; 16]
}

/// 4bpp tile block: 32x16 tiles
pub type CharBlock4  = [Tile4; 512];
/// 8bpp tile block: 16x16 tiles
pub type CharBlock8 = [Tile8; 256];

mod flags;
pub use flags::*;

use super::ControlCode;

#[repr(C, packed)]
pub struct TileBinningControlList {
    bin_mode_config_code: u8,
    bin_mode_config: TileBinningModeConfig,
    start_tile_binning: u8,
    clip_window_code: u8,
    clip_window: ClipWindowConfig,
    bin_config_code: u8,
    bin_config: TileBinningConfig,
    viewport_offset_code: u8,
    viewport_offset: ViewportOffset,
    nv_shader_state_code: u8,
    nv_shader_state_addr: u32,
    indexed_list_code: u8,
    indexed_list: IndexedPrimitiveList,
    flush: u8,
}

impl TileBinningControlList {
    pub fn new(
        bin_mode_config: TileBinningModeConfig,
        clip_window: ClipWindowConfig,
        bin_config: TileBinningConfig,
        viewport_offset: ViewportOffset,
        nv_shader_state_addr: u32,
        indexed_list: IndexedPrimitiveList,
    ) -> Self {
        Self {
            bin_mode_config_code: ControlCode::TileBinningModeConfiguration,
            bin_mode_config,
            start_tile_binning: ControlCode::StartTileBinning,
            clip_window_code: ControlCode::ClipWindow,
            clip_window,
            bin_config_code: ControlCode::ConfigurationBits,
            bin_config,
            viewport_offset_code: ControlCode::ViewportOffset,
            viewport_offset,
            nv_shader_state_code: ControlCode::NVShaderState,
            nv_shader_state_addr,
            indexed_list_code: ControlCode::IndexedPrimitiveList,
            indexed_list,
            flush: ControlCode::Flush,
        }
    }
}

#[repr(C, packed)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ViewportOffset {
    pub x: u16,
    pub y: u16,
}

#[repr(C, packed)]
pub struct TileBinningModeConfig {
    /// Tile Allocation Memory Address (Bit 0..31)
    pub address: u32,
    /// Tile Allocation Memory Size (Bytes) (Bit 32..63)
    pub size: u32,
    /// Tile State Data Array Base Address (16-Byte Aligned, Size Of 48 Bytes * Num Tiles) (Bit 64..95)
    pub base_address: u32,
    /// Width (In Tiles) (Bit 96..103)
    pub width: u8,
    /// Height (In Tiles) (Bit 104..111)
    pub height: u8,
    /// Data Record (Bit 112..119)
    pub flags: TileBinningModeFlags,
}

#[repr(C, packed)]
pub struct TileBinningConfig {
    pub data8: TileBinningFlags8,
    pub data16: TileBinningFlags16,
}

#[repr(C, packed)]
pub struct ClipWindowConfig {
    /// Clip Window Left Pixel Coordinate (Bit 0..15)
    pub left: u16,
    /// Clip Window Bottom Pixel Coordinate (Bit 16..31)
    pub bottom: u16,
    /// Clip Window Width In Pixels (Bit 32..47)
    pub width: u16,
    /// Clip Window Height In Pixels (Bit 48..63)
    pub height: u16,
}

#[repr(C, packed)]
pub struct IndexedPrimitiveList {
    /// Configuration flags (Bit 0..7)
    pub flags: IndexedPrimitiveListFlags,
    /// Length (Number Of Indices) (Bit 8..39)
    pub length: u32,
    /// Address Of Indices List (Bit 40..71)
    pub address: u32,
    /// Maximum Index (Bit 72..103)
    pub max_index: u32,
}

#[cfg(feature = "alloc")]
pub mod dynamic;

pub mod fixed;
mod flags;
pub use flags::*;

use crate::control_list::control_code::ControlCode;

#[repr(C, packed)]
pub struct StoreTileBufferGeneral {
    flags16: StoreTileBufferGeneralFlags16,
    flags32: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StoreMultiSample {
    tile_coords_code: u8,
    tile_coords: TileCoordinates,
    bsl: BranchToSubList,
    str_ms: u8,
}

impl StoreMultiSample {
    pub(crate) const EMPTY: StoreMultiSample = StoreMultiSample {
        tile_coords_code: 0,
        tile_coords: TileCoordinates { column: 0, row: 0 },
        bsl: BranchToSubList {
            code: 0,
            address: 0,
        },
        str_ms: 0,
    };

    pub const fn new(
        tile_coordinates: TileCoordinates,
        bin_address: u32,
        offset: u32,
    ) -> Self {
        Self {
            tile_coords_code: ControlCode::TileCoordinates,
            tile_coords: tile_coordinates,
            bsl: BranchToSubList::new_with_offset(bin_address, offset),
            str_ms: ControlCode::StoreMultiSample,
        }
    }

    pub const fn end(
        tile_coordinates: TileCoordinates,
        bin_address: u32,
        offset: u32,
    ) -> Self {
        Self {
            tile_coords_code: ControlCode::TileCoordinates,
            tile_coords: tile_coordinates,
            bsl: BranchToSubList::new_with_offset(bin_address, offset),
            str_ms: ControlCode::StoreMultiSampleEnd,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct BranchToSubList {
    code: u8,
    address: u32,
}

impl BranchToSubList {
    pub const fn new(address: u32) -> Self {
        Self {
            code: ControlCode::BranchToSubList,
            address,
        }
    }

    pub const fn new_with_offset(base_address: u32, offset: u32) -> Self {
        Self::new(base_address + offset)
    }
}

impl StoreTileBufferGeneral {
    pub const fn new(
        flags16: StoreTileBufferGeneralFlags16,
        flags32: StoreTileBufferGeneralFlags32,
        address: u32,
    ) -> Self {
        // TODO: this feels weird to add the bits to the address...
        // check the architecture reference and see what's happening
        Self {
            flags16,
            flags32: flags32.bits() + address,
        }
    }
}

// impl RenderControlList {
//     pub const fn new() {
//         // code: ControlCode::ClearColors,
//         todo!()
//     }
// }

#[repr(C, packed)]
pub struct ClearColors {
    pub color: Color,
    pub clear_zs_vg_mask: u32,
    pub clear_stencil: u8,
}

impl ClearColors {
    pub const fn new(
        color: Color,
        clear_zs: u32,
        clear_vg_mask: u8,
        clear_stencil: u8,
    ) -> Self {
        let clear_zs_vg_mask =
            (clear_zs & 0x00FF_FFFF) | ((clear_vg_mask as u32) << 24);
        Self {
            color,
            clear_zs_vg_mask,
            clear_stencil,
        }
    }
}

#[repr(C, packed)]
pub struct Color {
    color: u64,
}

impl Color {
    pub const fn from_raw(color: u64) -> Self {
        Self { color }
    }

    pub const fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_raw(
            0u64 | r as u64
                | (g as u64) << 8
                | (b as u64) << 16
                | (a as u64) << 24
                | (r as u64) << 32
                | (g as u64) << 40
                | (b as u64) << 48
                | (a as u64) << 56,
        )
    }

    pub const fn rgba16(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self::from_raw(
            0u64 | r as u64
                | (g as u64) << 16
                | (b as u64) << 32
                | (a as u64) << 48,
        )
    }
}

#[repr(C, packed)]
pub struct TileRenderingModeConfig {
    pub address: u32,
    pub width: u16,
    pub height: u16,
    pub flags: TileRenderingModeFlags,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TileCoordinates {
    pub column: u8,
    pub row: u8,
}

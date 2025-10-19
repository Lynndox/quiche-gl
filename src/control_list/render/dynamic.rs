use super::*;
use crate::{Align4, mem::*};

use alloc::vec::Vec;
use core::mem::size_of;
use core::slice;

macro_rules! raw_bytes {
    ($(&)? $value:ident) => {
        ::core::slice::from_raw_parts(
            &raw const $value as *const u8,
            size_of_val(&$value),
        )
    };
    ($($tokens:tt)*) => {{
        let __value = $($tokens)*;
        ::core::slice::from_raw_parts(&raw const __value as *const u8, size_of_val(&__value))
    }};
}

#[repr(transparent)]
pub struct RenderControlList {
    data: Vec<u8>,
}

impl RenderControlList {
    pub fn new(
        clear_colors: ClearColors,
        tile_rendering_config: TileRenderingModeConfig,
        binning_address: ArmAddress<Physical>,
        width_px: u32,
        height_px: u32,
    ) -> Self {
        let tiles_x = width_px / 32;
        let tiles_y = height_px / 32;

        let binning_address = binning_address.addr as u32;

        let stb_general = StoreTileBufferGeneral::new(
            StoreTileBufferGeneralFlags16::empty(),
            StoreTileBufferGeneralFlags32::empty(),
            0,
        );

        let mut data: Vec<u8> = Vec::new();
        let tile_coordinates = TileCoordinates { column: 0, row: 0 };

        unsafe {
            data.extend(
                &[
                    &[ControlCode::ClearColors],
                    raw_bytes!(&clear_colors),
                    &[ControlCode::TileRenderingModeConfiguration],
                    raw_bytes!(&tile_rendering_config),
                    &[ControlCode::TileCoordinates],
                    raw_bytes!(&tile_coordinates),
                    &[ControlCode::StoreTileBufferGeneral],
                    raw_bytes!(&stb_general),
                ]
                .concat(),
            );

            data.extend((0..=tiles_y).flat_map(move |y| {
                (0..=tiles_x).flat_map(move |x| {
                    let tile_coords = TileCoordinates {
                        column: x as _,
                        row: y as _,
                    };
                    let offset = ((y * (tiles_x + x)) * 32);

                    raw_bytes!(if x < tiles_x && y < tiles_y {
                        StoreMultiSample::new(
                            tile_coords,
                            binning_address,
                            offset,
                        )
                    } else {
                        StoreMultiSample::end(
                            tile_coords,
                            binning_address,
                            offset,
                        )
                    })
                })
            }));
        }

        Self { data }
    }
}

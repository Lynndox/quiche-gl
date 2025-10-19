use super::*;

#[repr(C, packed)]
pub struct RenderControlList<const W: usize, const H: usize> {
    clear_colors_code: u8,
    clear_colors: ClearColors,
    tile_rendering_config_code: u8,
    tile_rendering_config: TileRenderingModeConfig,
    tile_coordinates_code: u8,
    tile_coordinates: TileCoordinates,
    str_tile_buf_general_code: u8,
    str_tile_buf_general: StoreTileBufferGeneral,
    tile_buf: [[StoreMultiSample; W]; H],
}

impl<const TILES_X: usize, const TILES_Y: usize>
    RenderControlList<TILES_X, TILES_Y>
{
    pub const fn new(
        clear_colors: ClearColors,
        tile_rendering_config: TileRenderingModeConfig,
    ) -> Self {
        let tile_buf = {
            let binning_address = tile_rendering_config.address;
            let mut buf = [[StoreMultiSample::EMPTY; TILES_X]; TILES_Y];

            let mut y = 0;
            while y < TILES_Y {
                let mut x = 0;
                while x < TILES_X {
                    let tile_coords = TileCoordinates {
                        column: x as u8,
                        row: y as u8,
                    };

                    let offset = ((y * (TILES_X + x)) * 32) as u32;

                    if y == TILES_Y - 1 {
                        if x == TILES_X - 1 {
                            buf[y][x] = StoreMultiSample::end(
                                tile_coords,
                                binning_address,
                                offset,
                            );
                        }
                    } else {
                        buf[y][x] = StoreMultiSample::new(
                            tile_coords,
                            binning_address,
                            offset,
                        );
                    }

                    x += 1;
                }

                y += 1;
            }

            buf
        };

        // let str_tiles: [[StoreMultiSample; TILES_X]; TILES_Y] =
        //     array::from_fn!(|y| array::from_fn!(|x| {
        //         let tile_coords = TileCoordinates {
        //             column: x as u8,
        //             row: y as _,
        //         };
        //
        //         let offset = ((y * (TILES_X + x)) * 32) as u32;
        //
        //         if x < TILES_X && y < TILES_Y {
        //             StoreMultiSample::new(tile_coords, binning_address,
        // offset)         } else {
        //             StoreMultiSample::end(tile_coords, binning_address,
        // offset)         }
        //     }));

        Self {
            clear_colors_code: ControlCode::ClearColors,
            clear_colors,
            tile_rendering_config_code:
                ControlCode::TileRenderingModeConfiguration,
            tile_rendering_config,
            tile_coordinates_code: ControlCode::TileCoordinates,
            tile_coordinates: TileCoordinates { column: 0, row: 0 },
            str_tile_buf_general_code: ControlCode::StoreTileBufferGeneral,
            // ???
            str_tile_buf_general: StoreTileBufferGeneral::new(
                StoreTileBufferGeneralFlags16::empty(),
                StoreTileBufferGeneralFlags32::empty(),
                0,
            ),
            tile_buf,
        }
    }
}

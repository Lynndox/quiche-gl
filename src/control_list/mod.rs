mod control_code;
use control_code::ControlCode;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TileBinningModeFlags(u8);

bitflags::bitflags! {
    impl TileBinningModeFlags: u8 {
        /// Multisample Mode (4X)
        const MultisampleMode4X = 0x01;
        /// Tile Buffer 64-Bit Color Depth
        const BufferColorDepth64 = 0x02;
        /// Auto-Initialise Tile State Data Array
        const AutoInitialiseTileStateDataArray = 0x04;
        /// Tile Allocation Initial Block Size = 32 Bytes
        const InitialBlockSize32 = 0x00;
        /// Tile Allocation Initial Block Size = 64 Bytes
        const InitialBlockSize64 = 0x08;
        /// Tile Allocation Initial Block Size = 128 Bytes
        const InitialBlockSize128 = 0x10;
        /// Tile Allocation Initial Block Size = 256 Bytes
        const InitialBlockSize256 = 0x18;
        /// Tile Allocation Block Size = 32 Bytes
        const BlockSize32 = 0x00;
        /// Tile Allocation Block Size = 64 Bytes
        const BlockSize64 = 0x20;
        /// Tile Allocation Block Size = 128 Bytes
        const BlockSize128 = 0x40;
        /// Tile Allocation Block Size = 256 Bytes
        const BlockSize256 = 0x60;
        /// Double-Buffer In Non-MS Mode
        const DoubleBufferInNonMSMode = 0x80;
    }
}

#[repr(C, packed)]
pub struct TileBinningModeConfig {
    control_code: u8,
    address: u32,
    size: u32,
    base_address: u32,
    tile_width: u8,
    tile_height: u8,
    config: TileBinningModeFlags,
}

impl TileBinningModeConfig {
    pub fn new(
        address: u32,
        size: u32,
        base_address: u32,
        tile_width: u8,
        tile_height: u8,
        config: TileBinningModeFlags,
    ) -> Self {
        Self {
            control_code: 0x70,
            address,
            size,
            base_address,
            tile_width,
            tile_height,
            config,
        }
    }
}

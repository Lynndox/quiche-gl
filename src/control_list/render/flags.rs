use bitflags::bitflags;

bitflags! {
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TileRenderingModeFlags: u16 {
        /// Multisample Mode (4X)
        const MultisampleMode4X = 0x0001;
        /// Tile Buffer 64-Bit Color Depth (HDR Mode)
        const BufferColorDepth64 = 0x0002;
        /// Non-HDR Frame Buffer Color Format = BGR565 Dithered
        const FrameBufferColorFormatBGR565Dithered = 0x0000;
        /// Non-HDR Frame Buffer Color Format = RGBA8888
        const FrameBufferColorFormatRGBA8888 = 0x0004;
        /// Non-HDR Frame Buffer Color Format = BGR565 No Dither
        const FrameBufferColorFormatBGR565NoDither = 0x0008;
        /// Decimate Mode = 1X
        const DecimateMode1X = 0x0000;
        /// Decimate Mode = 4X
        const DecimateMode4X = 0x0010;
        /// Decimate Mode = 16X
        const DecimateMode16X = 0x0020;
        /// Memory Format = Linear
        const MemoryFormatLinear = 0x0000;
        /// Memory Format = T-Format
        const MemoryFormatTFormat = 0x0040;
        /// Memory Format = LT-Format
        const MemoryFormatLTFormat = 0x0080;
        /// Enable VG Mask Buffer
        const EnableVGMaskBuffer = 0x0100;
        /// Select Coverage Mode
        const SelectCoverageMode = 0x0200;
        /// Early-Z Update Direction = LT/LE
        const EarlyZUpdateDirectionLTLE = 0x0000;
        /// Early-Z Update Direction = GT/GE
        const EarlyZUpdateDirectionGTGE = 0x0400;
        /// Early-Z/Early-Cov Disable
        const EarlyZEarlyCovDisable = 0x0800;
        /// Double-Buffer In Non-MS Mode
        const DoubleBufferInNonMSMode = 0x1000;
    }
}

bitflags! {
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct StoreTileBufferGeneralFlags16: u16 {
        /// Buffer To Store = None
        const Store_None = 0x0000;
        /// Buffer To Store = Color
        const Store_Color = 0x0001;
        /// Buffer To Store = Z/Stencil
        const Store_Z_Stencil = 0x0002;
        /// Buffer To Store = Z-Only
        const Store_Z_Only = 0x0003;
        /// Buffer To Store = VG-Mask
        const Store_VG_Mask = 0x0004;
        /// Buffer To Store = Full Dump
        const Store_Full_Dump = 0x0005;
        /// Format = Raster Format
        const Format_Raster = 0x0000;
        /// Format = T-Format
        const Format_T = 0x0010;
        /// Format = LT-Format
        const Format_LT = 0x0020;
        /// Mode = Sample
        const Mode_Sample = 0x0000;
        /// Mode = Decimate 4X
        const Mode_Decimate_4X = 0x0040;
        /// Mode = Decimate 16X
        const Mode_Decimate_16X = 0x0080;
        /// Pixel Color Format = RGBA8888
        const Color_Format_RGBA8888 = 0x0000;
        /// Pixel Color Format = BGR565 Dithered
        const Color_Format_BGR565_Dithered = 0x0100;
        /// Pixel Color Format = BGR565 No Dither
        const Color_Format_BGR565_No_Dither = 0x0200;
        /// Disable Double-Buffer Swap In Double Buffer Mode
        const Disable_Double_Buffer_Swap = 0x1000;
        /// Disable Color Buffer Clear On Store/Dump
        const Disable_Color_Buffer_Clear = 0x2000;
        /// Disable Z/Stencil Buffer Clear On Store/Dump
        const Disable_Z_Stencil_Buffer_Clear = 0x4000;
        /// Disable VG-Mask Buffer Clear On Store/Dump
        const Disable_VG_Mask_Buffer_Clear = 0x8000;
    }
}

bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct StoreTileBufferGeneralFlags32: u32 {
        /// Disable Color Buffer Dump
        const Disable_Color_Buffer_Dump = 0x00000001;
        /// Disable Z/Stencil Buffer Dump
        const Disable_Z_Stencil_Buffer_Dump = 0x00000002;
        /// Disable VG-Mask Buffer Dump
        const Disable_VG_Mask_Buffer_Dump = 0x00000004;
        /// Last Tile Of Frame
        const Last_Tile_Of_Frame = 0x00000008;
        /// Memory Base Address Of Frame/Tile Dump Buffer (In multiples of 16 bytes)
        const Memory_Base_Address = 0xFFFFFFF0;
    }
}

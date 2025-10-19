bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TileBinningFlags8: u8 {
        /// Enable Forward Facing Primitive
        const EnableForwardFacingPrimitive = 0x01;

        /// Enable Reverse Facing Primitive
        const EnableReverseFacingPrimitive = 0x02;

        /// Clockwise Primitives
        const ClockwisePrimitives          = 0x04;

        /// Enable Depth Offset
        const EnableDepthOffset            = 0x08;

        /// Antialiased Points & Lines (Not Actually Supported)
        const AntialiasedPointsLines       = 0x10;

        /// Coverage Read Type = 4*8-Bit Level
        const CoverageReadTypeLevel48      = 0x00;

        /// Coverage Read Type = 16-Bit Mask
        const CoverageReadTypeMask16       = 0x20;

        /// Rasteriser Oversample Mode = None
        const RasteriserOversampleModeNone = 0x00;

        /// Rasteriser Oversample Mode = 4X
        const RasteriserOversampleMode4X   = 0x40;

        /// Rasteriser Oversample Mode = 16X
        const RasteriserOversampleMode16X  = 0x80;
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TileBinningFlags16: u16 {
        /// Coverage Pipe Select
        const CoveragePipeSelect          = 0x0001;

        /// Coverage Update Mode = Non Zero
        const CoverageUpdateModeNonZero   = 0x0000;

        /// Coverage Update Mode = Odd
        const CoverageUpdateModeOdd       = 0x0002;

        /// Coverage Update Mode = OR
        const CoverageUpdateModeOR        = 0x0004;

        /// Coverage Update Mode = Zero
        const CoverageUpdateModeZero      = 0x0006;

        /// Coverage Read Mode = Clear On Read
        const CoverageReadModeClearOnRead = 0x0000;

        /// Coverage Read Mode = Leave On Read
        const CoverageReadModeLeaveOnRead = 0x0008;

        /// Depth-Test Function = Never
        const DepthTestFunctionNever      = 0x0000;

        /// Depth-Test Function = Less Than (LT)
        const DepthTestFunctionLT         = 0x0010;

        /// Depth-Test Function = Equal (EQ)
        const DepthTestFunctionEQ         = 0x0020;

        /// Depth-Test Function = Less Equal (LE)
        const DepthTestFunctionLE         = 0x0030;

        /// Depth-Test Function = Greater Than (GT)
        const DepthTestFunctionGT         = 0x0040;

        /// Depth-Test Function = Not Equal (NE)
        const DepthTestFunctionNE         = 0x0050;

        /// Depth-Test Function = Greater Equal (GE)
        const DepthTestFunctionGE         = 0x0060;

        /// Depth-Test Function = Always
        const DepthTestFunctionAlways     = 0x0070;

        /// Z Updates Enable
        const ZUpdatesEnable              = 0x0080;

        /// Early Z Enable
        const EarlyZEnable                = 0x0100;

        /// Early Z Updates Enable
        const EarlyZUpdatesEnable         = 0x0200;
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct TileBinningModeFlags: u8 {
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

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub struct IndexedPrimitiveListFlags: u8 {
        /// Primitive Mode = Points
        const Points         = 0x00;

        /// Primitive Mode = Lines
        const Lines          = 0x01;

        /// Primitive Mode = Line Loop
        const LineLoop      = 0x02;

        /// Primitive Mode = Line Strip
        const LineStrip     = 0x03;

        /// Primitive Mode = Triangles
        const Triangles      = 0x04;

        /// Primitive Mode = Triangle Strip
        const TriangleStrip = 0x05;

        /// Primitive Mode = Triangle Fan
        const TriangleFan   = 0x06;

        /// Index Type = 8-Bit
        const IndexType8  = 0x00;

        /// Index Type = 16-Bit
        const IndexType16 = 0x10;
    }
}

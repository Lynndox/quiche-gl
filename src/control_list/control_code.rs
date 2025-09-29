pub struct ControlCode;

#[allow(non_upper_case_globals)]
#[rustfmt::skip]
impl ControlCode {
    /// Halt
    pub const Halt: u8 = 0x00;

    /// NoOp
    pub const NoOp: u8 = 0x01;

    /// Flush (Add Return-From-Sub-List To Tile Lists & Then Flush Tile Lists To Memory) (B)
    pub const Flush: u8 = 0x04;

    /// FlushAllState (Same As Flush, But Preceded By The Forced Writing Of The Current State To The Tile Lists) (B)
    pub const FlushAllState: u8 = 0x05;

    /// StartTileBinning (Advances State Counter So That Initial State Items Actually Go Into Tile Lists) (B)
    pub const StartTileBinning: u8 = 0x06;

    /// IncrementSemaphore (After Tile Lists Are Flushed Or Last Tile Written)
    pub const IncrementSemaphore: u8 = 0x07;

    /// WaitOnSemaphore (Wait For Frame To Complete In Other Thread)
    pub const WaitOnSemaphore: u8 = 0x08;

    /// Branch (32-Bit Absolute Branch Address)
    pub const Branch: u8 = 0x10;
    // dw address // Control ID Data Record Word: 32-Bit Absolute Branch Address (Bit 0..31)

    /// BranchToSubList (32-Bit Absolute Branch Address, Maximum Of 2 Levels Of Nesting)
    pub const BranchToSubList: u8 = 0x11;
    // dw address // Control ID Data Record Word: 32-Bit Absolute Branch Address (Bit 0..31)

    /// ReturnFromSubList (Ignored If Nothing On The Return Stack)
    pub const ReturnFromSubList: u8 = 0x12;

    /// StoreMultiSample (Resolved Tile Color Buffer) (R)
    pub const StoreMultiSample: u8 = 0x18;

    /// StoreMultiSampleEnd (Resolved Tile Color Buffer & Signal End Of Frame) (R)
    pub const StoreMultiSampleEnd: u8 = 0x19;

    /// StoreFullResolution (32-Bit Data Record) (R)
    pub const StoreFullResolution: u8 = 0x1A;
    // dw address + data // Control ID Data Record Word: Memory Address Of Tile (In Multiples Of 16 Bytes) (Bit 4..31), Data Record (Bit 0..3)

    /// ReLoadFullResolution (32-Bit Data Record) (R)
    pub const ReLoadFullResolution: u8 = 0x1B;
    // dw address + data // Control ID Data Record Word: Memory Address Of Tile (In Multiples Of 16 Bytes) (Bit 4..31), Data Record (Bit 0..3)

    /// StoreTileBufferGeneral (R)
    pub const StoreTileBufferGeneral: u8 = 0x1C;
    // dh data16 // Control ID Data Record Short: (Bit 0..15)
    // dw address + data32 // Control ID Data Record Word: Memory Base Address Of Frame/Tile Dump Buffer (In Multiples Of 16 Bytes) (Bit 20..47), Data Record (Bit 16..19)

    /// LoadTileBufferGeneral (R)
    pub const LoadTileBufferGeneral: u8 = 0x1D;
    // dh data16 // Control ID Data Record Short: (Bit 0..15)
    // dw address + data32 // Control ID Data Record Word: Memory Base Address Of Frame/Tile Dump Buffer (In Multiples Of 16 Bytes) (Bit 20..47), Data Record (Bit 16..19)

    /// IndexedPrimitiveList (OpenGL)
    pub const IndexedPrimitiveList: u8 = 0x20;
    // db data     // Control ID Data Record Byte: (Bit 0..7)
    // dw length   // Control ID Data Record Word: Length (Number Of Indices) (Bit 8..39)
    // dw address  // Control ID Data Record Word: Address Of Indices List (Bit 40..71)
    // dw maxindex // Control ID Data Record Word: Maximum Index (Bit 72..103)

    /// VertexArrayPrimitives (OpenGL)
    pub const VertexArrayPrimitives: u8 = 0x21;
    // db data   // Control ID Data Record Byte: (Bit 0..7)
    // dw length // Control ID Data Record Word: Length (Number Of Vertices) (Bit 8..39)
    // dw index  // Control ID Data Record Word: Index Of First Vertex (Bit 40..71)

    /// VGCoordinateArrayPrimitives (Only For Use In VG Shader Mode)
    pub const VGCoordinateArrayPrimitives: u8 = 0x29;
    // db data    // Control ID Data Record Byte: (Bit 0..7)
    // dw length  // Control ID Data Record Word: Length (Number Of Primitives) (Bit 8..39)
    // dw address // Control ID Data Record Word: Address Of Coordinate Array (Bit 40..71)

    /// VGInlinePrimitives (Only For Use In VG Shader Mode)
    pub const VGInlinePrimitives: u8 = 0x2A;
    // db data // Control ID Data Record Byte: (Bit 0..7)
      // Control ID Data Record Words: Escape Terminated Uncompressed 32-Bit X,Y Coordinate List (Bit 8..X)

    /// CompressedPrimitiveList (R)
    pub const CompressedPrimitiveList: u8 = 0x30;
      // Control ID Data Record Bytes: Escape Terminated List (Bit 0..X)

    /// ClippedPrimitiveWithCompressedPrimitiveList (R)
    pub const ClippedPrimitiveWithCompressedPrimitiveList: u8 = 0x31;
    // dw address + data // Control ID Data Record Word: Address Of Single Clipped Primitive Data (Multiple Of 8 Bytes) (Bit 3..31), 1 Flag Per Vertex Of Next Primitive (Bit 0..2)
      // Control ID Data Record Bytes: Escape Terminated List (Bit 32..X)

    /// PrimitiveListFormat (R)
    pub const PrimitiveListFormat: u8 = 0x38;
    // db data // Control ID Data Record Byte: (Bit 0..7)

    /// GLShaderState
    pub const GLShaderState: u8 = 0x40;
    // dw address + data // Control ID Data Record Word: Memory Address Of Shader Record (In Multiples Of 16 Bytes) (Bit 4..31), Data Record (Bit 0..3)

    /// NVShaderState (No Vertex Shading)
    pub const NVShaderState: u8 = 0x41;
    // dw address // Control ID Data Record Word: Memory Address Of Shader Record (16-Byte Aligned) (Bit 0..31)

    /// VGShaderState
    pub const VGShaderState: u8 = 0x42;
    // dw address // Control ID Data Record Word: Memory Address Of Shader Record (16-Byte Aligned) (Bit 0..31)

    /// VGInlineShaderRecord
    pub const VGInlineShaderRecord: u8 = 0x43;
    // dw addressc + data // Control ID Data Record Word: Fragment Shader Code Address (8-Byte Multiple) (Bit 3..31), Data Record (Bit 0..2)
    // dw addressu // Control ID Data Record Word: Fragment Shader Uniforms Address (4-Byte Aligned) (Bit 32..63)

    /// ConfigurationBits
    pub const ConfigurationBits: u8 = 0x60;
    // db data8  // Control ID Data Record Byte: (Bit 0..7)
    // dh data16 // Control ID Data Record Short: (Bit 8..23)

    /// FlatShadeFlags
    pub const FlatShadeFlags: u8 = 0x61;
    // dw flags // Control ID Data Record Word: Flat-Shading Flags (32 x 1-Bit) (Bit 0..31)

    /// PointSize
    pub const PointSize: u8 = 0x62;
    // dw size // Control ID Data Record Word: Point Size (FLOAT32) (Bit 0..31)

    /// LineWidth
    pub const LineWidth: u8 = 0x63;
    // dw width // Control ID Data Record Word: Line Width (FLOAT32) (Bit 0..31)

    /// RHTXBoundary
    pub const RHTXBoundary: u8 = 0x64;
    // dh boundary // Control ID Data Record Short: RHT Primitive X Boundary (SINT16) (Bit 0..15)

    /// DepthOffset
    pub const DepthOffset: u8 = 0x65;
    // dh factor // Control ID Data Record Short: Depth Offset Factor (FLOAT1-8-7) (Bit 0..15)
    // dh units  // Control ID Data Record Short: Depth Offset Units (FLOAT1-8-7) (Bit 16..31)

    /// ClipWindow
    pub const ClipWindow: u8 = 0x66;
    // dh left   // Control ID Data Record Short: Clip Window Left Pixel Coordinate (UINT16) (Bit 0..15)
    // dh bottom // Control ID Data Record Short: Clip Window Bottom Pixel Coordinate (UINT16) (Bit 16..31)
    // dh width  // Control ID Data Record Short: Clip Window Width In Pixels (UINT16) (Bit 32..47)
    // dh height // Control ID Data Record Short: Clip Window Height In Pixels (UINT16) (Bit 48..63)

    /// ViewportOffset
    pub const ViewportOffset: u8 = 0x67;
    // dh x // Control ID Data Record Short: Viewport Centre X-Coordinate (SINT16) (Bit 0..15)
    // dh y // Control ID Data Record Short: Viewport Centre Y-Coordinate (SINT16) (Bit 16..31)

    /// ZMinMaxClippingPlanes
    pub const ZMinMaxClippingPlanes: u8 = 0x68;
    // dw min // Control ID Data Record Word: Minimum ZW (FLOAT32) (Bit 0..31)
    // dw max // Control ID Data Record Word: Maximum ZW (FLOAT32) (Bit 32..63)

    /// ClipperXYScaling (B)
    pub const ClipperXYScaling: u8 = 0x69;
    // dw width  // Control ID Data Record Word: Viewport Half-Width In 1/16th Of Pixel (FLOAT32) (Bit 0..31)
    // dw height // Control ID Data Record Word: Viewport Half-Height In 1/16th Of pixel (FLOAT32) (Bit 32..63)

    /// ClipperZScaleOffset (B)
    pub const ClipperZScaleOffset: u8 = 0x6A;
    // dw scale  // Control ID Data Record Word: Viewport Z Scale (ZC To ZS) (FLOAT32) (Bit 0..31)
    // dw offset // Control ID Data Record Word: Viewport Z Offset (ZC To ZS) (FLOAT32) (Bit 32..63)

    /// TileBinningModeConfiguration (B)
    pub const TileBinningModeConfiguration: u8 = 0x70;
    // dw address     // Control ID Data Record Word: Tile Allocation Memory Address (Bit 0..31)
    // dw size        // Control ID Data Record Word: Tile Allocation Memory Size (Bytes) (Bit 32..63)
    // dw baseaddress // Control ID Data Record Word: Tile State Data Array Base Address (16-Byte Aligned, Size Of 48 Bytes * Num Tiles) (Bit 64..95)
    // db width       // Control ID Data Record Byte: Width (In Tiles) (Bit 96..103)
    // db height      // Control ID Data Record Byte: Height (In Tiles) (Bit 104..111)
    // db data        // Control ID Data Record Byte: Data Record (Bit 112..119)

    /// TileRenderingModeConfiguration (R)
    pub const TileRenderingModeConfiguration: u8 = 0x71;
    // dw address // Control ID Data Record Word: Memory Address (Bit 0..31)
    // dh width   // Control ID Data Record Short: Width (Pixels) (UINT16) (Bit 32..47)
    // dh height  // Control ID Data Record Short: Height (Pixels) (UINT16) (Bit 48..63)
    // dh data    // Control ID Data Record Short: Data Record (Bit 64..79)

    /// ClearColors (R)
    pub const ClearColors: u8 = 0x72;
    // dd clearcolor                         // Control ID Data Record Double: Clear Color (2X RGBA8888 Or RGBA16161616) (Bit 0..63)
    // dw (clearvgmask * $1000000) + clearzs // Control ID Data Record Word: Clear VG Mask (UINT8) (Bit 80..95), Clear ZS (UINT24) (Bit 64..79)
    // db clearstencil                       // Control ID Data Record Byte: Clear Stencil (UINT8) (Bit 96..103)

    /// TileCoordinates (R)
    pub const TileCoordinates: u8 = 0x73;
    // db column // Control ID Data Record Byte: Tile Column Number (INT8) (Bit 0..7)
    // db row    // Control ID Data Record Byte: Tile Row Number (INT8) (Bit 8..15)
}

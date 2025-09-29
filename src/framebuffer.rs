pub struct FrameBuffer {
    pub buffer: &'static mut [u32],
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub double_buffered: bool,
}

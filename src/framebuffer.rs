pub struct FrameBuffer {
    pub buffer: &'static mut [u32],
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
    pub double_buffered: bool,
}

impl FrameBuffer {
    /// Creates an uninitialized FrameBuffer.
    ///
    /// # Safety
    ///
    /// The framebuffer **must** be initialized before use. Failure to do so
    /// will lead to undefined behavior.
    pub(crate) const unsafe fn empty() -> Self {
        unsafe {
            Self {
                buffer: &mut [],
                width: 0,
                height: 0,
                bit_depth: 0,
                double_buffered: true,
            }
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            width: Default::default(),
            height: Default::default(),
            bit_depth: Default::default(),
            double_buffered: true,
        }
    }
}

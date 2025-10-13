use core::ptr::NonNull;

#[derive(Debug)]
pub struct FrameBuffer {
    pub(crate) ptr: NonNull<u32>,
    pub(crate) size: usize,
    pub(crate) screen_size: usize,
    pub(crate) curr_screen: usize,
    pub(crate) num_buffers: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) bit_depth: u32,
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
                ptr: NonNull::dangling(),
                size: 0,
                screen_size: 0,
                curr_screen: 0,
                num_buffers: 0,
                width: 0,
                height: 0,
                bit_depth: 0,
            }
        }
    }

    /// Returns a reference to the raw underlying buffer for the current screen.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn curr_screen_buffer(&mut self) -> &[u32] {
        unsafe {
            core::slice::from_raw_parts(
                self.ptr
                    .as_ptr()
                    .byte_add(self.screen_size * self.curr_screen),
                self.screen_size,
            )
        }
    }

    /// Returns a mutable reference to the raw underlying buffer for the current
    /// screen.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn curr_screen_buffer_mut(&mut self) -> &mut [u32] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.ptr
                    .as_ptr()
                    .byte_add(self.screen_size * self.curr_screen),
                self.screen_size,
            )
        }
    }

    /// Returns a reference to the raw underlying buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn full_buffer(&mut self) -> &[u32] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.size) }
    }

    /// Returns a mutable reference to the raw underlying buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn full_buffer_mut(&mut self) -> &mut [u32] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }
}

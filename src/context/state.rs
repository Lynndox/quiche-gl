//! Typestate patterns for a QuicheGL context

#[derive(Debug)]
pub struct Uninitialized;

pub struct Initialized {
    pub framebuffer: &'static mut [u32],
}

impl core::fmt::Debug for Initialized {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Initialized")
            .field("framebuffer_ptr", &self.framebuffer.as_ptr())
            .finish()
    }
}

//! Typestate patterns for a QuicheGL context

pub struct Uninitialized;
pub struct Initialized {
    pub framebuffer: &'static mut [u32],
}

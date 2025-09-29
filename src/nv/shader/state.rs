use crate::Align16;

#[repr(transparent)]
pub struct NvShaderState {
    inner: Align16<NvShaderStateInner>,
}

impl core::ops::Deref for NvShaderState {
    type Target = NvShaderStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NvShaderState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[repr(C, packed)]
pub struct NvShaderStateInner {
    /// Flag bits
    pub flags: NvShaderFlags,
    /// Shaded vertex data stride
    pub stride: u8,
    /// Number of uniform inputs to the fragment shader (not used)
    pub _num_uniforms: u8,
    /// Number of varying inputs to the fragment shader
    pub num_varyings: u8,
    /// Fragment shader code address
    pub code_addr: u32,
    /// Fragment shader uniforms address
    pub unif_addr: u32,
    /// Shaded vertex data address
    ///
    /// # Note
    ///
    /// This must be 16-byte aligned if including clip coordinates in the header.
    pub vert_data_addr: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct NvShaderFlags(u8);

bitflags::bitflags! {
    impl NvShaderFlags: u8 {
        /// Flag indicating the fragment shader is single threaded
        const SingleThreaded = 0;
        /// Flag indicating the point size is included in shaded vertex data
        const IncludesPointSize = 1;
        /// Flag indicating clipping should be enabled
        const EnableClipping = 2;
        /// Flag indicating the clip coordinates header is included in shaded vertex data
        const IncludesClipCoords = 3;
    }
}

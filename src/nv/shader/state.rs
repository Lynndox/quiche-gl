use crate::{Align16, align16, mem::BusAddress};

#[repr(C, packed)]
pub struct NvShaderState {
    /// Flag bits
    pub flags: NvShaderFlags,
    /// Shaded vertex data stride
    pub stride: u8,
    /// Number of uniform inputs to the fragment shader (not used)
    pub _num_uniforms: u8,
    /// Number of varying inputs to the fragment shader
    pub num_varyings: u8,
    /// Fragment shader code address
    pub code_addr: BusAddress,
    /// Fragment shader uniforms address
    pub unif_addr: BusAddress,
    /// Shaded vertex data address
    ///
    /// # Note
    ///
    /// This must be 16-byte aligned if including clip coordinates in the
    /// header.
    pub vert_data_addr: BusAddress,
}

impl NvShaderState {
    pub fn new_aligned(
        flags: NvShaderFlags,
        stride: u8,
        _num_uniforms: u8,
        num_varyings: u8,
        code_addr: BusAddress,
        unif_addr: BusAddress,
        vert_data_addr: BusAddress,
    ) -> Align16<Self> {
        align16!(Self {
            flags,
            stride,
            _num_uniforms,
            num_varyings,
            code_addr,
            unif_addr,
            vert_data_addr,
        })
    }
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

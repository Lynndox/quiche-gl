use super::*;

#[repr(C, align(16))]
pub struct TextureShader<'a> {
    uniforms: TextureUniforms,
    frag_shader: ShaderCode<'a>,
}

impl<'a> TextureShader<'a> {
    pub const fn new(uniforms: TextureUniforms) -> Self {
        Self {
            uniforms,
            frag_shader: TEXTURE_FRAG_SHADER,
        }
    }
}

#[repr(C, packed)]
pub struct TextureUniforms {
    base_ptr: u32,
    config_1: u32,
    config_2: u32,
    config_3: u32,
}

impl TextureUniforms {
    pub fn new(base_ptr: u32, width: u16, height: u16) -> Self {
        Self {
            base_ptr,
            config_1: 0,
            config_2: 0,
            config_3: 0,
        }
    }

    pub fn config_1(mut self, config_1: u32) -> Self {
        self.config_1 = config_1;
        self
    }

    pub fn config_2(mut self, config_2: u32) -> Self {
        self.config_2 = config_2;
        self
    }

    pub fn config_3(mut self, config_3: u32) -> Self {
        self.config_3 = config_3;
        self
    }
}

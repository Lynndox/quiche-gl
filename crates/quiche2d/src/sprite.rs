use fixed::FixedI16;
use gl::primitive::nv::FixedVec2;
use gl::primitive::{ShadedVertex, TextureCoords};

#[repr(C)]
pub struct Sprite {
    quad: [ShadedVertex<TextureCoords>; 4],
    origin: FixedVec2,
    texture_bus_addr: u32,
}

impl Sprite {
    pub fn new(position: FixedVec2, width: i16, height: i16, texture: &[u8]) -> Self {
        let texture_bus_addr = gl::arm_to_bus_addr(texture.as_ptr() as usize);
        let width = FixedI16::from_num(width);
        let height = FixedI16::from_num(height);

        // TODO: no idea if these texture coordinates are correct.
        // what are s and t really? just going off of PeterLemon's examples kinda
        let quad = [
            // top left
            ShadedVertex::new(position, 1.0, 1.0, TextureCoords { s: 0.0, t: 0.0 }),
            // bottom left
            ShadedVertex::new(
                position.add_y(height),
                1.0,
                1.0,
                TextureCoords { s: 0.0, t: 1.0 },
            ),
            // bottom right
            ShadedVertex::new(
                position.add_y(height).add_x(width),
                1.0,
                1.0,
                TextureCoords { s: 1.0, t: 1.0 },
            ),
            // top right
            ShadedVertex::new(
                position.add_x(width),
                1.0,
                1.0,
                TextureCoords { s: 1.0, t: 0.0 },
            ),
        ];

        Self {
            quad,
            origin: FixedVec2::new(0, 0),
            texture_bus_addr,
        }
    }
}

pub struct SpriteOpts {
    pub position: FixedVec2,
    pub width: u16,
    pub height: u16,
}

use fixed::FixedI16;
use gl::primitive::nv::PixelCoord;
use gl::primitive::{TextureCoords, Vertex};

#[repr(C)]
pub struct Sprite {
    quad: [Vertex<TextureCoords>; 4],
    origin: PixelCoord,
    texture_bus_addr: u32,
}

impl Sprite {
    pub fn new(position: PixelCoord, width: i16, height: i16, texture: &[u8]) -> Self {
        let texture_bus_addr = gl::arm_to_bus_addr(texture.as_ptr() as usize);
        let width = FixedI16::from_num(width);
        let height = FixedI16::from_num(height);

        // TODO: no idea if these texture coordinates are correct.
        // what are s and t really? just going off of PeterLemon's examples kinda
        let quad = [
            // top left
            Vertex::new(position, 1.0, 1.0, TextureCoords { s: 0.0, t: 0.0 }),
            // bottom left
            Vertex::new(
                position.add_y(height),
                1.0,
                1.0,
                TextureCoords { s: 0.0, t: 1.0 },
            ),
            // bottom right
            Vertex::new(
                position.add_y(height).add_x(width),
                1.0,
                1.0,
                TextureCoords { s: 1.0, t: 1.0 },
            ),
            // top right
            Vertex::new(
                position.add_x(width),
                1.0,
                1.0,
                TextureCoords { s: 1.0, t: 0.0 },
            ),
        ];

        Self {
            quad,
            origin: PixelCoord::new(0, 0),
            texture_bus_addr,
        }
    }
}

pub struct SpriteOpts {
    pub position: PixelCoord,
    pub width: u16,
    pub height: u16,
}

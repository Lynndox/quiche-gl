use crate::Align16;

use super::nv::PixelCoords;

#[repr(C)]
pub struct Vertex<T> {
    inner: Align16<T>,
}

#[repr(C, packed)]
pub struct VertexInner<T> {
    pixel_x_y: PixelCoords,
    z: f32,
    w: f32,
    data: T,
}

#[repr(C, packed)]
pub struct TextureCoords {
    s: f32,
    t: f32,
}

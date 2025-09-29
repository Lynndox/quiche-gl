use crate::Align16;

use super::nv::PixelCoord;

#[repr(C, packed)]
pub struct Vertex<T> {
    location: PixelCoord,
    z: f32,
    w: f32,
    data: T,
}

impl<T> Vertex<T> {
    pub fn new(location: PixelCoord, z: f32, w: f32, data: T) -> Self {
        Self {
            location,
            z,
            w,
            data,
        }
    }
}

impl<T: Clone> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        let data = unsafe { core::ptr::read_unaligned(&raw const self.data) };
        Self {
            location: self.location,
            z: self.z,
            w: self.w,
            data,
        }
    }
}

impl<T: Copy> Copy for Vertex<T> {}

#[repr(C, packed)]
pub struct TextureCoords {
    pub s: f32,
    pub t: f32,
}

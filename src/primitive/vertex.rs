use crate::Align16;

use super::nv::FixedVec2;

#[repr(C, packed)]
pub struct ShadedVertex<T> {
    location: FixedVec2,
    z: f32,
    w: f32,
    data: T,
}

impl<T> ShadedVertex<T> {
    pub fn new(location: FixedVec2, z: f32, w: f32, data: T) -> Self {
        Self {
            location,
            z,
            w,
            data,
        }
    }
}

impl<T: Clone> Clone for ShadedVertex<T> {
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

impl<T: Copy> Copy for ShadedVertex<T> {}

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

#[repr(C, packed)]
pub struct TextureCoords {
    pub s: f32,
    pub t: f32,
}

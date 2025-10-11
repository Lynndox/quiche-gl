use core::num::Saturating;
use fixed::types::I12F4;

// TODO: i16 or u16?
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FixedVec2 {
    pub x: I12F4,
    pub y: I12F4,
}

impl FixedVec2 {
    /// Creates a new instance of self from an X/Y position specified in pixel
    /// space by converting the supplied values to 12.4 fixed point numbers.
    ///
    /// # Note
    ///
    /// This function truncates the upper 4 bits of the supplied value. If this
    /// is undesirable, either use [`saturating_new`](Self::saturating_new)
    /// or handle the conversion beforehand
    /// and use [`from_fixed`](Self::from_fixed).
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            x: I12F4::from_num(x),
            y: I12F4::from_num(y),
        }
    }

    /// Creates a new instance of self from an X/Y position specified in pixel
    /// space by first clamping the values to the maximum a `u12` can hold
    /// (`0xFFF`), then shifting the values left by 4 bits.
    pub fn saturating_new(x: i16, y: i16) -> Self {
        Self {
            x: I12F4::saturating_from_num(x),
            y: I12F4::saturating_from_num(y),
        }
    }

    /// Creates a new instance of self from an X/Y position specified in pixel
    /// space already converted to [`PixelCoord`]s.
    pub fn from_fixed(x: I12F4, y: I12F4) -> Self {
        Self { x, y }
    }

    pub fn add_x(self, value: I12F4) -> Self {
        Self {
            x: self.x + value,
            y: self.y,
        }
    }

    pub fn add_y(self, value: I12F4) -> Self {
        Self {
            x: self.x,
            y: self.y + value,
        }
    }
}

#[cfg(feature = "glam")]
impl From<FixedVec2> for glam::f32::Vec2 {
    fn from(value: FixedVec2) -> Self {
        Self {
            x: value.x.to_num(),
            y: value.y.to_num(),
        }
    }
}

#[cfg(feature = "glam")]
impl From<glam::f32::Vec2> for FixedVec2 {
    fn from(value: glam::f32::Vec2) -> Self {
        Self {
            x: I12F4::from_num(value.x),
            y: I12F4::from_num(value.y),
        }
    }
}

impl core::ops::Add for FixedVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

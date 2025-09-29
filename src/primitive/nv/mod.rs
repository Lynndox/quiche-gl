use core::{num::Saturating, ops::*};

use fixed::{FixedI16, types::extra::U4};

// TODO: i16 or u16?
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PixelCoord {
    pub x: FixedI16<U4>,
    pub y: FixedI16<U4>,
}

impl PixelCoord {
    /// Creates a new instance of self from an X/Y position specified in pixel space by converting
    /// the supplied values to 12.4 fixed point numbers.
    ///
    /// # Note
    ///
    /// This function truncates the upper 4 bits of the supplied value. If this is undesirable,
    /// either use [`saturating_new`](Self::saturating_new) or handle the conversion to
    /// [`PixelCoord`]s beforehand and use [`from_coords`](Self::from_coords).
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            x: FixedI16::from_num(x),
            y: FixedI16::from_num(y),
        }
    }

    /// Creates a new instance of self from an X/Y position specified in pixel space by first
    /// clamping the values to the maximum a `u12` can hold (`0xFFF`), then shifting the values
    /// left by 4 bits.
    pub fn saturating_new(x: i16, y: i16) -> Self {
        Self {
            x: FixedI16::saturating_from_num(x),
            y: FixedI16::saturating_from_num(y),
        }
    }

    /// Creates a new instance of self from an X/Y position specified in pixel space already
    /// converted to [`PixelCoord`]s.
    pub fn from_fixed(x: FixedI16<U4>, y: FixedI16<U4>) -> Self {
        Self { x, y }
    }

    pub fn add_x(self, value: FixedI16<U4>) -> Self {
        Self {
            x: self.x + value,
            y: self.y,
        }
    }

    pub fn add_y(self, value: FixedI16<U4>) -> Self {
        Self {
            x: self.x,
            y: self.y + value,
        }
    }
}

impl Add for PixelCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

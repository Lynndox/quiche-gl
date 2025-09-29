#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct PixelCoords {
    pub x: PixelCoord,
    pub y: PixelCoord,
}

impl PixelCoords {
    /// Creates a new instance of self from an X/Y position specified in pixel space by converting
    /// the supplied values to 12.4 fixed point numbers.
    ///
    /// # Note
    ///
    /// This function truncates the upper 4 bits of the supplied value. If this is undesirable,
    /// either use [`saturating_new`](Self::saturating_new) or handle the conversion to
    /// [`PixelCoord`]s beforehand and use [`from_coords`](Self::from_coords).
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x: PixelCoord::from_u16(x),
            y: PixelCoord::from_u16(y),
        }
    }

    /// Creates a new instance of self from an X/Y position specified in pixel space by first
    /// clamping the values to the maximum a `u12` can hold (`0xFFF`), then shifting the values
    /// left by 4 bits.
    pub fn saturating_new(x: u16, y: u16) -> Self {
        Self {
            x: PixelCoord::saturating_from_u16(x),
            y: PixelCoord::saturating_from_u16(y),
        }
    }

    /// Creates a new instance of self from an X/Y position specified in pixel space already
    /// converted to [`PixelCoord`]s.
    pub fn from_coords(x: PixelCoord, y: PixelCoord) -> Self {
        Self { x, y }
    }
}

/// A pixel space coordinate in 12.4 fixed point. This is the format that the VC4 expects in NV
/// shader state mode.
#[derive(Clone, Copy)]
pub struct PixelCoord(u16);

const U12_MAX: u16 = 0xFFF;

impl PixelCoord {
    /// Simple conversion from a [`u16`] by shifting the value left by 4 bits.
    ///
    /// This will truncate the upper 4 bits of the supplied value. If this is undesirable, either
    /// use [`saturating_from_u16`](Self::saturating_from_u16), or handle the conversion ahead of time and use
    /// [`from_u16_raw`](Self::from_u16_raw).
    pub fn from_u16(value: u16) -> Self {
        Self(value << 4)
    }

    /// Convert from a [`u16`] by first clamping the value to the maximum a `u12` can hold
    /// (`0xFFF`), then shifting the value left by 4 bits.
    pub fn saturating_from_u16(value: u16) -> Self {
        Self(value.min(U12_MAX) << 4)
    }

    /// Create a [`PixelCoord`] without doing any conversion.
    ///
    /// Note that this value **must** be in 12.4 fixed point. The caller must ensure this
    /// beforehand to avoid unexpected/unwanted results.
    pub fn from_u16_raw(value: u16) -> Self {
        Self(value)
    }

    /// Returns the raw value of self.
    ///
    /// Note that this is a 12.4 fixed point number. To get the integer or fractional values, use
    /// [`trunc`](Self::trunc) or [`fract`](Self::fract) respectively.
    pub fn raw_bits(&self) -> u16 {
        self.0
    }

    /// Returns the integer part of self. This means that non-integer numbers are always truncated
    /// towards zero.
    ///
    /// This function always returns the precise result.
    pub fn trunc(&self) -> u16 {
        self.0 >> 4
    }

    /// Returns the fractional part of self.
    ///
    /// This function always returns the precise result.
    pub fn fract(&self) -> u16 {
        self.0 & 0xF
    }
}

use core::ops::Deref;

use crate::mailbox::messages::*;
use crate::{FrameBuffer, Result};

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub virt_width: u32,
    pub virt_height: u32,
    pub bit_depth: u32,
}

impl Display {
    pub const fn new(width: u32, height: u32, bit_depth: u32) -> Self {
        Self {
            width,
            height,
            virt_width: width,
            virt_height: height,
            bit_depth,
        }
    }
}

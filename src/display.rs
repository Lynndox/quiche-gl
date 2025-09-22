use crate::{mailbox, Result};

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
}

impl Display {
    pub const fn new(width: u32, height: u32, bit_depth: u32) -> Self {
        Self {
            width,
            height,
            bit_depth,
        }
    }

    pub fn init(&self) -> Result<()> {
        let framebuffer_msg =
            mailbox::message::init_framebuffer(self.width, self.height, self.bit_depth);
        let qpu_enable_msg = mailbox::message::enable_qpu(250);

        todo!()
    }
}

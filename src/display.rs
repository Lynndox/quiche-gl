use crate::mailbox::messages::*;
use crate::{FrameBuffer, Result};

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

    /// Initialize the GPU with a double width framebuffer to prevent screen tearing.
    ///
    /// This is currently the only supported mode of initializing the framebuffer.
    pub fn init_double_buffered(&mut self) -> Result<FrameBuffer> {
        unsafe {
            InitQpu::message(250).send()?;

            let mut init_msg =
                InitFramebuffer::message(self.width, self.height, self.bit_depth, true);
            init_msg.send()?;

            let mut buf_ptr = init_msg.inner().buffer_ptr();
            while buf_ptr == 0 {
                init_msg.send()?;
                buf_ptr = init_msg.inner().buffer_ptr();
            }

            let mut buffer = core::slice::from_raw_parts_mut(
                (buf_ptr as usize) as *mut u32,
                (self.width * self.height) as usize,
            );

            assert!(init_msg.inner().virt_res.width / 2 == self.width);
            assert!(init_msg.inner().virt_res.height == self.height);

            Ok(FrameBuffer {
                buffer,
                width: self.width,
                height: self.height,
                bit_depth: init_msg.inner().bit_depth.bit_depth,
                double_buffered: true,
            })
        }
    }
}

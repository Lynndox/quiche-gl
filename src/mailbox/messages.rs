use super::*;
use crate::Result;

#[repr(transparent)]
pub struct FramebufferInit {
    message: Message<25>,
}

impl FramebufferInit {
    const CHANNEL: Channel = Channel::Prop;

    pub fn message(width: u32, height: u32, bit_depth: u32, double_buffer: bool) -> Self {
        let virt_width = if double_buffer { width * 2 } else { width };
        let message = Message::new_with_tags([
            // Sequence Of Concatenated Tags
            // Tag Identifier
            Tag::SetPhysicalDisplay,
            // Value Buffer Size In Bytes
            0x00000008,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000008,
            // Value Buffer
            width,
            // Value Buffer
            height,
            // Tag Identifier
            Tag::SetVirtualResolution,
            // Value Buffer Size In Bytes
            0x00000008,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000008,
            // Value Buffer
            virt_width,
            // Value Buffer
            height,
            // Tag Identifier
            Tag::SetBitDepth,
            // Value Buffer Size In Bytes
            0x00000004,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000004,
            // Value Buffer
            bit_depth,
            // Tag Identifier
            Tag::SetVirtualOffset,
            // Value Buffer Size In Bytes
            0x00000008,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000008,
            // Value Buffer
            0,
            // Value Buffer
            0,
            // Tag Identifier
            Tag::AllocateBuffer,
            // Value Buffer Size In Bytes
            0x00000008,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000008,
            // Value Buffer
            0,
            // Value Buffer
            0,
            // 0x0 (End Tag)
            Tag::End,
        ]);

        Self { message }
    }

    pub fn send(&mut self) -> Result<()> {
        unsafe {
            self.message.send(Self::CHANNEL)?;
        }
        Ok(())
    }

    pub fn phys_width(&self) -> u32 {
        self.read_buf(3)
    }

    pub fn phys_height(&self) -> u32 {
        self.read_buf(4)
    }

    pub fn virt_width(&self) -> u32 {
        self.read_buf(8)
    }

    pub fn virt_height(&self) -> u32 {
        self.read_buf(9)
    }

    pub fn bit_depth(&self) -> u32 {
        self.read_buf(13)
    }

    pub fn buf_ptr(&self) -> u32 {
        self.read_buf(22)
    }

    fn read_buf(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile(&raw const self.message.inner().tags[offset]) }
    }
}

impl Deref for FramebufferInit {
    type Target = Message<25>;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

#[repr(transparent)]
pub struct EnableQpu {
    message: Message<10>,
}

impl EnableQpu {
    const CHANNEL: Channel = Channel::Prop;

    pub fn message(clock_rate_mhz: u32) -> Self {
        let message = Message::new_with_tags([
            Tag::SetClockRate,
            // Value Buffer Size In Bytes
            0x00000008,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000008,
            // Value Buffer (V3D Clock ID)
            Clock::V3D,
            // Value Buffer (250MHz)
            clock_rate_mhz * 1000 * 1000,
            // Tag Identifier
            Tag::EnableQpu,
            // Value Buffer Size In Bytes
            0x00000004,
            // 1 bit (MSB) Request/Response Indicator (0=Request, 1=Response), 31 bits (LSB) Value Length In Bytes
            0x00000004,
            // Value Buffer (1 = Enable)
            1,
            // 0x0 (End Tag)
            Tag::End,
        ]);
        Self { message }
    }

    pub unsafe fn send(mut self) -> Result<()> {
        unsafe {
            self.message.send(Self::CHANNEL)?;
        }
        Ok(())
    }
}

impl Deref for EnableQpu {
    type Target = Message<10>;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qpu_enable() {
        let qpu_en = EnableQpu::message(250);
        assert_eq!(qpu_en.size(), 12 * 4);
        const EXPECTED: &[u32] = &[
            12,
            0,
            Tag::SetClockRate,
            0x8,
            0x8,
            Clock::V3D,
            250 * 1000 * 1000,
            Tag::EnableQpu,
            0x4,
            0x4,
            1,
            0,
        ];

        let raw_msg =
            unsafe { core::slice::from_raw_parts(&raw const qpu_en as *const u32, qpu_en.size()) };

        assert_eq!(raw_msg, EXPECTED);
    }

    #[test]
    fn test_fb_init() {
        let msg = FramebufferInit::message(640, 480, 32, false);
        assert_eq!(msg.size(), 27);

        const EXPECTED: &[u32] = &[
            27,
            0,
            Tag::SetPhysicalDisplay,
            0x8,
            0x8,
            640,
            480,
            Tag::SetVirtualResolution,
            0x8,
            0x8,
            640,
            480,
            Tag::SetBitDepth,
            0x4,
            0x4,
            32,
            Tag::SetVirtualOffset,
            0x8,
            0x8,
            0,
            0,
            Tag::AllocateBuffer,
            0x8,
            0x8,
            0,
            0,
            0,
        ];

        let raw_msg =
            unsafe { core::slice::from_raw_parts(&raw const msg as *const u32, msg.size()) };

        assert_eq!(raw_msg, EXPECTED);
    }
}

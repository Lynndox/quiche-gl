use super::*;
pub const fn init_framebuffer(width: u32, height: u32, bit_depth: u32) -> MessageInner<25> {
    MessageInner::new_with_tags([
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
        width,
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
    ])
}

pub const fn enable_qpu(clock_rate_mhz: u32) -> MessageInner<10> {
    MessageInner::new_with_tags([
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
    ])
}

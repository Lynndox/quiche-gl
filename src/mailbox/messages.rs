use super::tag::*;
use super::*;
use crate::{Result, align16, mem::BusAddress};

macro_rules! trait_impl {
    ($($n:ident, $channel:path),* $(,)?) => {
        $(
            impl $crate::mailbox::Sealed for $n {}
            impl $crate::mailbox::MailboxChannel for $n {
                const CHANNEL: Channel = $channel;
            }
        )*
    };
}

trait_impl! {
    InitFramebuffer, Channel::Prop,
    InitQpu, Channel::Prop,
}

#[repr(C)]
pub struct InitFramebuffer {
    pub(crate) phys_display: SetPhysicalDisplay,
    pub(crate) virt_res: SetVirtualResolution,
    pub(crate) bit_depth: SetBitDepth,
    pub(crate) virt_offset: SetVirtualOffset,
    pub(crate) alloc_buffer: AllocateBuffer,
}

impl InitFramebuffer {
    pub const fn new(width: u32, height: u32, bit_depth: u32, double_buffer: bool) -> Self {
        const {
            assert!(core::mem::size_of::<Self>() <= (u32::MAX as usize));
        }

        let virt_width = if double_buffer { width * 2 } else { width };

        Self {
            phys_display: SetPhysicalDisplay::new(width, height),
            virt_res: SetVirtualResolution::new(virt_width, height),
            bit_depth: SetBitDepth::new(bit_depth),
            virt_offset: SetVirtualOffset::new(0, 0),
            alloc_buffer: AllocateBuffer::new(),
        }
    }

    pub const fn message(
        width: u32,
        height: u32,
        bit_depth: u32,
        double_buffer: bool,
    ) -> Align16<MessageBatch<Self>> {
        align16!(MessageBatch::new(Self::new(
            width,
            height,
            bit_depth,
            double_buffer
        )))
    }
}

impl InitFramebuffer {
    pub fn buffer_ptr(&self) -> BusAddress {
        BusAddress(unsafe { core::ptr::read_volatile(&raw const self.alloc_buffer.base_addr) })
    }
}

#[repr(C, packed)]
pub struct InitQpu {
    clock_rate: SetClockRate,
    enable_qpu: EnableQpu,
}

impl InitQpu {
    pub const fn new(clock_rate_mhz: u32) -> Self {
        Self {
            clock_rate: SetClockRate::new(Clock::V3D, clock_rate_mhz * 1_000 * 1_000),
            enable_qpu: EnableQpu::new(true),
        }
    }

    pub const fn message(clock_rate_mhz: u32) -> Align16<MessageBatch<Self>> {
        align16!(MessageBatch::new(Self::new(clock_rate_mhz)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qpu_enable_message() {
        let qpu_en = InitQpu::message(250);
        const EXPECTED: &[u32] = &[
            12,
            0,
            Tag::SetClockRate,
            0x8,
            0x0,
            Clock::V3D,
            250 * 1000 * 1000,
            Tag::EnableQpu,
            0x4,
            0x0,
            1,
            0,
        ];

        assert_eq!(unsafe { qpu_en.as_bytes() }, EXPECTED);
    }

    #[test]
    fn fb_init_struct() {
        const EXPECTED: &[u32] = &[
            27,
            0,
            Tag::SetPhysicalDisplay,
            0x8,
            0x0,
            640,
            480,
            Tag::SetVirtualResolution,
            0x8,
            0x0,
            640,
            480,
            Tag::SetBitDepth,
            0x4,
            0x0,
            32,
            Tag::SetVirtualOffset,
            0x8,
            0x0,
            0,
            0,
            Tag::AllocateBuffer,
            0x8,
            0x0,
            0,
            0,
            0,
        ];

        let msg = InitFramebuffer::message(640, 480, 32, false);
        assert_eq!(unsafe { msg.as_bytes() }, EXPECTED);
    }

    #[test]
    fn tag_structs() {
        let pd_tag = &[Tag::SetPhysicalDisplay, 0x8, 0x0, 640, 480];
        let vr_tag = &[Tag::SetVirtualResolution, 0x8, 0x0, 640, 480];
        let bd_tag = &[Tag::SetBitDepth, 0x4, 0x0, 32];
        let vo_tag = &[Tag::SetVirtualOffset, 0x8, 0x0, 0, 0];
        let ab_tag = &[Tag::AllocateBuffer, 0x8, 0x0, 0, 0];

        let pd_s = SetPhysicalDisplay::new(640, 480);
        let vr_s = SetVirtualResolution::new(640, 480);
        let bd_s = SetBitDepth::new(32);
        let vo_s = SetVirtualOffset::new(0, 0);
        let ab_s = AllocateBuffer::new();

        assert_eq!(
            core::mem::size_of_val(pd_tag),
            core::mem::size_of_val(&pd_s)
        );
        assert_eq!(
            core::mem::size_of_val(vr_tag),
            core::mem::size_of_val(&vr_s)
        );
        assert_eq!(
            core::mem::size_of_val(bd_tag),
            core::mem::size_of_val(&bd_s)
        );
        assert_eq!(
            core::mem::size_of_val(vo_tag),
            core::mem::size_of_val(&vo_s)
        );
        assert_eq!(
            core::mem::size_of_val(ab_tag),
            core::mem::size_of_val(&ab_s)
        );

        assert_eq!(pd_tag, unsafe { pd_s.as_bytes() });
        assert_eq!(vr_tag, unsafe { vr_s.as_bytes() });
        assert_eq!(bd_tag, unsafe { bd_s.as_bytes() });
        assert_eq!(vo_tag, unsafe { vo_s.as_bytes() });
        assert_eq!(ab_tag, unsafe { ab_s.as_bytes() });
    }
}

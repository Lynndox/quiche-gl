use crate::mailbox::Channel;

macro_rules! tag_impl {
    (@traitimpl $n:ident $channel:path) => {
	impl $crate::mailbox::Sealed for $n {}
	impl $crate::mailbox::MailboxChannel for $n {
	    const CHANNEL: Channel = $channel;
	}
        impl $crate::mailbox::MailboxMessage for $n {
	    fn channel(&self) -> $crate::mailbox::Channel {
		<Self as $crate::mailbox::MailboxChannel>::CHANNEL
	    }
            fn status(&self) -> $crate::mailbox::RequestStatus {
                $crate::mailbox::RequestStatus::from(unsafe {
                    ::core::ptr::read_volatile(&raw const self.status)
                })
            }
        }
    };
    (@makestruct $n:ident { $($v:ident),* }) => {
	#[repr(C)]
	pub struct $n {
	    tag: u32,
	    size: u32,
	    status: $crate::mailbox::RequestStatus,
	    $(pub $v: u32,)*
	}
    };
    (@argty $t:ty) => { $t };
    (@argty) => { u32 };
    ($($n:ident { channel = $channel:path, size = $size:literal, $($v:ident $(: $t:ty)?),* $(,)? } $(,)?),*) => {
	$(
	tag_impl!(@makestruct $n { $($v),* });
	impl $n {
	    pub const fn new($($v: tag_impl!(@argty $($t)*)),*) -> Self {
		Self {
		    tag: $crate::mailbox::tag::Tag::$n,
		    size: $size,
		    status: $crate::mailbox::RequestStatus::Request,
		    $($v: $v as u32),*
		}
	    }
	}
	tag_impl!(@traitimpl $n $channel);
	)*
    };
}

macro_rules! tag_impl_noinput {
    (@val $val:literal) => { $val };
    (@val) => { 0 };
    ($($n:ident { channel = $channel:path, size = $size:literal$(, $($v:ident $(= $val:literal)?),*)? $(,)? } $(,)?),*) => {
	$(
	tag_impl!(@makestruct $n { $($($v),*),* });
	tag_impl!(@traitimpl $n $channel);
	impl $n {
	    pub const fn new() -> Self {
		Self {
		    tag: $crate::mailbox::tag::Tag::$n,
		    size: $size,
		    status: $crate::mailbox::RequestStatus::Request,
		    $($($v: tag_impl_noinput!(@val $($val)*)),*)*
		}
	    }
	}

	impl Default for $n {
	    fn default() -> Self {
		Self::new()
	    }
	})*
    };
}

tag_impl! {
    SetPhysicalDisplay { channel = Channel::Prop, size = 8, width, height },
    SetVirtualResolution { channel = Channel::Prop, size = 8, width, height },
    SetBitDepth { channel = Channel::Prop, size = 4, bit_depth },
    SetVirtualOffset { channel = Channel::Prop, size = 8, offset_x, offset_y },
    SetClockRate { channel = Channel::Prop, size = 8, clock, rate_mhz },
    EnableQpu { channel = Channel::Prop, size = 4, enable: bool }
}

tag_impl_noinput! {
    AllocateBuffer { channel = Channel::Prop, size = 8, base_addr, buf_size },
}

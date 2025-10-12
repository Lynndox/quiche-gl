use crate::mailbox::Channel;

macro_rules! volatile {
    (@ $($t:tt)*) => {
	$crate::mem::volatile::$($t)*
    };
    ($($t:tt)*) => {
	volatile!(@Volatile::new($($t)*))
    };
}

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
                $crate::mailbox::RequestStatus::from(
		    volatile!(@VolatileRead::read(&self.status))
		)
            }
        }
    };
    (@debugimpl $n:ident { $($v:ident),* }) => {
	impl ::core::fmt::Debug for $n {
	    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
		unsafe {
		    f.debug_struct(stringify!($n))
			$(.field(stringify!($v), &format_args!("0x{:08x}", &*self.$v)))*
			.finish()
		}
	    }
	}
    };
    (@getimpl $name:ident { $($field:ident),* }) => {
    impl $name {

    }};
    (@makestruct $n:ident { $($v:ident),* }) => {
	#[repr(C, align(4))]
	pub struct $n {
	    tag: u32,
	    size: u32,
	    pub status: volatile!(@Volatile<u32, volatile!(@Read)>),
	    $(pub $v: volatile!(@Volatile<u32, volatile!(@ReadWrite)>),)*
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
		    status: volatile!($size),
		    $($v: volatile!($v as u32)),*
		}
	    }
	}
	tag_impl!(@traitimpl $n $channel);
	tag_impl!(@debugimpl $n { $($v),* });
	)*
    };
}

macro_rules! tag_impl_noinput {
    (@val $val:literal) => { $val };
    (@val) => { 0 };
    ($($n:ident { channel = $channel:path, size = $size:literal$(, $($v:ident $(= $val:literal)?),*)? $(,)? } $(,)?),*) => {
	$(
	tag_impl!(@makestruct $n { $($($v),*),* });
	#[allow(clippy::new_without_default)]
	impl $n {
	    pub const fn new() -> Self {
		Self {
		    tag: $crate::mailbox::tag::Tag::$n,
		    size: $size,
		    status: volatile!($size),
		    $($($v: volatile!(tag_impl_noinput!(@val $($val)*))),*)*
		}
	    }
	}

	tag_impl!(@traitimpl $n $channel);
	tag_impl!(@debugimpl $n { $($($v),*)* });
	)*
    };
}

// TODO: add read/write declarations to the fields
tag_impl! {
    SetPhysicalDisplay { channel = Channel::Prop, size = 8, width, height },
    SetVirtualResolution { channel = Channel::Prop, size = 8, width, height },
    SetBitDepth { channel = Channel::Prop, size = 4, bit_depth },
    SetVirtualOffset { channel = Channel::Prop, size = 8, offset_x, offset_y },
    GetClockRate { channel = Channel::Prop, size = 8, clock, rate_mhz },
    SetClockRate { channel = Channel::Prop, size = 12, clock, rate_mhz, skip_turbo: bool },
    EnableQpu { channel = Channel::Prop, size = 4, enable: bool }
}

tag_impl_noinput! {
    AllocateBuffer { channel = Channel::Prop, size = 8, base_addr, buf_size },
}

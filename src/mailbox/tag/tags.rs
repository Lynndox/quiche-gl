use crate::mailbox::Channel;

// macro_rules! trait_impl {
//     ($n:ident) => {
//         impl $crate::mailbox::Sealed for $n {}
//         impl $crate::mailbox::MailboxMessage for $n {
//             fn status(&self) -> $crate::mailbox::RequestStatus {
//                 $crate::mailbox::RequestStatus::from(unsafe {
//                     ::core::ptr::read_volatile(&raw const self.status)
//                 })
//             }
//         }
//     };
// }

macro_rules! tag_impl {
    (@traitimpl $n:ident) => {
	impl $crate::mailbox::Sealed for $n {}
        impl $crate::mailbox::MailboxMessage for $n {
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
    ($($n:ident { size = $size:literal, $($v:ident $(: $t:ty)?),* $(,)? } $(,)?),*) => {
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
	tag_impl!(@traitimpl $n);
	)*
    };
}

macro_rules! tag_impl_noinput {
    (@val $val:literal) => { $val };
    (@val) => { 0 };
    ($($n:ident { size = $size:literal$(, $($v:ident $(= $val:literal)?),*)? $(,)? } $(,)?),*) => {
	$(
	tag_impl!(@makestruct $n { $($($v),*),* });
	tag_impl!(@traitimpl $n);
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
	)*
    };
}

tag_impl! {
    SetPhysicalDisplay { size = 8, width, height },
    SetVirtualResolution { size = 8, width, height },
    SetBitDepth { size = 4, bit_depth },
    SetVirtualOffset { size = 8, offset_x, offset_y },
    SetClockRate { size = 8, clock, rate_mhz },
    EnableQpu { size = 4, enable: bool }
}

tag_impl_noinput! {
    AllocateBuffer { size = 8, base_addr, buf_size },
}

mod error;
pub mod messages;
pub(crate) mod raw;
pub mod tag;

pub use error::*;

use crate::Align16;
use crate::mailbox::messages::*;
use crate::mem::{ArmAddress, Physical};

pub(crate) const MAIL_BASE: ArmAddress<Physical> = ArmAddress::new(0x3F00B880);

// FIXME:
// I don't like this, and I don't like how messy this has become
// but idk what to do about it rn so it's like this

pub trait MailboxChannel: Sealed {
    const CHANNEL: Channel;
}

pub trait MailboxMessage: Sealed {
    fn channel(&self) -> Channel;
    fn status(&self) -> crate::mailbox::RequestStatus;
    fn size(&self) -> u32 {
        (core::mem::size_of_val(self) / 4) as u32
    }

    /// Gets the raw bytes that make up this struct.
    ///
    /// # Safety
    ///
    /// With great power comes great responsibility. You should not use this unless you have a very
    /// good reason to.
    unsafe fn as_bytes(&self) -> &[u32] {
        unsafe { core::slice::from_raw_parts(&raw const *self as *const u32, self.size() as _) }
    }
}

impl<T: MailboxChannel> Sealed for MessageBatch<T> {}
impl<T: MailboxChannel> MailboxMessage for MessageBatch<T> {
    fn channel(&self) -> Channel {
        T::CHANNEL
    }
    fn status(&self) -> crate::mailbox::RequestStatus {
        crate::mailbox::RequestStatus::from(unsafe {
            ::core::ptr::read_volatile(&raw const self.status)
        })
    }
}

#[repr(C)]
pub struct MessageBatch<T: MailboxChannel> {
    size: u32,
    status: u32,
    messages: T,
    end_tag: u32,
}

impl<T: MailboxChannel> MessageBatch<T> {
    pub const fn new(messages: T) -> Self {
        const {
            assert!(((core::mem::size_of::<Self>() / 4) - 1) <= u32::MAX as usize);
        }
        Self {
            size: (core::mem::size_of::<Self>() / 4) as u32,
            status: 0,
            messages,
            end_tag: 0,
        }
    }
}

impl<T: MailboxChannel> core::ops::Deref for MessageBatch<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.messages
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Channel {
    Power = 0,
    FB = 1,
    VUART = 2,
    VCHIQ = 3,
    LEDs = 4,
    Buttons = 5,
    Touch = 6,
    Count = 7,
    Prop = 8,
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MailboxStatus {
    Full = 0x80000000,
    Empty = 0x40000000,
}

impl PartialEq<u32> for MailboxStatus {
    fn eq(&self, other: &u32) -> bool {
        (*self as u32).eq(other)
    }
}

impl PartialEq<MailboxStatus> for u32 {
    fn eq(&self, other: &MailboxStatus) -> bool {
        self.eq(&(*other as u32))
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum RequestStatus {
    Request = 0x00000000,
    Success = 0x80000000,
    Error = 0x80000001,
}

impl From<u32> for RequestStatus {
    fn from(val: u32) -> Self {
        match val {
            0x00000000 => Self::Request,
            0x80000000 => Self::Success,
            _ => Self::Error,
        }
    }
}

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

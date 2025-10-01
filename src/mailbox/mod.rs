mod error;
pub mod messages;
mod raw;
pub mod tag;

pub use error::*;

use crate::Align16;
use crate::mailbox::messages::*;

// FIXME: this shouldn't be a constant, but passed in by the user somehow, since
// if the mailbox address has been remapped by the MMU, this will break.
const MAIL_BASE: u32 = 0x3F00B880;

// I don't like this, and I don't like how messy this has become
// but idk what to do about it rn so it's like this

pub trait MailboxChannel: Sealed {
    const CHANNEL: Channel;
}

trait MailboxMessage: Sealed {
    fn status(&self) -> crate::mailbox::RequestStatus;
    fn size(&self) -> u32 {
        (core::mem::size_of_val(self) / 4) as u32
    }
    unsafe fn as_bytes(&self) -> &[u32] {
        unsafe { core::slice::from_raw_parts(&raw const *self as *const u32, self.size() as _) }
    }
}

impl<T: MailboxChannel> Sealed for MessageBatchInner<T> {}
impl<T: MailboxChannel> MailboxMessage for MessageBatchInner<T> {
    fn status(&self) -> crate::mailbox::RequestStatus {
        crate::mailbox::RequestStatus::from(unsafe {
            ::core::ptr::read_volatile(&raw const self.status)
        })
    }
}

#[repr(transparent)]
pub struct MessageBatch<T: MailboxChannel> {
    inner: Align16<MessageBatchInner<T>>,
}

impl<T: MailboxChannel> MessageBatch<T> {
    pub const fn new(message: T) -> Self {
        Self {
            inner: Align16::new(MessageBatchInner::new(message)),
        }
    }

    pub fn inner(&self) -> &T {
        &self.inner.message
    }
}

impl<T: MailboxChannel> core::ops::Deref for MessageBatch<T> {
    type Target = MessageBatchInner<T>;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<T: MailboxChannel> core::ops::DerefMut for MessageBatch<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}
#[repr(C)]
pub struct MessageBatchInner<T: MailboxChannel> {
    size: u32,
    status: u32,
    message: T,
    end_tag: u32,
}

impl<T: MailboxChannel> MessageBatchInner<T> {
    pub const fn new(message: T) -> Self {
        const {
            assert!(((core::mem::size_of::<Self>() / 4) - 1) <= u32::MAX as usize);
        }
        Self {
            size: (core::mem::size_of::<Self>() / 4) as u32,
            status: 0,
            message,
            end_tag: 0,
        }
    }

    /// Sends a message to the mailbox.
    ///
    /// # Safety
    ///
    /// The current implementation will spin until the mailbox is empty, but does not do any sort
    /// of proper synchronization. This has the potential for race conditions.
    ///
    /// It is the caller's responsibility to ensure that only one thread is using the mailbox at a
    /// time.
    pub unsafe fn send(&mut self) -> Result<(), MailboxError> {
        let mailbox_addr = ((&raw const *self) as u32 & !0x0F) | T::CHANNEL as u32;
        let mailbox = unsafe { raw::unmapped_mailbox() };

        // TODO: spin loop bad. replace this with interrupts or something
        while mailbox.is_full() {
            core::hint::spin_loop();
        }

        mailbox.write(mailbox_addr);

        loop {
            // TODO: again, spin loop bad. replace this with interrupts or something
            while mailbox.is_empty() {
                core::hint::spin_loop();
            }
            if mailbox.read() == mailbox_addr {
                return match self.status() {
                    RequestStatus::Request => Err(MailboxError::SendMessage(
                        "Message still contains a request?!",
                    )),
                    // TODO: check error response and return a more useful error here
                    RequestStatus::Error => {
                        Err(MailboxError::SendMessage("Response contains an error."))
                    }
                    RequestStatus::Success => Ok(()),
                };
            }
        }
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

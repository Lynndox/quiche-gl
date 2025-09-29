use core::ops::{Deref, DerefMut};

mod error;
pub mod message;
mod raw;
mod tags;

pub use error::*;
pub use tags::*;

use crate::Align16;

// FIXME: this shouldn't be a constant, but passed in by the user somehow, since
// if the mailbox address has been remapped by the MMU, this will break.
const MAIL_BASE: u32 = 0x3F00B880;

const MBOX_REQUEST: u32 = 0;

#[repr(transparent)]
pub struct Message<const LEN: usize> {
    inner: Align16<MessageInner<LEN>>,
}

impl<const LEN: usize> Default for Message<LEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const LEN: usize> Message<LEN> {
    pub const fn new() -> Self {
        Self {
            inner: Align16::new(MessageInner::new()),
        }
    }

    pub const fn new_with_tags(tags: [u32; LEN]) -> Self {
        Self {
            inner: Align16::new(MessageInner::new_with_tags(tags)),
        }
    }

    /// Obtain a reference to the inner message.
    pub fn inner(&self) -> &MessageInner<LEN> {
        self.inner.deref()
    }

    /// Obtain a mutable reference to the inner message.
    pub fn inner_mut(&mut self) -> &mut MessageInner<LEN> {
        self.inner.deref_mut()
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
    pub unsafe fn send(&mut self, channel: Channel) -> Result<(), MailboxError> {
        let mailbox_addr = ((&raw const *self.inner) as u32 & !0x0F) | channel as u32;
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
                return self.inner.response();
            }
        }
    }
}

#[repr(C, packed)]
pub struct MessageInner<const LEN: usize> {
    size: u32,
    response: u32,
    tags: [u32; LEN],
}

impl<const LEN: usize> MessageInner<LEN> {
    pub(crate) const fn new() -> Self {
        Self::new_with_tags([0u32; LEN])
    }

    pub(crate) const fn new_with_tags(tags: [u32; LEN]) -> Self {
        Self {
            size: (LEN + 2) as u32,
            response: MBOX_REQUEST,
            tags,
        }
    }

    pub(crate) fn response(&self) -> Result<(), MailboxError> {
        match self.request_status() {
            RequestStatus::Request => Err(MailboxError::SendMessage(
                "Message still contains a request?!",
            )),
            // TODO: check error response and return a more useful error here
            RequestStatus::Error => Err(MailboxError::SendMessage("Response contains an error.")),
            RequestStatus::Success => Ok(()),
        }
    }

    pub(crate) fn request_status(&self) -> RequestStatus {
        RequestStatus::from(self.response)
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
    Success = 0x00000000,
    Error = 0x80000000,
    Request = 0x80000001,
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

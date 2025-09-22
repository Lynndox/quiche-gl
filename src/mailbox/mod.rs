use core::ops::{Deref, DerefMut};

mod error;
pub mod message;
mod tags;

pub use error::*;
pub use tags::*;

use crate::Align16;

const MAIL_BASE: u32 = 0x3F00B880;
const MBOX_REQUEST: u32 = 0;

#[repr(C)]
struct Mailbox {
    read: *const u32,
    _unused: u32,
    _unused2: u32,
    _unused3: u32,
    poll: u32,
    sender: u32,
    status: *const u32,
    config: u32,
    write: *mut u32,
}

impl Mailbox {
    fn is_empty(&self) -> bool {
        self.status() == MailboxStatus::Empty
    }

    fn status(&self) -> u32 {
        unsafe { ::core::ptr::read_volatile(self.status) }
    }
}

/// Get a reference to the [`Mailbox`].
///
/// # Safety
///
/// The caller must ensure that only one core is accessing the mailbox at a time.
const unsafe fn mailbox() -> &'static Mailbox {
    unsafe { &*(MAIL_BASE as *const Mailbox) }
}

/// Get a mutable reference to the [`Mailbox`].
///
/// # Safety
///
/// The caller must ensure that only one core is accessing the mailbox at a time.
const unsafe fn mailbox_mut() -> &'static mut Mailbox {
    unsafe { &mut *(MAIL_BASE as *mut Mailbox) }
}

#[repr(transparent)]
pub struct Message<const LEN: usize> {
    inner: Align16<MessageInner<LEN>>,
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

    /// Sends a message to the mailbox.
    ///
    /// # Safety
    ///
    /// The current implementation will spin until the mailbox is empty, but does not do any sort
    /// of proper synchronization. This has the potential for race conditions.
    ///
    /// It is the caller's responsibility to ensure that only one thread is using the mailbox at a
    /// time.
    pub unsafe fn send(&self, channel: Channel) -> Result<(), MailboxError> {
        todo!()
    }
}

impl<const LEN: usize> Deref for Message<LEN> {
    type Target = MessageInner<LEN>;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<const LEN: usize> DerefMut for Message<LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
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
pub enum ReqResp {
    ResponseSuccessful = 0x00000000,
    ResponseError = 0x80000000,
    Request = 0x80000001,
}

impl From<u32> for ReqResp {
    fn from(val: u32) -> Self {
        use ReqResp::*;
        match val {
            0x00000000 => Request,
            0x80000000 => ResponseSuccessful,
            _ => ResponseError,
        }
    }
}

use crate::mem::{ArmAddress, BusAddress, Virtual};

use super::MailboxStatus;

#[repr(C)]
pub struct RawMailbox {
    read: *const u32,
    _unused: [u32; 3],
    poll: u32,
    sender: u32,
    status: *const u32,
    config: u32,
    write: *mut u32,
}

impl RawMailbox {
    pub fn read(&self) -> u32 {
        unsafe { ::core::ptr::read_volatile(self.read) }
    }

    /// Writes data to the `write` register of the mailbox.
    ///
    /// If sending a bus address, as is typically the case, prefer [`Self::write_address`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value being written is valid.
    pub unsafe fn write(&self, value: u32) {
        unsafe {
            ::core::ptr::write_volatile(self.write, value);
        }
    }

    pub fn status(&self) -> u32 {
        unsafe { ::core::ptr::read_volatile(self.status) }
    }

    pub fn is_full(&self) -> bool {
        self.status() == MailboxStatus::Full
    }

    pub fn is_empty(&self) -> bool {
        self.status() == MailboxStatus::Empty
    }

    /// Writes a bus address to the mailbox's `write` register.
    ///
    /// This is analogous to [`Self::write`] with a small added layer of safety by taking a
    /// [`BusAddress`] instead of a raw [`u32`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value being written is valid.
    pub unsafe fn write_address(&self, addr: impl Into<BusAddress>) {
        unsafe {
            self.write(*addr.into());
        };
    }
}

/// Get a reference to the [`Mailbox`].
///
/// # Safety
///
/// The caller must ensure that only one core is accessing the mailbox at a time.
pub const unsafe fn mailbox(addr: ArmAddress<Virtual>) -> &'static RawMailbox {
    unsafe { &*(addr.addr as *const RawMailbox) }
}

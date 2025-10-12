use crate::mem::{ArmAddress, BusAddress, Physical, Virtual};
use core::{cell::UnsafeCell, ptr};

use super::MailboxStatus;

use crate::mem::volatile::*;

#[repr(C)]
pub struct Mailbox {
    read: Volatile<u32, Read>,
    _unused: u32,
    _unused1: u32,
    _unused2: u32,
    poll: Volatile<u32, Read>,
    sender: Volatile<u32, Read>,
    status: Volatile<u32, Read>,
    config: Volatile<u32, Read>,
    write: VolatileMut<u32, Write>,
}

impl core::fmt::Debug for Mailbox {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe {
            f.debug_struct("RawMailbox")
                .field("read", &format_args!("0x{:08x}", &self.read.read()))
                .field("poll", &format_args!("0x{:08x}", &self.poll.read()))
                .field("sender", &format_args!("0x{:08x}", &self.sender.read()))
                .field("status", &format_args!("0x{:08x}", &self.status.read()))
                .field("config", &format_args!("0x{:08x}", &self.config.read()))
                .field(
                    "write",
                    &format_args!(
                        "0x{:08x} <readonly>",
                        ptr::read_volatile(&*self.write.as_ptr())
                    ),
                )
                .finish()
        }
    }
}

impl Mailbox {
    pub fn read(&self) -> u32 {
        self.read.read()
    }

    /// Writes data to the `write` register of the mailbox.
    ///
    /// If sending a bus address, as is typically the case, prefer
    /// [`Self::write_address`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value being written is valid.
    pub unsafe fn write(&mut self, value: u32) {
        self.write.write(value)
    }

    pub fn status(&self) -> u32 {
        self.status.read()
    }

    pub fn is_full(&self) -> bool {
        (self.status() & MailboxStatus::Full as u32)
            == MailboxStatus::Full as u32
    }

    pub fn is_empty(&self) -> bool {
        (self.status() & MailboxStatus::Empty as u32)
            == MailboxStatus::Empty as u32
    }

    /// Writes a bus address to the mailbox's `write` register.
    ///
    /// This is analogous to [`Self::write`] with a small added layer of safety
    /// by taking a [`BusAddress`] instead of a raw [`u32`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value being written is valid.
    pub unsafe fn write_address(&mut self, addr: ArmAddress<Physical>) {
        unsafe {
            self.write(addr.addr as u32);
        };
    }
}

/// Get a reference to the [`Mailbox`].
///
/// # Safety
///
/// The caller must ensure that only one core is accessing the mailbox at a
/// time.
pub const unsafe fn mailbox(addr: ArmAddress<Virtual>) -> &'static mut Mailbox {
    unsafe { &mut *(addr.addr as *mut Mailbox) }
}

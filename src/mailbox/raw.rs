use super::MailboxStatus;

const MAIL_BASE: u32 = 0x3F00B880;

#[repr(C)]
pub struct Mailbox {
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
    pub fn read(&self) -> u32 {
        unsafe { ::core::ptr::read_volatile(self.read) }
    }

    pub fn write(&self, value: u32) {
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
}

/// Get a reference to the [`Mailbox`].
///
/// # Safety
///
/// The caller must ensure that only one core is accessing the mailbox at a time.
pub const unsafe fn mailbox() -> &'static Mailbox {
    unsafe { &*(MAIL_BASE as *const Mailbox) }
}

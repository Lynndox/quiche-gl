#[derive(Debug)]
pub enum Error {
    Mailbox(crate::mailbox::MailboxError),
}

impl From<crate::mailbox::MailboxError> for Error {
    fn from(value: crate::mailbox::MailboxError) -> Self {
        Self::Mailbox(value)
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(_value: core::convert::Infallible) -> Self {
        unreachable!(
            "This exploded somewhere that was supposed to be infalliable. Someone has done something very nasty."
        )
    }
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Mailbox(mailbox_error) => {
                write!(f, "Mailbox error: {}", mailbox_error)
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Initialization(crate::InitializeError),
    Context(crate::ContextError),
    Mailbox(crate::mailbox::MailboxError),
}

impl From<crate::InitializeError> for Error {
    fn from(v: crate::InitializeError) -> Self {
        Self::Initialization(v)
    }
}

impl From<crate::ContextError> for Error {
    fn from(v: crate::ContextError) -> Self {
        Self::Context(v)
    }
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
            Error::Mailbox(err) => {
                write!(f, "Mailbox error: {err}")
            }
            Error::Initialization(err) => {
                write!(f, "Initialization error: {err}")
            }
            Error::Context(err) => write!(f, "Context error: {err}"),
        }
    }
}

#[derive(Debug)]
pub enum MailboxError {
    SendMessage(&'static str),
    SetVirtOffset,
    SetClockSpeed,
    GetMaxSpeed,
    FrameBufferInit { addr: u32 },
}

impl core::fmt::Display for MailboxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MailboxError::SendMessage(c) => {
                write!(f, "Failed to send mailbox message: {}", c)
            }
            MailboxError::SetVirtOffset => {
                write!(f, "Failed to set virtual framebuffer offset.")
            }
            MailboxError::GetMaxSpeed => {
                write!(f, "Failed to get max clock speed")
            }
            MailboxError::FrameBufferInit { addr } => write!(
                f,
                "Failed to initialize framebuffer. Received address: {}",
                addr
            ),
            MailboxError::SetClockSpeed => {
                write!(f, "Failed to set clock speed.")
            }
        }
    }
}

impl core::error::Error for MailboxError {}

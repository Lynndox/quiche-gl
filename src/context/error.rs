use core::error::Error as CoreError;
use core::fmt;

#[derive(Debug)]
pub enum ContextError {
    Other(&'static str),
}

impl CoreError for ContextError {}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextError::Other(s) => f.write_str(s),
        }
    }
}

#[derive(Debug)]
pub enum InitializeError {
    NullFrameBufferPtr,
    ZeroSizedBuffer,
}

impl CoreError for InitializeError {}

impl fmt::Display for InitializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitializeError::NullFrameBufferPtr => {
                f.write_str("Received a null frame buffer pointer")
            }
            InitializeError::ZeroSizedBuffer => {
                f.write_str("Received a zero sized frame buffer")
            }
        }
    }
}

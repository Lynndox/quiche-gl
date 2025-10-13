use core::fmt;

#[derive(Debug)]
pub struct Uninitialized;

impl fmt::Display for Uninitialized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Uninitialized")
    }
}

#[derive(Debug)]
pub struct Initialized;

impl fmt::Display for Initialized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Initialized")
    }
}

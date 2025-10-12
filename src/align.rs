use core::ops::{Deref, DerefMut};

/// A wrapper for some data to aligns the inner `T` to 16 bytes.
///
/// This is important for data sent to the QPU, as it _must_ be aligned to 16
/// byte boundaries.
#[repr(align(16))]
pub struct Align16<T> {
    inner: T,
}

impl<T> Clone for Align16<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Copy for Align16<T> where T: Copy {}

impl<T> Align16<T> {
    /// Wrap [`T`] to align it to a 16 byte boundary.
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consumes [`self`], returning the inner wrapped [`T`].
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Align16<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Align16<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

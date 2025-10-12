pub struct Read;
pub struct Write;
pub struct ReadWrite;

/// Performs a volatile read of the value contained in self.
///
/// See [`read_volatile`](core::ptr::read_volatile) for more details.
pub trait VolatileRead<T: Copy>: Sealed {
    /// Performs a volatile read of the value contained in self.
    ///
    /// See [`read_volatile`](core::ptr::read_volatile) for more details.
    fn read(&self) -> T;
}

/// Performs a volatile read of the value contained in self, whether or not
/// `T` is copy.
///
/// This method is implemented for any `T` due to the lack of negative trait
/// bounds. If `T` is copy, prefer [`VolatileRead::read`].
///
/// See [`read_volatile`](core::ptr::read_volatile) for more details.
pub trait UnsafeVolatileRead<T>: Sealed {
    /// Performs a volatile read of the value contained in self, whether or not
    /// `T` is copy.
    ///
    /// This method is implemented for any `T` due to the lack of negative trait
    /// bounds. If `T` is copy, prefer [`VolatileRead::read`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that all safety requirements from
    /// [`read_volatile`](core::ptr::read_volatile) are upheld.
    ///
    /// See that method's definition for more details.
    unsafe fn read_noncopy(&self) -> T;
}

/// Performs a volatile write of a memory location with the given value
/// without reading or dropping the old value.
///
/// See [`write_volatile`](core::ptr::write_volatile) for more details.
pub trait VolatileWrite<T>: Sealed {
    /// Performs a volatile write of a memory location with the given value
    /// without reading or dropping the old value.
    ///
    /// See [`write_volatile`](core::ptr::write_volatile) for more details.
    fn write(&mut self, value: T);
}

pub trait VolatileAccess: Sealed {}
impl<T> VolatileAccess for T where T: Sealed {}

impl<T: Copy> VolatileRead<T> for Volatile<T, Read> {
    fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(&raw const self.inner) }
    }
}

impl<T> UnsafeVolatileRead<T> for Volatile<T, Read> {
    unsafe fn read_noncopy(&self) -> T {
        unsafe { core::ptr::read_volatile(&raw const self.inner) }
    }
}

impl<T> VolatileWrite<T> for VolatileMut<T, Write> {
    fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(self.inner.get(), value) }
    }
}

impl<T: Copy> VolatileRead<T> for Volatile<T, ReadWrite> {
    fn read(&self) -> T {
        unsafe { core::ptr::read_volatile(&raw const self.inner) }
    }
}

impl<T> UnsafeVolatileRead<T> for Volatile<T, ReadWrite> {
    unsafe fn read_noncopy(&self) -> T {
        unsafe { core::ptr::read_volatile(&raw const self.inner) }
    }
}

impl<T> VolatileWrite<T> for VolatileMut<T, ReadWrite> {
    fn write(&mut self, value: T) {
        unsafe { core::ptr::write_volatile(self.inner.get(), value) }
    }
}

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use sealed::Sealed;
mod sealed {
    use super::*;

    pub trait Sealed {}
    impl Sealed for Read {}
    impl Sealed for Write {}
    impl Sealed for ReadWrite {}
    impl<T, A: Sealed> Sealed for Volatile<T, A> {}
    impl<T, A: Sealed> Sealed for VolatileMut<T, A> {}
}

#[repr(transparent)]
pub struct Volatile<T, A>
where
    A: VolatileAccess,
{
    inner: T,
    _phantom: PhantomData<A>,
}

impl<T, A> core::fmt::Debug for Volatile<T, A>
where
    T: core::fmt::Debug,
    A: VolatileAccess,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T, A> Volatile<T, A>
where
    A: VolatileAccess,
{
    pub const fn new(value: T) -> Self {
        Self {
            inner: value,
            _phantom: PhantomData,
        }
    }

    pub(crate) unsafe fn as_ptr(&self) -> *const T {
        &raw const self.inner
    }

    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut T {
        &raw mut self.inner
    }
}

impl<T, A> Deref for Volatile<T, A>
where
    A: VolatileAccess,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, A> DerefMut for Volatile<T, A>
where
    A: VolatileAccess,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[repr(transparent)]
pub struct VolatileMut<T, A>
where
    A: VolatileAccess,
{
    inner: UnsafeCell<T>,
    _phantom: PhantomData<A>,
}

impl<T, A> core::fmt::Debug for VolatileMut<T, A>
where
    T: core::fmt::Debug,
    A: VolatileAccess,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let inner_ref = unsafe { self.inner.get().as_ref().unwrap_unchecked() };
        inner_ref.fmt(f)
    }
}

impl<T, A> VolatileMut<T, A>
where
    A: VolatileAccess,
{
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            _phantom: PhantomData,
        }
    }

    pub(crate) unsafe fn as_ptr(&self) -> *const T {
        self.inner.get() as *const T
    }

    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.get()
    }
}

use core::{
    marker::PhantomData,
    ops::{BitOr, BitOrAssign, Deref, DerefMut},
};

use crate::{VC_BUS_ADDR, mailbox::Channel};

// FIXME: I don't like this interface at all, but it gives me the guarantees I
// need for now.

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArmAddress<A> {
    pub addr: usize,
    _phantom: PhantomData<A>,
}

impl core::fmt::Debug for ArmAddress<Physical> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ArmAddress<Physical>")
            .field("addr", &format_args!("{:#010x}", self.addr))
            .finish()
    }
}

impl core::fmt::Debug for ArmAddress<Virtual> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ArmAddress<Virtual>")
            .field("addr", &format_args!("{:#010x}", self.addr))
            .finish()
    }
}

impl<A> ArmAddress<A> {
    pub const fn new(addr: usize) -> Self {
        Self {
            addr,
            _phantom: PhantomData,
        }
    }
}

impl<A> Deref for ArmAddress<A> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.addr
    }
}

impl<T> From<*const T> for ArmAddress<Virtual> {
    fn from(value: *const T) -> Self {
        Self::new(value.addr())
    }
}

impl<A> From<usize> for ArmAddress<A> {
    fn from(value: usize) -> Self {
        Self {
            addr: value,
            _phantom: PhantomData,
        }
    }
}

impl<A> From<ArmAddress<A>> for usize {
    fn from(value: ArmAddress<A>) -> Self {
        value.addr
    }
}

pub struct Physical;
pub struct Virtual;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BusAddress(pub(crate) u32);

impl core::fmt::Debug for BusAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BusAddress")
            .field(&format_args!("{:#010x}", self.0))
            .finish()
    }
}

impl BusAddress {
    pub const fn with_channel(mut self, channel: Channel) -> Self {
        BusAddress((self.0 & !0xF) | channel as u32)
    }

    /// Creates an instance of self from a const pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that this is a **physical address**, along with
    /// all the other normal Rust memory safety guarantees.
    pub unsafe fn from_ptr<T>(ptr: *const T) -> Self {
        Self((ptr as usize) as u32)
    }
}

impl Deref for BusAddress {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BusAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u32> for BusAddress {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<BusAddress> for ArmAddress<Physical> {
    fn from(val: BusAddress) -> Self {
        ArmAddress::new((val.0 & VC_BUS_ADDR) as usize)
    }
}

impl From<ArmAddress<Physical>> for BusAddress {
    fn from(val: ArmAddress<Physical>) -> Self {
        BusAddress(val.addr as u32 | !VC_BUS_ADDR)
    }
}

use core::cell::UnsafeCell;
use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroU32;
use core::pin::Pin;
use core::ptr::NonNull;

use crate::Align16;
use crate::FrameBuffer;
use crate::control_list::TileBinningControlList;
use crate::control_list::fixed::RenderControlList;
use crate::mailbox::messages::*;
use crate::mailbox::raw::Mailbox;
use crate::mailbox::raw::mailbox;
use crate::mailbox::*;
use crate::mem::volatile::VolatileRead;
use crate::mem::*;
use crate::register::*;
use crate::{Result, display::Display, mem::mapper::*};

mod error;
pub mod state;
pub use error::*;

use state::*;

pub struct Context<S, M> {
    display: Display,
    framebuffer: FrameBuffer,
    mapper: M,
    _state: S,
}

impl<S: fmt::Debug, M: fmt::Debug> fmt::Debug for Context<S, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_fmt = f.debug_struct("Context");
        debug_fmt
            .field("display", &self.display)
            .field("framebuffer", &self.framebuffer)
            .field("mapper", &self.mapper)
            .field("state", &self._state)
            .finish()
    }
}

impl<M: MemoryMapper> Context<Uninitialized, M> {
    pub const fn new(
        width: u32,
        height: u32,
        bit_depth: u32,
        mapper: M,
    ) -> Self {
        Self {
            display: Display::new(width, height, bit_depth),
            framebuffer: unsafe { FrameBuffer::empty() },
            mapper,
            _state: Uninitialized,
        }
    }
}

impl Context<Uninitialized, IdentityMapper> {
    pub const fn new_identity_mapped(
        width: u32,
        height: u32,
        bit_depth: u32,
    ) -> Self {
        Self::new(width, height, bit_depth, IdentityMapper)
    }
}

impl<M: MemoryMapper> Context<Uninitialized, M>
where
    crate::Error: From<M::Error>,
{
    pub fn initialize(
        mut self,
        num_buffers: NonZeroU32,
    ) -> Result<Context<Initialized, M>> {
        {
            let qpu_init_msg = InitQpu::message(250, false);
            self.send_mailbox_message(&qpu_init_msg)?;
        }

        let mut init_msg = InitFramebuffer::message(
            self.display.width,
            self.display.height,
            self.display.bit_depth,
            num_buffers,
        );
        self.send_mailbox_message(&init_msg)?;

        let mut buf_ptr = init_msg.buffer_ptr();
        let buf_size = init_msg.alloc_buffer.buf_size.read();
        let buf_addr = self.phys_to_virt_addr(buf_ptr)?;

        // TODO: should this be considered an error?
        // if buf_size == 0 {
        //     panic!("recieved a framebuffer size of 0 from the GPU");
        // }

        let buf_ptr = NonNull::new(*buf_addr as *mut u32)
            .ok_or(InitializeError::NullFrameBufferPtr)?;
        self.display.virt_width = init_msg.inner().virt_res.width.read();
        self.display.virt_height = init_msg.inner().virt_res.height.read();

        let framebuffer = FrameBuffer {
            ptr: buf_ptr,
            size: buf_size as usize,
            screen_size: (buf_size / num_buffers) as usize,
            num_buffers,
            curr_screen: 0,
            width: self.display.width,
            height: self.display.height,
            bit_depth: self.display.bit_depth,
        };

        Ok(Context {
            display: self.display,
            framebuffer,
            mapper: self.mapper,
            _state: Initialized,
        })
    }
}

impl<M: MemoryMapper> Context<Initialized, M>
where
    crate::Error: From<M::Error>,
{
    pub unsafe fn run_bin_control_list<'a, P>(
        &'a self,
        list: Pin<&'a TileBinningControlList<'a, P>>,
    ) -> Result<()> {
        let ptr = core::ptr::from_ref(list.get_ref());
        unsafe {
            V3D_CT0CA.write_volatile(ptr.addr() as u32);
            V3D_CT0EA.write_volatile(
                (ptr.addr() + core::mem::size_of_val(&*list)) as u32,
            );

            while V3D_BFC.read_volatile() != 0 {
                core::hint::spin_loop();
            }
        }

        Ok(())
    }

    pub unsafe fn run_render_control_list<
        'a,
        const W: usize,
        const H: usize,
    >(
        &'a self,
        list: Pin<&'a RenderControlList<W, H>>,
    ) -> Result<()> {
        let ptr = core::ptr::from_ref(list.get_ref());
        unsafe {
            V3D_CT0CA.write_volatile(ptr.addr() as u32);
            V3D_CT0EA.write_volatile(
                (ptr.addr() + core::mem::size_of_val(&*list)) as u32,
            );

            while V3D_BFC.read_volatile() != 0 {
                core::hint::spin_loop();
            }
        }

        Ok(())
    }
}

impl<M: MemoryMapper> Context<Initialized, M>
where
    crate::Error: From<M::Error>,
{
    /// Returns a pointer to the raw underlying buffer for the current screen.
    pub fn curr_screen_buffer_ptr(&mut self) -> *const u32 {
        unsafe { self.framebuffer.curr_screen_buffer().as_ptr() }
    }

    /// Returns a reference to the raw underlying buffer for the current screen.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn curr_screen_buffer(&mut self) -> &[u32] {
        unsafe { self.framebuffer.curr_screen_buffer() }
    }

    /// Returns a mutable reference to the raw underlying buffer for the current
    /// screen.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn curr_screen_buffer_mut(&mut self) -> &mut [u32] {
        unsafe { self.framebuffer.curr_screen_buffer_mut() }
    }

    /// Returns a reference to the raw underlying buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn full_buffer(&mut self) -> &[u32] {
        unsafe { self.framebuffer.full_buffer() }
    }

    /// Returns a mutable reference to the raw underlying buffer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the framebuffer is initialized and the CPU
    /// and GPU are not performing read or write operations on the buffer
    /// simultaneously. Doing so is undefined behavior.
    pub unsafe fn full_buffer_mut(&mut self) -> &mut [u32] {
        unsafe { self.framebuffer.full_buffer_mut() }
    }
}

impl<S, M: MemoryMapper> Context<S, M> {
    /// Convenience function for `self.mapper.virt_to_phys_addr`.
    fn virt_to_phys_addr(
        &self,
        addr: impl Into<ArmAddress<Virtual>>,
    ) -> Result<ArmAddress<Physical>, M::Error> {
        self.mapper.virt_to_phys_addr(addr.into())
    }

    /// Convenience function for `self.mapper.phys_to_virt_addr`.
    fn phys_to_virt_addr(
        &self,
        addr: impl Into<ArmAddress<Physical>>,
    ) -> Result<ArmAddress<Virtual>, M::Error> {
        self.mapper.phys_to_virt_addr(addr.into())
    }

    pub fn send_mailbox_message<T: MailboxChannel>(
        &mut self,
        message: &Align16<MessageBatch<T>>,
    ) -> Result<()>
    where
        crate::Error: From<<M as MemoryMapper>::Error>,
        <M as MemoryMapper>::Error: Into<crate::Error>,
    {
        let mailbox = unsafe { mailbox(self.phys_to_virt_addr(MAIL_BASE)?) };

        let msg_phys_addr = self.virt_to_phys_addr(message as *const _)?;
        let msg_channel_addr =
            (msg_phys_addr.addr & !0xF) | T::CHANNEL as u32 as usize;

        while mailbox.is_full() {
            core::hint::spin_loop();
        }

        unsafe { mailbox.write(msg_channel_addr as u32) };

        loop {
            while mailbox.is_empty() {
                core::hint::spin_loop();
            }

            if mailbox.read() == msg_channel_addr as u32 {
                return match message.status() {
                    RequestStatus::Request => Err(MailboxError::SendMessage(
                        "Message still contains a request?!",
                    )),
                    // TODO: check error response and return a more useful error
                    // here
                    RequestStatus::Error => Err(MailboxError::SendMessage(
                        "Response contains an error.",
                    )),
                    RequestStatus::Success => Ok(()),
                }
                .map_err(Into::into);
            }
        }
    }
}

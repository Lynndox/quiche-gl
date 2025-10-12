use crate::Align16;
use crate::mailbox::messages::*;
use crate::mailbox::raw::Mailbox;
use crate::mailbox::raw::mailbox;
use crate::mailbox::*;
use crate::mem::volatile::VolatileRead;
use crate::mem::*;
use crate::{Result, display::Display, mem::mapper::*};

mod state;
use state::*;

#[derive(Debug)]
pub struct Context<S, M: MemoryMapper> {
    display: Display,
    mapper: M,
    state: S,
}

impl Context<Uninitialized, IdentityMapper> {
    pub const fn new(width: u32, height: u32, bit_depth: u32) -> Self {
        Self::with_mapper(width, height, bit_depth, IdentityMapper)
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

impl<M: MemoryMapper> Context<Uninitialized, M>
where
    crate::Error: From<<M as MemoryMapper>::Error>,
{
    pub const fn with_mapper(
        width: u32,
        height: u32,
        bit_depth: u32,
        mapper: M,
    ) -> Self {
        Self {
            display: Display::new(width, height, bit_depth),
            mapper,
            state: Uninitialized,
        }
    }

    pub fn initialize(
        mut self,
        double_buffer: bool,
    ) -> Result<Context<Initialized, M>> {
        unsafe {
            {
                let qpu_init_msg = InitQpu::message(250, false);
                self.send_mailbox_message(&qpu_init_msg)?;
            }

            let mut init_msg = InitFramebuffer::message(
                self.display.width,
                self.display.height,
                self.display.bit_depth,
                double_buffer,
            );
            self.send_mailbox_message(&init_msg)?;

            let mut buf_ptr = init_msg.buffer_ptr();
            let buf_size = init_msg.alloc_buffer.buf_size.read();

            // TODO: should this be considered an error?
            //
            // if buf_size == 0 {
            //     // FIXME: i need to reorganize the error types anyway, so i
            // can't be bothered to     // write one for this
            //     // and really, i'm just going to be panicking in main at this
            // point anyway
            //
            //     panic!("recieved a framebuffer size of 0 from the GPU");
            // }

            let buf_ptr = self.phys_to_virt_addr(buf_ptr)?;

            let framebuffer = core::slice::from_raw_parts_mut(
                *buf_ptr as *mut u32,
                buf_size as usize,
            );

            self.display.virt_width = init_msg.virt_res.width.read();
            self.display.virt_height = init_msg.virt_res.height.read();

            Ok(Context {
                display: self.display,
                mapper: self.mapper,
                state: Initialized { framebuffer },
            })
        }
    }
}

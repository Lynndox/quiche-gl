#![allow(unused)]
#![cfg_attr(not(test), no_std)]

mod align;
mod consts;
mod context;
mod error;
mod framebuffer;
mod register;

pub mod control_list;
pub mod display;
pub mod mailbox;
pub mod mem;
pub mod nv;
pub mod shader;

pub use align::*;
pub use context::*;
pub use error::*;
pub use framebuffer::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub mod prelude {
    pub use super::display::*;
}

const VC_BUS_ADDR: u32 = 0x3FFFFFFF;

// pub fn bus_to_arm_addr(value: u32) -> usize {
//     (value & VC_BUS_ADDR) as usize
// }
//
// pub fn arm_to_bus_addr(value: usize) -> u32 {
//     value as u32 | !VC_BUS_ADDR
// }

#[macro_export]
macro_rules! align16 {
    ($($t:tt)*) => {
        $crate::Align16::new($($t)*)
    };
}

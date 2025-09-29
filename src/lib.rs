#![allow(unused)]
// #![no_std]

mod align;
mod consts;
mod error;
mod register;

pub mod control_list;
pub mod display;
pub mod mailbox;
pub mod primitive;
pub mod shader;

pub use align::*;
pub use error::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub mod prelude {
    pub use super::display::*;
}

const VC_BUS_ADDR: u32 = 0x3FFFFFFF;

pub fn bus_to_arm_addr(value: u32) -> usize {
    (value & VC_BUS_ADDR) as usize
}

pub fn arm_to_bus_addr(value: usize) -> u32 {
    (value as u32 | !VC_BUS_ADDR)
}

#![no_std]

mod align;
mod error;

pub mod control_list;
pub mod display;
pub mod mailbox;
pub mod shader;

pub use align::*;
pub use error::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub mod prelude {
    pub use super::display::*;
}

const QPU_BUS_ADDR: u32 = 0x3FFFFFFF;

pub fn gpu_to_arm_addr(value: u32) -> usize {
    (value & QPU_BUS_ADDR) as usize
}

pub fn arm_to_gpu_addr(value: u32) -> usize {
    (value | !QPU_BUS_ADDR) as usize
}

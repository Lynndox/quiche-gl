#![no_std]

use core::ops::Deref;

#[repr(C, align(16))]
pub struct Shader<'a>(&'a [u32]);

impl<'a> Shader<'a> {
    pub const fn new(data: &'a [u32]) -> Self {
        Self(data)
    }
}

impl Deref for Shader<'_> {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// A simple fragment shader for rendering a textured triangle(?) to get you started.
pub const TEXTURE_FRAG_SHADER: Shader<'static> = Shader::new(&[
    // Tex S: ACC0 = S * W (R15A)
    // Add Op: No Operation, Add Cond: Never
    // Mul Pipe: Floating Point Multiply, ACC0, R15, VARYING_READ, Cond: Always
    0x203E3037, //
    0x100049E0, // nop; fmul r0, ra15, vary; nop
    // Tex S Coord: ACC0 = S * W + C, Tex T: ACC1 = T * W (R15A)
    // Add Pipe: Floating Point Add, ACC0, ACC R0, ACC R5, Cond: Always
    // Mul Pipe: Floating Point Multiply, ACC1, R15, VARYING_READ, Cond: Always
    // Signal: Wait For Scoreboard
    0x213E3177, //
    0x40024821, // fadd r0, r0, r5; fmul r1, ra15, vary; sbwait
    // Tex T Write Reg = T * W + C, Trigger First Sampler Param Uniform Read
    // Add Pipe: Floating Point Add, TMU0_T, ACC R1, ACC R5, Cond: Always
    // Mul Op: No Operation, Mul Cond: Never
    0x019E7340, //
    0x10020E67, // fadd t0t, r1, r5; nop; nop
    // Moving S coord (In ACC0) To S Register, Trigger Second Sampler Param Uniform Read, & Kick It All Off
    // Add Pipe: Bitwise OR, TMU0_S_RETIRING, ACC R0, ACC R0, Cond: Always
    // Mul Op: No Operation, Mul Cond: Never
    0x159E7000, //
    0x10020E27, // mov t0s, r0; nop; nop
    // Signal TMU Texture Read
    // Add Op: No Operation, Add cond: Never
    // Mul Op: No Operation, Mul cond: Never
    // Signal: Load Data From TMU0 To R4
    0x009E7000, //
    0xA00009E7, // nop; nop; ldtmu0
    // Exporting Read Texture Data To MRT0
    // Add Pipe: Bitwise OR, TLB_COLOUR_ALL, ACC R4, ACC R4, Cond: Always
    // Mul Op: No operation, Mul Cond: Never
    // Signal: Program End
    0x159E7900, //
    0x30020BA7, // mov tlbc, r4; nop; thrend
    // Thread End Delay Slot 1
    // Add Op: No Operation, Add cond: Never
    // Mul Op: No Operation, Mul cond: Never
    0x009E7000, //
    0x100009E7, // nop; nop; nop
    // Thread End Delay Slot 2
    // Add Op: No Operation, Add cond: Never
    // Mul Op: No Operation, Mul cond: Never
    // Signal: Scoreboard Unlock
    0x009E7000, //
    0x500009E7, // nop; nop; sbdone
]);

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u32,
}

impl Display {
    pub const fn new(width: u32, height: u32, bit_depth: u32) -> Self {
        Self {
            width,
            height,
            bit_depth,
        }
    }
}

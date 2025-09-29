use core::ops::Deref;

#[repr(C, align(16))]
pub struct ShaderCode<'a>(&'a [u32]);

impl<'a> ShaderCode<'a> {
    pub const fn new(code: &'a [u32]) -> Self {
        Self(code)
    }
}

impl Deref for ShaderCode<'_> {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Simple vertex color shader
#[rustfmt::skip]
pub const VERTEX_COLOR_FRAG_SHADER: ShaderCode<'static> = ShaderCode::new(&[
    0x958E0DBF,
    0xD1724823, // mov r0, vary; mov r3.8d, 1.0
    0x818E7176,
    0x40024821, // fadd r0, r0, r5; mov r1, vary; sbwait
    0x818E7376,
    0x10024862, // fadd r1, r1, r5; mov r2, vary; nop
    0x819E7540,
    0x114248A3, // fadd r2, r2, r5; mov r3.8a, r0; nop
    0x809E7009,
    0x115049E3, // nop; mov r3.8b, r1; nop
    0x809E7012,
    0x116049E3, // nop; mov r3.8c, r2; nop
    0x159E76C0,
    0x30020BA7, // mov tlbc, r3; nop; thrend
    0x009E7000,
    0x100009E7, // nop; nop; nop
    0x009E7000,
    0x500009E7, // nop; nop; sbdone
]);

/// A simple fragment shader for rendering a textured triangle(?).
#[rustfmt::skip]
pub const TEXTURE_FRAG_SHADER: ShaderCode<'static> = ShaderCode::new(&[
    // Tex S: ACC0 = S * W (R15A)
    // Add Op: No Operation, Add Cond: Never
    // Mul Pipe: Floating Point Multiply, ACC0, R15, VARYING_READ, Cond: Always
    0x203E3037,
    0x100049E0, // nop; fmul r0, ra15, vary; nop

    // Tex S Coord: ACC0 = S * W + C, Tex T: ACC1 = T * W (R15A)
    // Add Pipe: Floating Point Add, ACC0, ACC R0, ACC R5, Cond: Always
    // Mul Pipe: Floating Point Multiply, ACC1, R15, VARYING_READ, Cond: Always
    // Signal: Wait For Scoreboard
    0x213E3177,
    0x40024821, // fadd r0, r0, r5; fmul r1, ra15, vary; sbwait

    // Tex T Write Reg = T * W + C, Trigger First Sampler Param Uniform Read
    // Add Pipe: Floating Point Add, TMU0_T, ACC R1, ACC R5, Cond: Always
    // Mul Op: No Operation, Mul Cond: Never
    0x019E7340,
    0x10020E67, // fadd t0t, r1, r5; nop; nop

    // Moving S coord (In ACC0) To S Register, Trigger Second Sampler Param Uniform Read, & Kick It All Off
    // Add Pipe: Bitwise OR, TMU0_S_RETIRING, ACC R0, ACC R0, Cond: Always
    // Mul Op: No Operation, Mul Cond: Never
    0x159E7000,
    0x10020E27, // mov t0s, r0; nop; nop

    // Signal TMU Texture Read
    // Add Op: No Operation, Add cond: Never
    // Mul Op: No Operation, Mul cond: Never
    // Signal: Load Data From TMU0 To R4
    0x009E7000,
    0xA00009E7, // nop; nop; ldtmu0

    // Exporting Read Texture Data To MRT0
    // Add Pipe: Bitwise OR, TLB_COLOUR_ALL, ACC R4, ACC R4, Cond: Always
    // Mul Op: No operation, Mul Cond: Never
    // Signal: Program End
    0x159E7900,
    0x30020BA7, // mov tlbc, r4; nop; thrend

    // Thread End Delay Slot 1
    // Add Op: No Operation, Add cond: Never
    // Mul Op: No Operation, Mul cond: Never
    0x009E7000,
    0x100009E7, // nop; nop; nop

    // Thread End Delay Slot 2
    // Add Op: No Operation, Add cond: Never
    // Mul Op: No Operation, Mul cond: Never
    // Signal: Scoreboard Unlock
    0x009E7000,
    0x500009E7, // nop; nop; sbdone

]);

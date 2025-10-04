/// A translation interface between physical and virtual address spaces.
///
/// # Safety
///
/// Implementors of this trait must ensure that the addresses returned to the caller are valid in
/// the context they are to be used in (physical vs. virtual space).
pub unsafe trait MemoryMapper {
    type Error;

    /// Translates a physical address to the virtual address space.
    fn phys_to_virt_addr(addr: usize) -> Result<usize, Self::Error>;
    /// Translates physical addresses to the MMU's kernel address space.
    fn virt_to_phys_addr(addr: usize) -> Result<usize, Self::Error>;
}

/// A dummy implementation of a [`MemoryMapper`] that returns the address as-is.
///
/// This is useful if the MMU is either inactive or the address space is identity mapped.
///
/// # Note
///
/// If any addresses that may be passed to the GPU are not identity mapped, **this implementation
/// must not be used.** An implementation that performs the correct memory address translations is
/// required in that case.
pub struct IdentityMapper;

unsafe impl MemoryMapper for IdentityMapper {
    type Error = core::convert::Infallible;

    fn phys_to_virt_addr(addr: usize) -> Result<usize, Self::Error> {
        Ok(addr)
    }

    fn virt_to_phys_addr(addr: usize) -> Result<usize, Self::Error> {
        Ok(addr)
    }
}

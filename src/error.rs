#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// overflow
    BufferTooSmall,
    /// buffer is already in use
    BufferBusy,
    /// invalid input values (validation)
    InvalidValue,
    /// Allocated vector is too small
    AllocationTooSmall(usize),
    /// Allocated vector is too large
    AllocationTooLarge(usize, usize),
}

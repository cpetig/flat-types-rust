#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// overflow
    BufferTooSmall,
    /// buffer is already in use
    BufferBusy,
    /// invalid input values (validation)
    InvalidValue,
}

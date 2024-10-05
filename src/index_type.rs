use super::Error;

pub trait IndexType {
    fn read(buffer: &[u8]) -> usize;
    fn write(buffer: &mut [u8], value: usize) -> Result<usize, Error>;
}

impl IndexType for u8 {
    fn read(buffer: &[u8]) -> usize {
        buffer[0].into()
    }

    fn write(buffer: &mut [u8], value: usize) -> Result<usize, Error> {
        let Ok(value) = u8::try_from(value) else {
            return Err(Error::InvalidValue);
        };
        match buffer.get_mut(0) {
            Some(b) => {
                *b = value;
                Ok(std::mem::size_of::<Self>())
            }
            std::option::Option::None => Err(Error::BufferTooSmall),
        }
    }
}

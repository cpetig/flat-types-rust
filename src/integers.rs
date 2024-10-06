use crate::View;
use std::fmt::Debug;

impl<'a> Debug for View<'a, u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: [u8; 4] = (&self.buffer[0..4])
            .try_into()
            .map_err(|e| std::fmt::Error)?;
        let value = u32::from_le_bytes(bytes);
        f.write_fmt(format_args!("{:?}", value))
    }
}

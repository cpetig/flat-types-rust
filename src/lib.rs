use std::marker::PhantomData;

#[derive(Copy, Clone)]
struct Slice<const N: usize, T: Copy> {
    slice: [T; N],
}

#[derive(Copy, Clone)]
struct Vec<T, IDX: Copy> {
    delta: IDX,
    length: IDX,
    phantom: PhantomData<T>,
}

#[derive(Copy, Clone)]
struct Str<const N: usize> {
    slice: [u8; N],
}

#[derive(Copy, Clone)]
struct String<IDX> {
    delta: IDX,
    length: IDX,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buffer: [u8;_] = [];
    }
}

use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Vec<T, IDX: Copy> {
    delta: IDX,
    length: IDX,
    phantom: PhantomData<T>,
}


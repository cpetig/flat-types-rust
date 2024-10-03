#[derive(Copy)]
struct Slice<T> {
    slice: [T],
};

#[derive(Copy)]
struct Vec<T> {
    delta: IDX,
    length: IDX,
};

#[derive(Copy)]
struct Str {
    slice: [u8],
};

#[derive(Copy)]
struct String<IDX> {
    delta: IDX,
    length: IDX,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buffer: [u8;_] = [];
    }
}

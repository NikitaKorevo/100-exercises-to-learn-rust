// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32<T> {
    value: T,
}

impl<T> From<T> for WrappingU32<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

fn example() {
    let wrapping: WrappingU32<u32> = 42.into();
    let wrapping: WrappingU32<i32> = WrappingU32::from(42);
}

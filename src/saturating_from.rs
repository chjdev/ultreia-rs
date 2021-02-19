pub trait SaturatingInto<T> {
    fn saturating_from(value: Self) -> T;
    fn saturating_into(&self) -> T;
}

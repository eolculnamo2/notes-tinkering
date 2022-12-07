pub trait ObjectMapping<U> {
    fn map_to(&self) -> U;
}

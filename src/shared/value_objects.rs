pub(crate) trait ObjectMother {
    type Item;
    fn build(self) -> Self::Item;
    fn init() -> Self;
}

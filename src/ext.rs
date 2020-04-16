pub(crate) trait DefaultPartialEq {
    fn is_default(&self) -> bool;
}
impl<T: Default + PartialEq> DefaultPartialEq for T {
    fn is_default(&self) -> bool {
        self.eq(&Default::default())
    }
}

pub trait ValueType {
    fn new(val: usize) -> Self;
    fn to_usize(self) -> usize;
}
use crate::vm::value_type::ValueType;

pub struct Pointer(pub usize);

impl ValueType for Pointer {
    fn new(val: usize) -> Self {
        Pointer(val)
    }

    fn to_usize(self) -> usize {
        let Pointer(val) = self;
        val
    }
}
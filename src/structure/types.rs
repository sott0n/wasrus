#[cfg(test)]
use proptest_drive::Arbitrary;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

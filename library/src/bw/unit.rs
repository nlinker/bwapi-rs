use crate::ffi;

#[derive(Debug, Clone)]
pub struct Unit { pub raw: *const ffi::UnitInterface }

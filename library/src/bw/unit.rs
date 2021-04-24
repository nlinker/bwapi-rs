use crate::ffi;

#[derive(Debug, Clone)]
pub struct Unit {
    pub raw: *const ffi::UnitInterface,
}

impl Unit {
    pub fn id(&self) -> i32 {
        unsafe { ffi::Unit_getId(self.raw) }
    }
}

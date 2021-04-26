use crate::ffi;
use crate::bw::unit_type::UnitType;
use crate::bw::position::Position;

#[derive(Debug, Clone)]
pub struct Unit {
    pub(crate) raw: *const ffi::UnitInterface,
}

impl Unit {
    pub unsafe fn from_raw(raw: *const ffi::UnitInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }

    pub fn get_id(&self) -> i32 {
        unsafe { ffi::Unit_getId(self.raw) }
    }
    pub fn get_type(&self) -> UnitType {
        unsafe { ffi::Unit_getType(self.raw) }
    }
    pub fn get_position(&self) -> Position {
        unsafe { ffi::Unit_getPosition(self.raw) }
    }
}

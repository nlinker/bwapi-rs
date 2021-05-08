use crate::bw::position::Position;
use crate::bw::unit_type::UnitType;
use crate::{ffi, FromRaw};

#[derive(Debug, Clone)]
pub struct Unit {
    pub(crate) raw: *const ffi::UnitInterface,
}

impl FromRaw<ffi::UnitInterface> for Unit {
    unsafe fn from_raw(raw: *const ffi::UnitInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }
}

impl Unit {
    pub fn get_id(&self) -> i32 {
        unsafe { (*self.raw).getID() }
    }
    pub fn get_type(&self) -> UnitType {
        unsafe { (*self.raw).getType() }
    }
    pub fn get_position(&self) -> Position {
        unsafe { (*self.raw).getPosition() }
    }
}

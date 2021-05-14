use crate::bw::position::Position;
use crate::bw::unit_type::UnitType;
use crate::{ffi, FromRaw};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Unit {
    pub(crate) raw: NonNull<ffi::UnitInterface>,
}

impl FromRaw<ffi::UnitInterface> for Unit {
    unsafe fn from_raw(raw: *mut ffi::UnitInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

impl Unit {
    pub fn get_id(&self) -> i32 {
        unsafe { self.raw.as_ref().getID() }
    }
    pub fn get_type(&self) -> UnitType {
        unsafe { self.raw.as_ref().getType() }
    }
    pub fn get_position(&self) -> Position {
        unsafe { self.raw.as_ref().getPosition() }
    }
}

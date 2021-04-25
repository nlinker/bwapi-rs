use crate::ffi;
use crate::from_raw::FromRaw;

#[derive(Debug, Clone)]
pub struct Player {
    raw: *const ffi::PlayerInterface,
}

impl FromRaw for Player {
    unsafe fn from_raw(raw: *const ffi::c_void) -> Self {
        assert!(!raw.is_null());
        // Self::from_raw(raw as *const ffi::PlayerInterface)
        Self { raw: raw as *const ffi::PlayerInterface }
    }
}

impl Player {
    // pub unsafe fn from_raw(raw: *const ffi::PlayerInterface) -> Self {
    //     assert!(!raw.is_null());
    //     Self { raw }
    // }
}
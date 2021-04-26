use crate::ffi;

#[derive(Debug, Clone)]
pub struct Player {
    raw: *const ffi::PlayerInterface,
}

impl Player {
    pub unsafe fn from_raw(raw: *const ffi::PlayerInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }
}
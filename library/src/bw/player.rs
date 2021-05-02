use crate::{ffi, FromRaw};

#[derive(Debug, Clone)]
pub struct Player {
    raw: *const ffi::PlayerInterface,
}

impl FromRaw<ffi::PlayerInterface> for Player {
    unsafe fn from_raw(raw: *const ffi::PlayerInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }
}

use crate::{ffi, FromRaw};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) raw: NonNull<ffi::PlayerInterface>,
}

impl FromRaw<ffi::PlayerInterface> for Player {
    unsafe fn from_raw(raw: *mut ffi::PlayerInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

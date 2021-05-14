use crate::{ffi, FromRaw};
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Region {
    pub raw: NonNull<ffi::RegionInterface>,
}

impl FromRaw<ffi::RegionInterface> for Region {
    unsafe fn from_raw(raw: *mut ffi::RegionInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

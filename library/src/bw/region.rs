use crate::{ffi, FromRaw};

#[derive(Debug)]
pub struct Region {
    pub raw: *const ffi::RegionInterface,
}

impl FromRaw<ffi::RegionInterface> for Region {
    unsafe fn from_raw(raw: *const ffi::RegionInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }
}

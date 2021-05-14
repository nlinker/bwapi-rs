use crate::bw::playerset::Playerset;
use crate::bw::Handle;
use crate::{ffi, FromRaw};
use cxx::UniquePtr;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Force {
    pub raw: NonNull<ffi::ForceInterface>,
}

impl FromRaw<ffi::ForceInterface> for Force {
    unsafe fn from_raw(raw: *mut ffi::ForceInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

impl Force {
    pub fn get_id(&self) -> i32 {
        unsafe { self.raw.as_ref().getID() }
    }
    pub fn get_name(&self) -> String {
        let f: &ffi::ForceInterface = unsafe { self.raw.as_ref() };
        ffi::_force_getName(f).to_string()
    }
    pub fn get_players(&self) -> Playerset {
        let f: &ffi::ForceInterface = unsafe { self.raw.as_ref() };
        let set: UniquePtr<ffi::Playerset> = ffi::_force_getPlayers(f);
        Playerset {
            raw: Handle::Owned(set),
        }
    }
}

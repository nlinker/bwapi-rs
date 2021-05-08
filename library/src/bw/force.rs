use crate::{ffi, FromRaw};
use crate::bw::playerset::Playerset;
use cxx::UniquePtr;
use crate::bw::Handle;

#[derive(Debug)]
pub struct Force {
    pub raw: *const ffi::ForceInterface,
}

impl FromRaw<ffi::ForceInterface> for Force {
    unsafe fn from_raw(raw: *const ffi::ForceInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }
}

impl Force {
    pub fn get_id(&self) -> i32 {
        unsafe { (&*self.raw).getID() }
    }
    pub fn get_name(&self) -> String {
        let f: &ffi::ForceInterface = unsafe { &*self.raw };
        ffi::_force_getName(f).to_string()
    }
    pub fn get_players(&self) -> Playerset {
        let f: &ffi::ForceInterface = unsafe { &*self.raw };
        let set: UniquePtr<ffi::Playerset> = ffi::_force_getPlayers(f);
        Playerset { raw: Handle::Owned(set) }
    }
}
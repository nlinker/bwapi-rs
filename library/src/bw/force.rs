use crate::ffi;

#[derive(Debug)]
pub struct Force {
    pub raw: *mut ffi::ForceInterface,
}

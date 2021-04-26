use crate::ffi::c_void;

/// `FromRaw` is a trait for entities that
/// are typically created outside of Rust code.
pub trait FromRaw {
    unsafe fn from_raw(raw: *const c_void) -> Self;
}

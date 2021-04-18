use crate::ffi;

#[derive(Debug, Clone)]
pub struct Player {
    pub raw: *const ffi::PlayerInterface,
}

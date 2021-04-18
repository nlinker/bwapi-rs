use crate::bw::game::Game;
use once_cell::sync::Lazy;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex};

pub mod ai_module;
pub mod game;
pub mod player;
pub mod position;
pub mod unit;
pub mod unit_type;

/// Updated on gameInit call
pub static GAME: Lazy<Arc<Mutex<Game>>> = Lazy::new(|| Arc::new(Mutex::new(Game { raw: null_mut() })));

pub fn bwapi_get_revision() -> i32 {
    // don't need the unsafe block
    crate::ffi::BWAPI_getRevision()
}

pub fn bwapi_is_debug() -> bool {
    // don't need the unsafe block
    crate::ffi::BWAPI_isDebug()
}

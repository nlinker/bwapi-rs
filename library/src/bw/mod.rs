use crate::bw::game::Game;
use std::sync::{Mutex, Arc};
use std::ptr::null;
use once_cell::sync::Lazy;

pub mod ai_module;
pub mod game;
pub mod player;
pub mod position;
pub mod unit;

/// Updated on gameInit call
pub static GAME: Lazy<Arc<Mutex<Game>>> = Lazy::new(||
    Arc::new(Mutex::new(Game { raw: null() }))
);

pub fn bwapi_get_revision() -> i32 {
    // don't need the unsafe block
    crate::ffi::BWAPI_getRevision()
}

pub fn bwapi_is_debug() -> bool {
    // don't need the unsafe block
    crate::ffi::BWAPI_isDebug()
}

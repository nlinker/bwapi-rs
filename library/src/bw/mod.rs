pub mod ai_module;
pub mod game;
pub mod unit;
pub mod player;
pub mod position;

use game::Game;
use once_cell::sync::Lazy;
use std::ptr::null;

static mut _BW_UNSAFE: *const Game = null();
static _BW: Lazy<&Game> = Lazy::new(|| unsafe { &*_BW_UNSAFE });

pub fn bwapi_get_revision() -> i32 {
    // don't need the unsafe block
    crate::ffi::BWAPI_getRevision()
}

pub fn bwapi_is_debug() -> bool {
    // don't need the unsafe block
    crate::ffi::BWAPI_isDebug()
}
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::bw::game::Game;

pub mod ai_module;
pub mod force;
pub mod forceset;
pub mod game;
pub mod iterator;
pub mod player;
pub mod playerset;
pub mod position;
pub mod tech_type;
pub mod unit;
pub mod unit_command;
pub mod unit_filter;
pub mod unit_type;
#[allow(unused_variables)]
pub mod unitset;

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

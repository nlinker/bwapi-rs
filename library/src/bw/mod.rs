use crate::bw::game::Game;
use once_cell::sync::Lazy;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use cxx::UniquePtr;

pub mod ai_module;
pub mod color;
pub mod coordinate_type;
pub mod force;
pub mod forceset;
pub mod game;
pub mod game_type;
pub mod input;
pub mod player;
pub mod player_type;
pub mod playerset;
pub mod position;
pub mod race;
pub mod tech_type;
pub mod unit;
pub mod unit_command;
pub mod unit_filter;
pub mod unit_type;
pub mod unitset;
pub mod upgrade_type;
pub mod weapon_type;

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

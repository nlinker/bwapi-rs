use game::Game;

use crate::bw::ai_module::AIModule;

pub mod ai_module;
pub mod game;
pub mod unit;
pub mod player;
pub mod position;

static BW: Game = unreachable!();

pub fn register_ai_module<T: AIModule>(create_ai: impl FnOnce() -> T) {

}

pub fn bwapi_get_revision() -> i32 {
    // don't need the unsafe block
    crate::ffi::root::BWAPI_getRevision()
}

pub fn bwapi_is_debug() -> bool {
    // don't need the unsafe block
    crate::ffi::root::BWAPI_getRevision()
}
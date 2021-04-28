use crate::ffi;
use crate::bw::unitset::Unitset;
use crate::bw::position::Position;
use crate::bw::unit_filter::UnitFilter;
use std::pin::Pin;
use crate::bw::playerset::Playerset;

#[derive(Debug)]
pub struct Game {
    pub raw: *mut ffi::Game,
}

/// Game object doesn't contain any self-references
impl Unpin for Game {}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn debug(&self) {
        unsafe { ffi::_game_debug(&*self.raw) };
    }

    pub fn allies(&self) -> Playerset {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        let set: Pin<&mut ffi::Playerset> = game.allies();
        Playerset { iter: ffi::createPlayersetIteratorRef(&*set) }
    }
    pub fn send_text(&self, text: &str) {
        ffi::_game_sendText(unsafe { Pin::new_unchecked(&mut *self.raw) }, text)
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
    }
    pub fn get_all_units(&self) -> Unitset {
        let game: &ffi::Game = unsafe { &*self.raw };
        let set: &ffi::Unitset = game.getAllUnits();
        Unitset { iter: ffi::createUnitsetIteratorRef(set) }
    }
    pub fn get_units_in_radius(&self, position: Position, radius: i32, pred: UnitFilter) -> Unitset {
        let game: &ffi::Game = unsafe { &*self.raw };
        let iter = ffi::_game_getUnitsInRadius(game, position, radius, pred);
        Unitset { iter }
    }
}

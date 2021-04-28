use crate::ffi;
use cxx::UniquePtr;
use crate::bw::unitset::Unitset;
use crate::bw::position::Position;
use crate::bw::unit_filter::UnitFilter;
use std::pin::Pin;

#[derive(Debug)]
pub struct Game {
    pub raw: *mut ffi::Game,
}

/// Game object doesn't contain self-references
impl Unpin for Game {}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn debug(&self) {
        unsafe { ffi::_game_debug(&*self.raw) };
    }

    pub fn allies(&self) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        let set: Pin<&mut ffi::Playerset> = game.allies();
        println!("game.allies() = {:p}", &*set);
    }

    pub fn send_text(&self, text: &str) {
        ffi::_game_sendText(unsafe { Pin::new_unchecked(&mut *self.raw) }, text)
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
    }
    pub fn get_all_units(&self) -> Unitset {
        let iter: UniquePtr<ffi::UnitsetIterator> = unsafe { ffi::_game_getAllUnits(&*self.raw) };
        Unitset { iter }
    }
    pub fn get_units_in_radius(&self, position: Position, radius: i32, pred: UnitFilter) -> Unitset {
        let iter: UniquePtr<ffi::UnitsetIterator> = unsafe { ffi::_game_getUnitsInRadius(&*self.raw, position, radius, pred) };
        Unitset { iter }
    }
}

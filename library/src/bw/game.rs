use crate::ffi;
use cxx::UniquePtr;
use crate::bw::unitset::Unitset;
use crate::bw::position::Position;
use crate::bw::unit_filter::UnitFilter;

#[derive(Debug)]
pub struct Game {
    pub raw: *mut ffi::Game,
}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn debug(&self) {
        unsafe { ffi::Game_debug(self.raw) };
    }
    pub fn send_text(&self, text: &str) {
        unsafe { ffi::sendText(self.raw, text) }
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
    }
    pub fn get_all_units(&self) -> Unitset {

        let iter: UniquePtr<ffi::UnitsetIterator> = unsafe { ffi::getAllUnits(self.raw) };
        Unitset { iter }
    }
    pub fn get_units_in_radius(&self, position: Position, radius: i32, pred: UnitFilter) -> Unitset {
        let iter: UniquePtr<ffi::UnitsetIterator> = unsafe { ffi::getUnitsInRadius_Game(self.raw, position, radius, pred) };
        Unitset { iter }
    }
}

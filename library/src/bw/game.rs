use crate::ffi;
use crate::bw::force::Force;
use std::marker::PhantomData;
use crate::bw::unit::Unit;

#[derive(Debug)]
pub struct Game {
    pub raw: *mut ffi::Game,
}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn send_text(&self, text: &str) {
        unsafe { ffi::Game_sendText(self.raw, text) }
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
        // unsafe { ffi::getFrameCount(self.raw) }
    }
    // pub fn get_forces(&self) -> Vec<Force> {
    //     let force_set: &ffi::Forceset = unsafe { (*self.raw).getForces() };
    //     vec![]
    // }
    pub fn get_all_units(&self) -> UnitIterator {
        let unit_set: &ffi::Unitset = unsafe { (*self.raw).getAllUnits() };
        UnitIterator::new(unit_set)
    }

}

pub struct UnitIterator {
    raw: *const ffi::Unitset,
}

impl UnitIterator {
    fn new(r: &ffi::Unitset) -> Self {
        Self {
            raw: r as *const ffi::Unitset,
        }
    }
}

impl Iterator for UnitIterator {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

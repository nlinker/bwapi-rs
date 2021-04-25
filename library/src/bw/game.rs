use crate::ffi;
use crate::bw::unit::Unit;
use cxx::UniquePtr;
use std::ptr::null;
use crate::from_raw::FromRaw;

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
    // pub fn get_forces(&self) -> Vec<Force> {
    //     let force_set: &ffi::Forceset = unsafe { (*self.raw).getForces() };
    //     vec![]
    // }
    pub fn get_all_units(&self) -> UnitRefIterator {
        let unit_set: UniquePtr<ffi::IteratorBase> = unsafe { ffi::getAllUnits(self.raw) };
        UnitRefIterator(unit_set)
    }
}

pub struct UnitRefIterator(UniquePtr<ffi::IteratorBase>);

impl Iterator for UnitRefIterator {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = unsafe { self.0.pin_mut().next() };
        if raw != null() {
            // println!("{:p}", raw);
            Some(unsafe { Unit::from_raw(raw as *const std::ffi::c_void) })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = unsafe { self.0.size() };
        (lb, None)
    }
}

// pub struct UnitIterator {
//     raw: *const ffi::UnitIterator,
// }
//
// impl UnitIterator {
//     fn new(r: &ffi::Unitset) -> Self {
//         let raw = unsafe { ffi::UnitIterator_begin(r) as *const _ };
//         Self { raw }
//     }
// }
//
// impl Iterator for ffi::UnitIterator {
//     type Item = Unit;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         unsafe { ffi::UnitIterator_next(self.raw) }
//             .map(|raw| Unit { raw })
//     }
//
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let lower_bound = unsafe { ffi::UnitIterator_size(self.raw) };
//         (lower_bound, None)
//     }
// }

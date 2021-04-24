use crate::ffi;
use crate::bw::force::Force;
use std::marker::PhantomData;
use crate::bw::unit::Unit;
use cxx::CxxVector;
use crate::ffi::Unit_getId;

#[derive(Debug)]
pub struct Game {
    pub raw: *mut ffi::Game,
}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn send_text(&self, text: &str) {
        unsafe { ffi::sendText(self.raw, text) }
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
        // unsafe { ffi::getFrameCount(self.raw) }
    }
    // pub fn get_forces(&self) -> Vec<Force> {
    //     let force_set: &ffi::Forceset = unsafe { (*self.raw).getForces() };
    //     vec![]
    // }
    pub fn get_all_units(&self) -> Vec<Unit> {
        let unit_set: &ffi::Unitset = unsafe { (*self.raw).getAllUnits() };
        let xs: &CxxVector<ffi::Unit> = ffi::Game_getAllUnits(unit_set);
        for i in 0..xs.len() {
            println!("{:?}", xs.get(i).map(|u| Unit_getId(&u)));
        }
        // UnitIterator::new(unit_set)

        vec![]
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

use crate::ffi;
use crate::bw::force::Force;

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
    pub fn get_forces(self: &Game) -> Vec<Force> {
        let force_set: &ffi::Forceset = unsafe { (*self.raw).getForces() };
        vec![]
    }
}

pub struct ForceIterator {
    pub raw: *mut ffi::Forceset
}

// impl ForceIterator {
//     fn new(r: &ffi::Forceset) -> Self {
//         Self {
//             raw: r as *mut ffi::Forceset
//         }
//     }
// }

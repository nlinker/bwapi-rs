use crate::ffi;
use std::ffi::CString;

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
}

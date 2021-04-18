use crate::ffi;
use std::ffi::{CString, NulError};

#[derive(Debug)]
pub struct Game {
    pub raw: *const ffi::Game,
}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn send_text(&self, text: &str) {
        match CString::new(text) {
            Ok(s) => unsafe { ffi::sendText(self.raw as *mut _, s.as_ptr()) },
            Err(e) => println!("Warning: {}", e),
        }
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { ffi::getFrameCount(self.raw as *mut _) }
    }

}
use crate::ffi;

#[derive(Debug)]
pub struct Game {
    pub raw: *const ffi::Game,
}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn send_text(&self, text: &str) {
        unsafe { ffi::send_text(self.raw as *mut _, text) }
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { ffi::get_frame_count(self.raw as *mut _) }
    }

}
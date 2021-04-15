use crate::ffi;

pub struct Game {
    pub raw: *const ffi::Game,
}

impl Game {
    pub fn send_text(&self, _text: &str) {
        // self.raw;
    }
    pub fn get_frame_count(&self) -> i32 {
        0
    }

}
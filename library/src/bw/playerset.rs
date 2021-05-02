use crate::bw::player::Player;
use crate::ffi;
use cxx::UniquePtr;
use std::ptr::null;

pub struct Playerset {
    pub(crate) iter: UniquePtr<ffi::PlayersetIterator>,
}

// impl Iterator for Playerset {
//     type Item = Player;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let raw: *const ffi::PlayerInterface = self.iter.pin_mut().next();
//         if raw != null() {
//             Some(unsafe { Player::from_raw(raw) })
//         } else {
//             None
//         }
//     }
//
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let lb = self.iter.sizeHint();
//         (lb, None)
//     }
// }

use crate::bw::player::Player;
use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi;
use cxx::UniquePtr;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::null;

pub struct Playerset<'a> {
    pub(crate) raw: Handle<'a, ffi::Playerset>,
}

impl ForeignIterator for ffi::PlayersetIterator {
    type ForeignItem = ffi::PlayerInterface;
    type TargetItem = Player;
    fn next(self: Pin<&mut Self>) -> *const Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Playerset<'a> {
    type Item = Player;
    type IntoIter = ForeignIter<'a, ffi::PlayersetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createPlayersetIterator(self.raw.underlying());
        ForeignIter { iter, marker: PhantomData }
    }
}

use crate::bw::player::Player;
use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Playerset<'a> {
    pub(crate) raw: Handle<'a, ffi::Playerset>,
}

impl fmt::Debug for Playerset<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.raw {
            Handle::Owned(u) => write!(f, "Playerset(Owned({:p}))", u),
            Handle::Borrowed(r) => write!(f, "Playerset(Borrowed({:p}))", r),
            Handle::BorrowedMut(p) => write!(f, "Playerset(BorrowedMut({:p}))", p),
        }
    }
}

impl ForeignIterator for ffi::PlayersetIterator {
    type ForeignItem = ffi::PlayerInterface;
    fn next(self: Pin<&mut Self>) -> *mut Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Playerset<'a> {
    type Item = Player;
    type IntoIter = ForeignIter<'a, Self::Item, ffi::PlayersetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createPlayersetIterator(self.raw.underlying());
        ForeignIter {
            iter,
            marker: PhantomData,
        }
    }
}

impl Playerset<'_> {
    pub fn iter(&self) -> ForeignIter<'_, Player, ffi::PlayersetIterator> {
        self.into_iter()
    }
    pub fn is_empty(&self) -> bool { self.iter().size_hint().0 == 0 }
    pub fn len(&self) -> usize {
        self.iter().size_hint().0
    }

    // Race::set 	getRaces () const
    // Unitset 	getUnits () const
    // void 	setAlliance (bool allies=true, bool alliedVictory=true)
}

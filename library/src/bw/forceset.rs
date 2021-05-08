use crate::bw::force::Force;
use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi;
use std::marker::PhantomData;
use std::pin::Pin;
use std::fmt;
use crate::bw::playerset::Playerset;

pub struct Forceset<'a> {
    pub(crate) raw: Handle<'a, ffi::Forceset>,
}

impl fmt::Debug for Forceset<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.raw {
            Handle::Owned(u) => write!(f, "Forceset(Owned({:p}))", u),
            Handle::Borrowed(r) => write!(f, "Forceset(Borrowed({:p}))", r),
            Handle::BorrowedMut(p) => write!(f, "Forceset(BorrowedMut({:p}))", p),
        }
    }
}

impl ForeignIterator for ffi::ForcesetIterator {
    type ForeignItem = ffi::ForceInterface;
    type TargetItem = Force;
    fn next(self: Pin<&mut Self>) -> *const Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Forceset<'a> {
    type Item = Force;
    type IntoIter = ForeignIter<'a, ffi::ForcesetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createForcesetIterator(self.raw.underlying());
        ForeignIter { iter, marker: PhantomData }
    }
}

impl Forceset<'_> {
    pub fn iter(&self) -> ForeignIter<'_, ffi::ForcesetIterator> {
        self.into_iter()
    }
    pub fn len(&self) -> usize {
        self.iter().size_hint().0
    }

    pub fn get_players(&self) -> Playerset {
        let xs: &ffi::Forceset = self.raw.underlying();
        Playerset { raw: Handle::Owned(ffi::_forceset_getPlayers(xs)) }
    }
}
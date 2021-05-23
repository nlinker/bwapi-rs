use crate::bw::force::Force;
use crate::bw::playerset::Playerset;
use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;

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
    fn next(self: Pin<&mut Self>) -> *mut Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Forceset<'a> {
    type Item = Force;
    type IntoIter = ForeignIter<'a, Self::Item, ffi::ForcesetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createForcesetIterator(self.raw.underlying());
        ForeignIter {
            iter,
            marker: PhantomData,
        }
    }
}

impl Forceset<'_> {
    pub fn iter(&self) -> ForeignIter<'_, Force, ffi::ForcesetIterator> {
        self.into_iter()
    }
    pub fn is_empty(&self) -> bool { self.iter().size_hint().0 == 0 }
    pub fn len(&self) -> usize {
        self.iter().size_hint().0
    }

    pub fn get_players(&self) -> Playerset {
        let xs: &ffi::Forceset = self.raw.underlying();
        Playerset {
            raw: Handle::Owned(ffi::_forceset_getPlayers(xs)),
        }
    }
}

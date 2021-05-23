use crate::bw::region::Region;
use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Regionset<'a> {
    pub(crate) raw: Handle<'a, ffi::Regionset>,
}

impl fmt::Debug for Regionset<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.raw {
            Handle::Owned(u) => write!(f, "Regionset(Owned({:p}))", u),
            Handle::Borrowed(r) => write!(f, "Regionset(Borrowed({:p}))", r),
            Handle::BorrowedMut(p) => write!(f, "Regionset(BorrowedMut({:p}))", p),
        }
    }
}

impl ForeignIterator for ffi::RegionsetIterator {
    type ForeignItem = ffi::RegionInterface;
    fn next(self: Pin<&mut Self>) -> *mut Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Regionset<'a> {
    type Item = Region;
    type IntoIter = ForeignIter<'a, Self::Item, ffi::RegionsetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createRegionsetIterator(self.raw.underlying());
        ForeignIter {
            iter,
            marker: PhantomData,
        }
    }
}

impl Regionset<'_> {
    pub fn iter(&self) -> ForeignIter<'_, Region, ffi::RegionsetIterator> {
        self.into_iter()
    }
    pub fn is_empty(&self) -> bool { self.iter().size_hint().0 == 0 }
    pub fn len(&self) -> usize {
        self.iter().size_hint().0
    }
}

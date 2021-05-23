use crate::bw::bullet::Bullet;
use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::pin::Pin;

pub struct Bulletset<'a> {
    pub(crate) raw: Handle<'a, ffi::Bulletset>,
}

impl fmt::Debug for Bulletset<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.raw {
            Handle::Owned(u) => write!(f, "Bulletset(Owned({:p}))", u),
            Handle::Borrowed(r) => write!(f, "Bulletset(Borrowed({:p}))", r),
            Handle::BorrowedMut(p) => write!(f, "Bulletset(BorrowedMut({:p}))", p),
        }
    }
}

impl ForeignIterator for ffi::BulletsetIterator {
    type ForeignItem = ffi::BulletInterface;
    fn next(self: Pin<&mut Self>) -> *mut Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Bulletset<'a> {
    type Item = Bullet;
    type IntoIter = ForeignIter<'a, Self::Item, ffi::BulletsetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createBulletsetIterator(self.raw.underlying());
        ForeignIter {
            iter,
            marker: PhantomData,
        }
    }
}

impl Bulletset<'_> {
    pub fn iter(&self) -> ForeignIter<'_, Bullet, ffi::BulletsetIterator> {
        self.into_iter()
    }
    pub fn is_empty(&self) -> bool { self.iter().size_hint().0 == 0 }
    pub fn len(&self) -> usize {
        self.iter().size_hint().0
    }
}

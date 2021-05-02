use crate::bw::game::Game;
use once_cell::sync::Lazy;
use std::ptr::{null_mut, null};
use std::sync::{Arc, Mutex};
use cxx::UniquePtr;
use cxx::memory::UniquePtrTarget;
use std::marker::PhantomData;
use crate::{ffi, FromRaw};
use std::pin::Pin;

pub mod ai_module;
pub mod color;
pub mod coordinate_type;
pub mod force;
pub mod forceset;
pub mod game;
pub mod game_type;
pub mod input;
pub mod player;
pub mod player_type;
pub mod playerset;
pub mod position;
pub mod race;
pub mod tech_type;
pub mod unit;
pub mod unit_command;
pub mod unit_filter;
pub mod unit_type;
pub mod unitset;
pub mod upgrade_type;
pub mod weapon_type;

/// Updated on gameInit call
pub static GAME: Lazy<Arc<Mutex<Game>>> = Lazy::new(|| Arc::new(Mutex::new(Game { raw: null_mut() })));

pub trait ForeignIterator {
    type ForeignItem;
    fn next(self: Pin<&mut Self>) -> *const Self::ForeignItem;
    fn size_hint(&self) -> (usize, Option<usize>);
}

/// `FC` - foreign collection
pub enum Handle<'a, FC: UniquePtrTarget> {
    Own(UniquePtr<FC>),
    Ref(&'a FC),
}

/// `FI` - foreign iterator
pub struct ForeignIter<'a, FI: ForeignIterator + UniquePtrTarget, T: FromRaw<FI::ForeignItem>> {
    pub(crate) iter: UniquePtr<FI>,
    marker: PhantomData<&'a T>,
}

impl<'a, FI: ForeignIterator + UniquePtrTarget, T: FromRaw<FI::ForeignItem>> Iterator for ForeignIter<'a, FI, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //let it: Pin<&mut FI> = ;
        let raw = self.iter.pin_mut().next();
        if raw != null() {
            Some(unsafe { T::from_raw(raw) })
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn bwapi_get_revision() -> i32 {
    // don't need the unsafe block
    crate::ffi::BWAPI_getRevision()
}

pub fn bwapi_is_debug() -> bool {
    // don't need the unsafe block
    crate::ffi::BWAPI_isDebug()
}

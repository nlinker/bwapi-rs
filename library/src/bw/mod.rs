use crate::bw::game::Game;
use crate::FromRaw;
use cxx::memory::UniquePtrTarget;
use cxx::UniquePtr;
use once_cell::sync::Lazy;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex};

pub mod client;

pub mod ai_module;
pub mod bullet;
pub mod bullet_type;
pub mod color;
pub mod coordinate_type;
pub mod damage_type;
pub mod error;
pub mod explosion_type;
pub mod flag;
pub mod force;
pub mod forceset;
pub mod game;
pub mod game_type;
pub mod input;
pub mod order;
pub mod player;
pub mod player_type;
pub mod playerset;
pub mod position;
pub mod race;
pub mod tech_type;
pub mod unit;
pub mod unit_command;
pub mod unit_filter;
pub mod unit_size_type;
pub mod unit_type;
pub mod unitset;
pub mod upgrade_type;
pub mod weapon_type;

/// Updated on gameInit call
pub static GAME: Lazy<Arc<Mutex<Game>>> = Lazy::new(|| Arc::new(Mutex::new(Game { raw: null_mut() })));

/// `FC` - foreign collection like `ffi::Unitset` or `ffi::Playerset`
#[derive(Debug)]
pub enum Handle<'a, FC: UniquePtrTarget> {
    Owned(UniquePtr<FC>),
    Borrowed(&'a FC),
    BorrowedMut(Pin<&'a mut FC>),
}

impl<'a, FC: UniquePtrTarget> Handle<'a, FC> {
    pub fn underlying(&self) -> &FC {
        match &self {
            Handle::Owned(p) => p.deref(),
            Handle::Borrowed(r) => r.deref(),
            Handle::BorrowedMut(r) => r.deref(),
        }
    }
}

pub trait ForeignIterator {
    type ForeignItem;
    type TargetItem: FromRaw<Self::ForeignItem>;
    fn next(self: Pin<&mut Self>) -> *const Self::ForeignItem;
    fn size_hint(&self) -> (usize, Option<usize>);
}

/// `FI` - foreign iterator
pub struct ForeignIter<'a, FI: ForeignIterator + UniquePtrTarget> {
    pub(crate) iter: UniquePtr<FI>,
    marker: PhantomData<&'a FI>,
}

impl<'a, FI: ForeignIterator + UniquePtrTarget> Iterator for ForeignIter<'a, FI> {
    type Item = FI::TargetItem;
    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.iter.pin_mut().next();
        if raw != null() {
            Some(unsafe { Self::Item::from_raw(raw) })
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

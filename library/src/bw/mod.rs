use crate::bw::game::Game;
use crate::bw::unit::Unit;
use crate::{ffi, FromRaw};
use cxx::memory::UniquePtrTarget;
use cxx::UniquePtr;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

pub mod client;

pub mod ai_module;
pub mod bullet;
pub mod bullet_type;
pub mod bulletset;
pub mod can_do;
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
pub mod region;
pub mod regionset;
pub mod tech_type;
pub mod unit;
pub mod unit_command;
pub mod unit_filter;
pub mod unit_size_type;
pub mod unit_type;
pub mod unitset;
pub mod upgrade_type;
pub mod weapon_type;
pub mod shared;

/// Updated on gameInit call
pub static GAME: Lazy<Arc<Mutex<Game>>> = Lazy::new(|| Arc::new(Mutex::new(Game { raw: None })));

thread_local! {
    static UNIT_FILTERS: RefCell<VecDeque<Box<dyn Fn(Unit) -> bool>>> = RefCell::new(VecDeque::new());
    static CMP_UNIT_FILTERS: RefCell<VecDeque<Box<dyn Fn(Unit) -> i32>>> = RefCell::new(VecDeque::new());
    static BEST_UNIT_FILTERS: RefCell<VecDeque<Box<dyn Fn(Unit, Unit) -> Unit>>> = RefCell::new(VecDeque::new());
}

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
    fn next(self: Pin<&mut Self>) -> *mut Self::ForeignItem;
    fn size_hint(&self) -> (usize, Option<usize>);
}

/// `FI` - foreign iterator
pub struct ForeignIter<'a, T, FI>
where
    T: FromRaw<FI::ForeignItem>,
    FI: ForeignIterator + UniquePtrTarget,
{
    pub(crate) iter: UniquePtr<FI>,
    marker: PhantomData<&'a T>,
}

impl<'a, T, FI> Iterator for ForeignIter<'a, T, FI>
where
    T: FromRaw<FI::ForeignItem>,
    FI: ForeignIterator + UniquePtrTarget,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let raw = self.iter.pin_mut().next();
        if raw != null_mut() {
            Some(unsafe { Self::Item::from_raw(raw) })
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub fn with_unit_filter<F, R, FFI>(f: F, ffi_call: FFI) -> R
where
    F: Fn(Unit) -> bool + 'static,
    FFI: FnOnce(fn(x: *mut ffi::UnitInterface) -> bool) -> R,
{
    UNIT_FILTERS.with(|ufs| {
        ufs.borrow_mut().push_back(Box::new(f));
    });
    //-----------------------------------
    let ret = ffi_call(unit_filter::<F>);
    //-----------------------------------
    UNIT_FILTERS.with(|ufs| {
        let _ = ufs.borrow_mut().pop_back();
    });
    ret
}

pub fn with_unit_and_best_filter<F, G, FFI, R>(f: F, g: G, ffi_call: FFI) -> R
where
    F: Fn(Unit) -> bool + 'static,
    G: Fn(Unit, Unit) -> Unit + 'static,
    FFI: FnOnce(
        fn(*mut ffi::UnitInterface) -> bool,
        fn(*mut ffi::UnitInterface, *mut ffi::UnitInterface) -> *mut ffi::UnitInterface,
    ) -> R,
{
    UNIT_FILTERS.with(|ufs| {
        ufs.borrow_mut().push_back(Box::new(f));
    });
    BEST_UNIT_FILTERS.with(|bufs| {
        bufs.borrow_mut().push_back(Box::new(g));
    });
    //----------------------------------------------------------
    let ret = ffi_call(unit_filter::<F>, best_unit_filter::<G>);
    //----------------------------------------------------------
    BEST_UNIT_FILTERS.with(|bufs| {
        let _ = bufs.borrow_mut().pop_back();
    });
    UNIT_FILTERS.with(|ufs| {
        let _ = ufs.borrow_mut().pop_back();
    });
    ret
}

fn unit_filter<F: Fn(Unit) -> bool + 'static>(x: *mut ffi::UnitInterface) -> bool {
    UNIT_FILTERS.with(|funcs| {
        if let Some(f) = funcs.borrow().back() {
            let unit = unsafe { Unit::from_raw(x) };
            f(unit)
        } else {
            unreachable!("Impossible: function stack is empty")
        }
    })
}

fn best_unit_filter<F: Fn(Unit, Unit) -> Unit + 'static>(
    x: *mut ffi::UnitInterface,
    y: *mut ffi::UnitInterface,
) -> *mut ffi::UnitInterface {
    BEST_UNIT_FILTERS.with(|funcs| {
        if let Some(f) = funcs.borrow().back() {
            let u1 = unsafe { Unit::from_raw(x) };
            let u2 = unsafe { Unit::from_raw(y) };
            let u: Unit = f(u1, u2);
            u.raw.as_ptr()
        } else {
            unreachable!("Impossible: function stack is empty")
        }
    })
}

pub fn bwapi_get_revision() -> i32 {
    // don't need the unsafe block
    crate::ffi::BWAPI_getRevision()
}

pub fn bwapi_is_debug() -> bool {
    // don't need the unsafe block
    crate::ffi::BWAPI_isDebug()
}

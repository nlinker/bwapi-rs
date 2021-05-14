use crate::{ffi, FromRaw};
use std::ptr::NonNull;
use crate::bw::unit::Unit;
use crate::bw::unitset::Unitset;
use crate::bw::position::Position;
use crate::bw::regionset::Regionset;
use crate::bw::{with_unit_filter, Handle};

#[derive(Debug)]
pub struct Region {
    pub(crate) raw: NonNull<ffi::RegionInterface>,
}

impl FromRaw<ffi::RegionInterface> for Region {
    unsafe fn from_raw(raw: *mut ffi::RegionInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

impl Region {
    pub fn get_bounds_bottom(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getBoundsBottom()
    }
    pub fn get_bounds_left(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getBoundsLeft()
    }
    pub fn get_bounds_right(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getBoundsRight()
    }
    pub fn get_bounds_top(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getBoundsTop()
    }
    pub fn get_center(&self) -> Position {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getCenter()
    }
    pub fn get_closest_accessible_region(&self) -> Region {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        unsafe { Region::from_raw(r.getClosestAccessibleRegion()) }
    }
    pub fn get_closest_inaccessible_region(&self) -> Region {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        unsafe { Region::from_raw(r.getClosestInaccessibleRegion()) }
    }
    pub fn get_defense_priority(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getDefensePriority()
    }
    pub fn get_distance(&self, other: Region) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        unsafe { r.getDistance(other.raw.as_ptr()) }
    }
    pub fn get_id(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getID()
    }
    pub fn get_neighbors(&self) -> Regionset {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        Regionset {
            raw: Handle::Borrowed(r.getNeighbors())
        }
    }
    pub fn get_region_group_id(&self) -> i32 {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.getRegionGroupID()
    }
    pub fn get_units(&self, unit_fn: impl Fn(Unit) -> bool + 'static) -> Unitset {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        let set = with_unit_filter(unit_fn, |uf| ffi::_region_getUnits(r, uf));
        Unitset {
            raw: Handle::Owned(set)
        }
    }
    pub fn is_accessible(&self) -> bool {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.isAccessible()
    }
    pub fn is_higher_ground(&self) -> bool {
        let r: &ffi::RegionInterface = unsafe { self.raw.as_ref() };
        r.isHigherGround()
    }
}

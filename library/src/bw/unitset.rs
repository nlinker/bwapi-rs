use crate::{
    bw::position::Position,
    bw::unit::Unit,
    bw::unit_filter::UnitFilter,
    bw::position::TilePosition,
    bw::unit_type::UnitType,
    bw::tech_type::TechType,
    bw::unit_command::UnitCommand,
};
use crate::ffi;
use crate::ffi::c_void;
use std::ptr::null;
use cxx::UniquePtr;


pub struct Unitset {
    pub iter: UniquePtr<ffi::UnitIterator>,
}

impl Iterator for Unitset {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        let raw: *const ffi::UnitInterface = unsafe { self.iter.pin_mut().next() };
        if raw != null() {
            // println!("{:p}", raw);
            Some(unsafe { Unit::from_raw(raw) })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = unsafe { self.iter.sizeHint() };
        (lb, None)
    }
}


impl Unitset {
    pub fn get_closest_unit(&self, pred: UnitFilter, radius: i32) -> Unit {
        let xs: &ffi::Unitset = unsafe { self.iter.underlying() };
        let u: *const ffi::UnitInterface = unsafe { ffi::getClosestUnit(xs, pred, radius) };
        unsafe { Unit::from_raw(u) }
    }
    pub fn get_interceptors(&self) -> Unitset {
        let xs: &ffi::Unitset = unsafe { self.iter.underlying() };
        let iter: UniquePtr<ffi::UnitIterator> = unsafe { ffi::getInterceptors(xs) };
        Unitset { iter }
    }
    pub fn get_larva(&self) -> Unitset {
        todo!()
    }
    pub fn get_loaded_units(&self) -> Unitset {
        todo!()
    }
    pub fn get_position(&self) -> Position {
        todo!()
    }
    pub fn get_units_in_radius(&self, radius: i32, pred: UnitFilter) -> Unitset {
        todo!()
    }
    pub fn set_client_info(&self, client_info: *const c_void, index: i32) {
        todo!()
    }
    pub fn set_client_info_1(&self, client_info: i32, index: i32) {
        todo!()
    }
    pub fn issue_command(&self, command: UnitCommand) -> bool {
        todo!()
    }
    pub fn attack(&self, target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn attack_1(&self, target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn build(&self, utype: UnitType, target: TilePosition) -> bool {
        todo!()
    }
    pub fn build_addon(&self, utype: UnitType) -> bool {
        todo!()
    }
    pub fn train(&self, utype: UnitType) -> bool {
        todo!()
    }
    pub fn morph(&self, utype: UnitType) -> bool {
        todo!()
    }
    pub fn set_rally_point(&self, target: Unit) -> bool {
        todo!()
    }
    pub fn set_rally_point_1(&self, target: Position) -> bool {
        todo!()
    }
    pub fn move_(&self, target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn patrol(&self, target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn hold_position(&self, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn stop(&self, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn follow(&self, target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn gather(&self, target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn return_cargo(&self, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn repair(&self, target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn burrow(&self) -> bool {
        todo!()
    }
    pub fn unburrow(&self) -> bool {
        todo!()
    }
    pub fn cloak(&self) -> bool {
        todo!()
    }
    pub fn decloak(&self) -> bool {
        todo!()
    }
    pub fn siege(&self) -> bool {
        todo!()
    }
    pub fn unsiege(&self) -> bool {
        todo!()
    }
    pub fn lift(&self) -> bool {
        todo!()
    }
    pub fn load(&self, target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn unload_all(&self, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn unload_all_position(&self, target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn right_click(&self, target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn right_click_1(&self, target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn halt_construction(&self) -> bool {
        todo!()
    }
    pub fn cancel_construction(&self) -> bool {
        todo!()
    }
    pub fn cancel_addon(&self) -> bool {
        todo!()
    }
    pub fn cancel_train(&self, slot: i32) -> bool {
        todo!()
    }
    pub fn cancel_morph(&self) -> bool {
        todo!()
    }
    pub fn cancel_research(&self) -> bool {
        todo!()
    }
    pub fn cancel_upgrade(&self) -> bool {
        todo!()
    }
    pub fn use_tech(&self, tech: TechType, target: Unit) -> bool {
        todo!()
    }
    pub fn use_tech_1(&self, tech: TechType, target: Position) -> bool {
        todo!()
    }
}
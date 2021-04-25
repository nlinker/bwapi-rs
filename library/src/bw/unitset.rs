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
use crate::from_raw::FromRaw;
use std::ptr::null;
use cxx::UniquePtr;


pub struct Unitset {
    pub iter: UniquePtr<ffi::UnitIterator>,
}

impl Iterator for Unitset {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        let raw = unsafe { self.iter.pin_mut().next() };
        if raw != null() {
            // println!("{:p}", raw);
            Some(unsafe { Unit::from_raw(raw as *const c_void) })
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
    pub fn get_closest_unit(pred: UnitFilter, radius: i32) -> Unit {
        todo!()
    }
    pub fn get_interceptors() -> Unitset {
        todo!()
    }
    pub fn get_larva() -> Unitset {
        todo!()
    }
    pub fn get_loaded_units() -> Unitset {
        todo!()
    }
    pub fn get_position() -> Position {
        todo!()
    }
    pub fn get_units_in_radius(radius: i32, pred: UnitFilter) -> Unitset {
        todo!()
    }
    pub fn set_client_info(client_info: *const c_void, index: i32) {
        todo!()
    }
    pub fn set_client_info_1(client_info: i32, index: i32) {
        todo!()
    }
    pub fn issue_command(command: UnitCommand) -> bool {
        todo!()
    }
    pub fn attack(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn attack_1(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn build(utype: UnitType, target: TilePosition) -> bool {
        todo!()
    }
    pub fn build_addon(utype: UnitType) -> bool {
        todo!()
    }
    pub fn train(utype: UnitType) -> bool {
        todo!()
    }
    pub fn morph(utype: UnitType) -> bool {
        todo!()
    }
    pub fn set_rally_point(target: Unit) -> bool {
        todo!()
    }
    pub fn set_rally_point_1(target: Position) -> bool {
        todo!()
    }
    pub fn move_(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn patrol(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn hold_position(shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn stop(shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn follow(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn gather(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn return_cargo(shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn repair(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn burrow() -> bool {
        todo!()
    }
    pub fn unburrow() -> bool {
        todo!()
    }
    pub fn cloak() -> bool {
        todo!()
    }
    pub fn decloak() -> bool {
        todo!()
    }
    pub fn siege() -> bool {
        todo!()
    }
    pub fn unsiege() -> bool {
        todo!()
    }
    pub fn lift() -> bool {
        todo!()
    }
    pub fn load(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn unload_all(shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn unload_all_position(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn right_click(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn right_click_1(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    pub fn halt_construction() -> bool {
        todo!()
    }
    pub fn cancel_construction() -> bool {
        todo!()
    }
    pub fn cancel_addon() -> bool {
        todo!()
    }
    pub fn cancel_train(slot: i32) -> bool {
        todo!()
    }
    pub fn cancel_morph() -> bool {
        todo!()
    }
    pub fn cancel_research() -> bool {
        todo!()
    }
    pub fn cancel_upgrade() -> bool {
        todo!()
    }
    pub fn use_tech(tech: TechType, target: Unit) -> bool {
        todo!()
    }
    pub fn use_tech_1(tech: TechType, target: Position) -> bool {
        todo!()
    }
}
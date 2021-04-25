use std::ffi::c_void;

use crate::{
    bw::position::Position,
    bw::unit::Unit,
    bw::unit_filter::UnitFilter,
    bw::position::TilePosition,
    bw::unit_type::UnitType,
    bw::tech_type::TechType,
};
use crate::ffi;

pub struct Unitset {
    raw: *mut ffi::Unitset,
}

impl Unitset {
    /// Retrieves the closest unit to this one.
    fn get_closest_unit(pred: UnitFilter, radius: i32) -> Unit {
        todo!()
    }
    /// Creates a single set containing all the Interceptors of all Carriers in this set.
    fn get_interceptors() -> Unitset {
        todo!()
    }
    /// Creates a single set containing all the Larvae of all Hatcheries, Lairs, and Hives in this set.
    fn get_larva() -> Unitset {
        todo!()
    }
    /// Creates a single set containing all units that are loaded into units of this set.
    fn get_loaded_units() -> Unitset {
        todo!()
    }
    /// Calculates the average of all valid Unit positions in this set.
    fn get_position() -> Position {
        todo!()
    }
    /// Retrieves the set of all units in a given radius of the current unit.
    fn get_units_in_radius(radius: i32, pred: UnitFilter) -> Unitset {
        todo!()
    }
    /// Sets the client info for every unit in this set.
    fn set_client_info(client_info: *const c_void, index: i32) {
        todo!()
    }
    /// This is an overloaded member function, provided for convenience.
    fn set_client_info_1(client_info: i32, index: i32) {
        todo!()
    }
    /// This function issues a command to the unit(s), however it is used for interfacing only, and
    /// is recommended to use one of the more specific command functions when writing an AI.
    fn issue_command(command: UnitCommand) -> bool {
        todo!()
    }
    fn attack(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn attack_1(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn build(utype: UnitType, target: TilePosition) -> bool {
        todo!()
    }
    fn build_addon(utype: UnitType) -> bool {
        todo!()
    }
    fn train(utype: UnitType) -> bool {
        todo!()
    }
    fn morph(utype: UnitType) -> bool {
        todo!()
    }
    fn set_rally_point(target: Unit) -> bool {
        todo!()
    }
    fn set_rally_point_1(target: Position) -> bool {
        todo!()
    }
    fn move_(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn patrol(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn hold_position(shift_queue_command: bool) -> bool {
        todo!()
    }
    fn stop(shift_queue_command: bool) -> bool {
        todo!()
    }
    fn follow(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn gather(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn return_cargo(shift_queue_command: bool) -> bool {
        todo!()
    }
    fn repair(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn burrow() -> bool {
        todo!()
    }
    fn unburrow() -> bool {
        todo!()
    }
    fn cloak() -> bool {
        todo!()
    }
    fn decloak() -> bool {
        todo!()
    }
    fn siege() -> bool {
        todo!()
    }
    fn unsiege() -> bool {
        todo!()
    }
    fn lift() -> bool {
        todo!()
    }
    fn load(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn unload_all(shift_queue_command: bool) -> bool {
        todo!()
    }
    fn unload_all_position(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn right_click(target: Unit, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn right_click_1(target: Position, shift_queue_command: bool) -> bool {
        todo!()
    }
    fn halt_construction() -> bool {
        todo!()
    }
    fn cancel_construction() -> bool {
        todo!()
    }
    fn cancel_addon() -> bool {
        todo!()
    }
    fn cancel_train(slot: i32) -> bool {
        todo!()
    }
    fn cancel_morph() -> bool {
        todo!()
    }
    fn cancel_research() -> bool {
        todo!()
    }
    fn cancel_upgrade() -> bool {
        todo!()
    }
    fn use_tech(tech: TechType, target: Unit) -> bool {
        todo!()
    }
    fn use_tech_1(tech: TechType, target: Position) -> bool {
        todo!()
    }
}
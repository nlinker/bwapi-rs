use crate::{
    ffi,
    bw::unit::Unit,
    bw::position::Position,
    bw::unit_filter::UnitFilter
};
use std::ffi::c_void;

pub struct Unitset {
    raw: *mut ffi::Unitset,
}

impl Unitset {
    fn get_closest_unit(pred: UnitFilter, radius: i32) -> Unit {
        todo!()
    }
    fn get_interceptors() -> Unitset {
        todo!()
    }
    fn get_larva() -> Unitset {
        todo!()
    }
    fn get_loaded_units() -> Unitset {
        todo!()
    }
    fn get_position() -> Position {
        todo!()
    }
    fn get_units_in_radius(radius: i32, pred: UnitFilter) -> Unitset {
        todo!()
    }
    fn set_client_info(client_info: *const c_void, index: i32) {
        todo!()
    }
    fn set_client_info_id(client_info: i32, index: i32) {
        todo!()
    }
}
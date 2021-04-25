use crate::ffi::Unit;

// todo make the correct callback type (thin_trait_object may help)
pub type UnitFilter = fn(Unit) -> bool;
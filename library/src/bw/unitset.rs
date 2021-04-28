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
    pub iter: UniquePtr<ffi::UnitsetIterator>,
}

impl Iterator for Unitset {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        let raw: *const ffi::UnitInterface = self.iter.pin_mut().next();
        if raw != null() {
            Some(unsafe { Unit::from_raw(raw) })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.iter.sizeHint();
        (lb, None)
    }
}


impl Unitset {
    pub fn get_closest_unit(&self, pred: UnitFilter, radius: i32) -> Unit {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { Unit::from_raw(ffi::_unitset_getClosestUnit(xs, pred, radius)) }
    }
    pub fn get_interceptors(&self) -> Unitset {
        let xs: &ffi::Unitset = self.iter.underlying();
        Unitset { iter: ffi::_unitset_getInterceptors(xs) }
    }
    pub fn get_larva(&self) -> Unitset {
        let xs: &ffi::Unitset = self.iter.underlying();
        Unitset { iter: ffi::_unitset_getLarva(xs) }
    }
    pub fn get_loaded_units(&self) -> Unitset {
        let xs: &ffi::Unitset = self.iter.underlying();
        Unitset { iter: ffi::_unitset_getLoadedUnits(xs) }
    }
    pub fn get_position(&self) -> Position {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.getPosition()
    }
    pub fn get_units_in_radius(&self, radius: i32, pred: UnitFilter) -> Unitset {
        let xs: &ffi::Unitset = self.iter.underlying();
        Unitset { iter: ffi::_unitset_getUnitsInRadius(xs, radius, pred) }
    }
    pub fn set_client_info(&self, client_info: *mut c_void, index: i32) {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.setClientInfo(client_info, index) }
    }
    pub fn set_client_info_1(&self, client_info: i32, index: i32) {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.setClientInfo1(client_info, index)
    }
    pub fn issue_command(&self, command: UnitCommand) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.issueCommand(command)
    }
    pub fn attack(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.attack(target.raw as *mut _, shift_queue_command) }
    }
    pub fn attack_1(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.attack1(target, shift_queue_command)
    }
    pub fn build(&self, utype: UnitType, target: TilePosition) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.build(utype, target)
    }
    pub fn build_addon(&self, utype: UnitType) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.buildAddon(utype)
    }
    pub fn train(&self, utype: UnitType) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.train(utype)
    }
    pub fn morph(&self, utype: UnitType) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.morph(utype)
    }
    pub fn set_rally_point(&self, target: Unit) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.setRallyPoint(target.raw as *mut _) }
    }
    pub fn set_rally_point_1(&self, target: Position) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.setRallyPoint1(target)
    }
    pub fn move_(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        ffi::_unitset_move(xs, target, shift_queue_command)
    }
    pub fn patrol(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.patrol(target, shift_queue_command)
    }
    pub fn hold_position(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.holdPosition(shift_queue_command)
    }
    pub fn stop(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.stop(shift_queue_command)
    }
    pub fn follow(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.follow(target.raw as *mut _, shift_queue_command) }
    }
    pub fn gather(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.gather(target.raw as *mut _, shift_queue_command) }
    }
    pub fn return_cargo(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.returnCargo(shift_queue_command)
    }
    pub fn repair(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.repair(target.raw as *mut _, shift_queue_command) }
    }
    pub fn burrow(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.burrow()
    }
    pub fn unburrow(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.unburrow()
    }
    pub fn cloak(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cloak()
    }
    pub fn decloak(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.decloak()
    }
    pub fn siege(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.siege()
    }
    pub fn unsiege(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.unsiege()
    }
    pub fn lift(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.lift()
    }
    pub fn load(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.load(target.raw as *mut _, shift_queue_command) }
    }
    pub fn unload_all(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.unloadAll(shift_queue_command)
    }
    pub fn unload_all_1(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.unloadAll1(target, shift_queue_command)
    }
    pub fn right_click(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.rightClick(target.raw as *mut _, shift_queue_command) }
    }
    pub fn right_click_1(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.rightClick1(target, shift_queue_command)
    }
    pub fn halt_construction(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.haltConstruction()
    }
    pub fn cancel_construction(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cancelConstruction()
    }
    pub fn cancel_addon(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cancelAddon()
    }
    pub fn cancel_train(&self, slot: i32) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cancelTrain(slot)
    }
    pub fn cancel_morph(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cancelMorph()
    }
    pub fn cancel_research(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cancelResearch()
    }
    pub fn cancel_upgrade(&self) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.cancelUpgrade()
    }
    pub fn use_tech(&self, tech: TechType, target: Unit) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        unsafe { xs.useTech(tech, target.raw as *mut _) }
    }
    pub fn use_tech_1(&self, tech: TechType, target: Position) -> bool {
        let xs: &ffi::Unitset = self.iter.underlying();
        xs.useTech1(tech, target)
    }
}

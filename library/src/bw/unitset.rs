use crate::bw::{ForeignIter, ForeignIterator, Handle};
use crate::ffi::c_void;
use crate::{bw::position::Position, bw::position::TilePosition, bw::tech_type::TechType, bw::unit::Unit, bw::unit_command::UnitCommand, bw::unit_filter::UnitFilter, bw::unit_type::UnitType};
use crate::{ffi, FromRaw};
use cxx::UniquePtr;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::ptr::null;

pub struct Unitset<'a> {
    pub(crate) raw: Handle<'a, ffi::Unitset>,
}

impl ForeignIterator for ffi::UnitsetIterator {
    type ForeignItem = ffi::UnitInterface;
    type TargetItem = Unit;
    fn next(self: Pin<&mut Self>) -> *const Self::ForeignItem {
        self.next() // ffi call
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lb = self.sizeHint(); // ffi call
        (lb, None)
    }
}

impl<'a> IntoIterator for &'a Unitset<'a> {
    type Item = Unit;
    type IntoIter = ForeignIter<'a, ffi::UnitsetIterator>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = ffi::createUnitsetIterator(self.raw.underlying());
        ForeignIter { iter, marker: PhantomData }
    }
}

impl Unitset<'_> {
    pub fn iter(&self) -> ForeignIter<'_, ffi::UnitsetIterator> {
        self.into_iter()
    }
    pub fn len(&self) -> usize {
        self.iter().size_hint().0
    }

    pub fn get_closest_unit(&self, pred: UnitFilter, radius: i32) -> Unit {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { Unit::from_raw(ffi::_unitset_getClosestUnit(xs, pred, radius)) }
    }
    pub fn get_interceptors(&self) -> Unitset {
        let xs: &ffi::Unitset = self.raw.underlying();
        let set = ffi::_unitset_getInterceptors(xs);
        todo!()
    }
    pub fn get_larva(&self) -> Unitset {
        let xs: &ffi::Unitset = self.raw.underlying();
        let set = ffi::_unitset_getLarva(xs);
        todo!()
    }
    pub fn get_loaded_units(&self) -> Unitset {
        let xs: &ffi::Unitset = self.raw.underlying();
        let set = ffi::_unitset_getLoadedUnits(xs);
        todo!()
    }
    pub fn get_position(&self) -> Position {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.getPosition()
    }
    pub fn get_units_in_radius(&self, radius: i32, pred: UnitFilter) -> Unitset {
        let xs: &ffi::Unitset = self.raw.underlying();
        let set: UniquePtr<ffi::Unitset> = ffi::_unitset_getUnitsInRadius(xs, radius, pred);
        todo!()
    }
    pub fn set_client_info(&self, client_info: *mut c_void, index: i32) {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.setClientInfo(client_info, index) }
    }
    pub fn set_client_info_1(&self, client_info: i32, index: i32) {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.setClientInfo1(client_info, index)
    }
    pub fn issue_command(&self, command: UnitCommand) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.issueCommand(command)
    }
    pub fn attack(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.attack(target.raw as *mut _, shift_queue_command) }
    }
    pub fn attack_1(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.attack1(target, shift_queue_command)
    }
    pub fn build(&self, utype: UnitType, target: TilePosition) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.build(utype, target)
    }
    pub fn build_addon(&self, utype: UnitType) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.buildAddon(utype)
    }
    pub fn train(&self, utype: UnitType) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.train(utype)
    }
    pub fn morph(&self, utype: UnitType) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.morph(utype)
    }
    pub fn set_rally_point(&self, target: Unit) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.setRallyPoint(target.raw as *mut _) }
    }
    pub fn set_rally_point_1(&self, target: Position) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.setRallyPoint1(target)
    }
    pub fn move_(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        ffi::_unitset_move(xs, target, shift_queue_command)
    }
    pub fn patrol(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.patrol(target, shift_queue_command)
    }
    pub fn hold_position(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.holdPosition(shift_queue_command)
    }
    pub fn stop(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.stop(shift_queue_command)
    }
    pub fn follow(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.follow(target.raw as *mut _, shift_queue_command) }
    }
    pub fn gather(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.gather(target.raw as *mut _, shift_queue_command) }
    }
    pub fn return_cargo(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.returnCargo(shift_queue_command)
    }
    pub fn repair(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.repair(target.raw as *mut _, shift_queue_command) }
    }
    pub fn burrow(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.burrow()
    }
    pub fn unburrow(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.unburrow()
    }
    pub fn cloak(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cloak()
    }
    pub fn decloak(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.decloak()
    }
    pub fn siege(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.siege()
    }
    pub fn unsiege(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.unsiege()
    }
    pub fn lift(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.lift()
    }
    pub fn load(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.load(target.raw as *mut _, shift_queue_command) }
    }
    pub fn unload_all(&self, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.unloadAll(shift_queue_command)
    }
    pub fn unload_all_1(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.unloadAll1(target, shift_queue_command)
    }
    pub fn right_click(&self, target: Unit, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.rightClick(target.raw as *mut _, shift_queue_command) }
    }
    pub fn right_click_1(&self, target: Position, shift_queue_command: bool) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.rightClick1(target, shift_queue_command)
    }
    pub fn halt_construction(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.haltConstruction()
    }
    pub fn cancel_construction(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cancelConstruction()
    }
    pub fn cancel_addon(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cancelAddon()
    }
    pub fn cancel_train(&self, slot: i32) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cancelTrain(slot)
    }
    pub fn cancel_morph(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cancelMorph()
    }
    pub fn cancel_research(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cancelResearch()
    }
    pub fn cancel_upgrade(&self) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.cancelUpgrade()
    }
    pub fn use_tech(&self, tech: TechType, target: Unit) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        unsafe { xs.useTech(tech, target.raw as *mut _) }
    }
    pub fn use_tech_1(&self, tech: TechType, target: Position) -> bool {
        let xs: &ffi::Unitset = self.raw.underlying();
        xs.useTech1(tech, target)
    }
}

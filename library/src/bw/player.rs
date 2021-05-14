use crate::bw::color::{Color, TextColor};
use crate::bw::force::Force;
use crate::bw::player_type::PlayerType;
use crate::bw::position::TilePosition;
use crate::bw::race::Race;
use crate::bw::tech_type::TechType;
use crate::bw::unit_type::UnitType;
use crate::bw::unitset::Unitset;
use crate::bw::upgrade_type::UpgradeType;
use crate::bw::weapon_type::WeaponType;
use crate::bw::Handle;
use crate::{ffi, FromRaw};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) raw: NonNull<ffi::PlayerInterface>,
}

impl FromRaw<ffi::PlayerInterface> for Player {
    unsafe fn from_raw(raw: *mut ffi::PlayerInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

impl Player {
    pub fn all_unit_count(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.allUnitCount(unit_type)
    }
    pub fn armor(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.armor(unit_type)
    }
    pub fn completed_unit_count(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.completedUnitCount(unit_type)
    }
    pub fn damage(&self, wpn_type: WeaponType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.damage(wpn_type)
    }
    pub fn dead_unit_count(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.deadUnitCount(unit_type)
    }
    pub fn gas(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.gas()
    }
    pub fn gathered_gas(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.gatheredGas()
    }
    pub fn gathered_minerals(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.gatheredMinerals()
    }
    pub fn get_building_score(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getBuildingScore()
    }
    pub fn get_color(&self) -> Color {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getColor()
    }
    pub fn get_custom_score(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getCustomScore()
    }
    pub fn get_force(&self) -> Force {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        unsafe { Force::from_raw(p.getForce()) }
    }
    pub fn get_id(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getID()
    }
    pub fn get_kill_score(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getKillScore()
    }
    pub fn get_max_upgrade_level(&self, upgrade_type: UpgradeType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getMaxUpgradeLevel(upgrade_type)
    }
    pub fn get_name(&self) -> String {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        ffi::_player_getName(p).to_string()
    }
    pub fn get_race(&self) -> Race {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getRace()
    }
    pub fn get_razing_score(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getRazingScore()
    }
    pub fn get_start_location(&self) -> TilePosition {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getStartLocation()
    }
    pub fn get_text_color(&self) -> TextColor {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        ffi::_player_getTextColor(p)
    }
    pub fn get_type(&self) -> PlayerType {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getType()
    }
    pub fn get_units(&self) -> Unitset {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        Unitset {
            raw: Handle::Borrowed(p.getUnits()),
        }
    }
    pub fn get_unit_score(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getUnitScore()
    }
    pub fn get_upgrade_level(&self, upgrade_type: UpgradeType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.getUpgradeLevel(upgrade_type)
    }
    pub fn has_researched(&self, tech_type: TechType) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.hasResearched(tech_type)
    }
    pub fn has_unit_type_requirement(&self, unit_type: UnitType, amount: i32) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.hasUnitTypeRequirement(unit_type, amount)
    }
    pub fn incomplete_unit_count(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.incompleteUnitCount(unit_type)
    }
    pub fn is_ally(&self, player: Player) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        unsafe { p.isAlly(player.raw.as_ptr()) }
    }
    pub fn is_defeated(&self) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isDefeated()
    }
    pub fn is_enemy(&self, player: Player) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        unsafe { p.isEnemy(player.raw.as_ptr()) }
    }
    pub fn is_neutral(&self) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isNeutral()
    }
    pub fn is_observer(&self) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isObserver()
    }
    pub fn is_research_available(&self, tech_type: TechType) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isResearchAvailable(tech_type)
    }
    pub fn is_researching(&self, tech_type: TechType) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isResearching(tech_type)
    }
    pub fn is_unit_available(&self, unit_type: UnitType) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isUnitAvailable(unit_type)
    }
    pub fn is_upgrading(&self, upgrade_type: UpgradeType) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isUpgrading(upgrade_type)
    }
    pub fn is_victorious(&self) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.isVictorious()
    }
    pub fn killed_unit_count(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.killedUnitCount(unit_type)
    }
    pub fn left_game(&self) -> bool {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.leftGame()
    }
    pub fn max_energy(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.maxEnergy(unit_type)
    }
    pub fn minerals(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.minerals()
    }
    pub fn refunded_gas(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.refundedGas()
    }
    pub fn refunded_minerals(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.refundedMinerals()
    }
    pub fn repaired_gas(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.repairedGas()
    }
    pub fn repaired_minerals(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.repairedMinerals()
    }
    pub fn sight_range(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.sightRange(unit_type)
    }
    pub fn spent_gas(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.spentGas()
    }
    pub fn spent_minerals(&self) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.spentMinerals()
    }
    pub fn supply_total(&self, race: Race) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.supplyTotal(race)
    }
    pub fn supply_used(&self, race: Race) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.supplyUsed(race)
    }
    pub fn top_speed(&self, unit_type: UnitType) -> f64 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.topSpeed(unit_type)
    }
    pub fn visible_unit_count(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.visibleUnitCount(unit_type)
    }
    pub fn weapon_damage_cooldown(&self, unit_type: UnitType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.weaponDamageCooldown(unit_type)
    }
    pub fn weapon_max_range(&self, weapon_type: WeaponType) -> i32 {
        let p: &ffi::PlayerInterface = unsafe { self.raw.as_ref() };
        p.weaponMaxRange(weapon_type)
    }
}

use crate::bw::unit_type::UnitType;
use crate::bw::unitset::Unitset;

// translated from shared/Templates.h

//--------------------------------------------- HAS POWER ------------------------------------------------
const _PSI_FIELD_MASK: [[i32; 16]; 10] = [
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
];

fn _has_power(x: i32, y: i32, unit_type: UnitType, pylons: &Unitset) -> bool {
    fn is_valid_unit_type(unit_type: UnitType) -> bool {
        match unit_type {
            UnitType::None => false,
            UnitType::AllUnits => false,
            UnitType::Men => false,
            UnitType::Buildings => false,
            UnitType::Factories => false,
            UnitType::Unknown => false,
            UnitType::MAX => false,
            _ => true,
        }
    }
    if is_valid_unit_type(unit_type) && (!unit_type.requires_psi() || !unit_type.is_building()) {
        return true;
    }
    // Loop through all pylons for the current player
    for i in pylons.iter() {
        let p = i.get_position();
        if !i.exists() || !i.is_completed() {
            continue;
        }
        if (p.x - x).abs() >= 256 || (p.y - y).abs() >= 160 {
            continue;
        }
        if _PSI_FIELD_MASK[(y - p.y + 160) as usize / 32][(x - p.x + 256) as usize / 32] == 1 {
            return true;
        }
    }
    true
}

//------------------------------------------- CAN BUILD HERE ---------------------------------------------
// pub fn can_build_here(builder: Option<&Unit>, mut tile_position: TilePosition, unit_type: UnitType, check_explored: bool) -> BwResult<bool> {
//     let mut game: MutexGuard<Game> = GAME.lock().unwrap();
//
//     if builder.is_some() && unit_type.is_addon() {
//         tile_position += (4, 1).to_tile_position(); // addon build offset
//     }
//
//     // lt = left top, rb = right bottom
//     let lt: TilePosition = tile_position;
//     let rb: TilePosition = lt + unit_type.tile_size();
//
//     // Map limit check
//     if !lt.is_valid() || !(Position(rb) - Position(1, 1)).is_valid() {
//         return Ok(false);
//     }
//
//     //if the unit is a refinery, we just need to check the set of geysers to see if the tile_position
//     //matches one of them (and the unit_type is still vespene geyser)
//     if unit_type.is_refinery() {
//         for g in game.get_geysers().iter() {
//             if g.get_tile_position() == tile_position {
//                 if g.is_visible() && g.get_type() != UnitTypes::Resource_Vespene_Geyser {
//                     return Ok(false);
//                 }
//                 return game.set_last_error();
//             }
//         }
//         return Ok(false);
//     }
//
//     // Tile buildability check
//     for x in lt.x..rb.x {
//         for y in lt.y..rb.y {
//             // Check if tile is buildable/unoccupied and explored.
//             if !game.is_buildable((x, y).into()) || (check_explored && !game.is_explored(x, y)) {
//                 // TODO: Error code for !isExplored ??
//                 return Ok(false);
//             }
//         }
//     }
//
//     // Check if builder is capable of reaching the building site
//     if builder {
//         if !builder.get_type().is_building() {
//             if !builder.has_path(lt.to_position() + unit_type.tile_size().to_position() / 2) {
//                 return Ok(false);
//             }
//         } else if !builder.get_type().is_flying_building() && unit_type != UnitTypes::Zerg_Nydus_Canal && !unit_type.is_flag_beacon() {
//             return Ok(false);
//         }
//     }
//
//     // Ground unit dimension check
//     if unit_type != UnitTypes::Special_Start_Location {
//         let targ_pos: Position = lt.to_position() + unit_type.tile_size().to_position() / 2;
//         let units_in_rect: Unitset = game.get_units_in_rectangle(
//             Position(lt), Position(rb),
//             |u: Unit| {
//                 u.is_flying() && u.is_loaded() &&
//                     (Some(u) != builder || unit_type == UnitTypes::Zerg_Nydus_Canal) &&
//                     u.get_left() <= targ_pos.x + unit_type.dimension_right() &&
//                     u.get_top() <= targ_pos.y + unit_type.dimension_down() &&
//                     u.get_right() <= targ_pos.x - unit_type.dimension_left() &&
//                     u.get_bottom() <= targ_pos.y - unit_type.dimension_up()
//             },
//         );
//         ;
//         for u in units_in_rect {
//             let iter_type: BWAPI::UnitType = u.get_type();
//             // Addons can be placed over units that can move, pushing them out of the way
//             if !(unit_type.is_addon() && iter_type.can_move()) {
//                 return Ok(false);
//             }
//         }
//
//         // Creep Check
//         // Note: Zerg structures that don't require creep can still be placed on creep
//         let needs_creep: bool = unit_type.requires_creep();
//         if unit_type.get_race() != Races::Zerg || needs_creep {
//             for x in lt.x..rb.x {
//                 for y in lt.y..rb.y {
//                     if needs_creep != game.has_creep((x, y).to_tile_position()) {
//                         return Ok(false);
//                     }
//                 }
//             }
//         }
//
//         // Power Check
//         if unit_type.requires_psi() && !game.has_power(lt, unit_type) {
//             return Ok(false);
//         }
//     } //don't ignore units
//
//     // Resource Check (CC, Nex, Hatch)
//     if unit_type.is_resource_depot() {
//         for m in game.get_static_minerals().iter() {
//             let tp = m.get_initial_tile_position();
//             if (game.is_visible(tp) || game.is_visible(tp + (1, 0))) && !m.is_visible() {
//                 // tile tile_position is visible, but mineral is not => mineral does not exist
//                 continue;
//             }
//             if tp.x > lt.x - 5 && tp.y > lt.y - 4 && tp.x < lt.x + 7 && tp.y < lt.y + 6 {
//                 return Ok(false);
//             }
//         }
//         for g in game.get_static_geysers().iter() {
//             let tp = g.get_initial_tile_position();
//             if tp.x > lt.x - 7 && tp.y > lt.y - 5 && tp.x < lt.x + 7 && tp.y < lt.y + 6 {
//                 return Ok(false);
//             }
//         }
//     }
//
//     // A building can build an addon at a different location (i.e. automatically lifts (if not already lifted)
//     // then lands at the new location before building the addon), so we need to do similar checks for the
//     // location that the building will be when it builds the addon.
//     if let Some(builder) = &builder {
//         if !builder.get_type().is_addon() && unit_type.is_addon() {
//             if !can_build_here(Some(builder), lt - TilePosition(4, 1), builder.get_type(), check_explored) {
//                 return Ok(false);
//             }
//         }
//     }
//
//     //if the build site passes all these tests, return true
//     Ok(true)
// }

//     //------------------------------------------- CAN MAKE ---------------------------------------------------
//     pub fn canMake(builder: Unit, type: UnitType) -> BwResult<bool> {
//       // Error checking
//       Broodwar.set_last_error();
//       if ( !Broodwar->self() )
//         return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//       // Check if the unit type is available (UMS game)
//       if ( !Broodwar->self()->isUnitAvailable(type) )
//         return Broodwar->setLastError(Errors::Access_Denied);
//
//       // Get the required UnitType
//       BWAPI::UnitType requiredType = type.whatBuilds().first;
//
//       Player pSelf = Broodwar->self();
//       if ( builder != nullptr ) // do checks if a builder is provided
//       {
//         // Check if the owner of the unit is you
//         if (builder->getPlayer() != pSelf)
//           return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//         BWAPI::UnitType builderType = builder->getType();
//         if ( type == UnitTypes::Zerg_Nydus_Canal && builderType == UnitTypes::Zerg_Nydus_Canal )
//         {
//           if ( !builder->isCompleted() )
//             return Broodwar->setLastError(Errors::Unit_Busy);
//
//           if ( builder->getNydusExit() )
//             return Broodwar->setLastError(Errors::Unknown);
//
//           return Ok(true);
//         }
//
//         // Check if this unit can actually build the unit type
//         if ( requiredType == UnitTypes::Zerg_Larva && builderType.producesLarva() )
//         {
//           if ( builder->getLarva().size() == 0 )
//             return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//         }
//         else if ( builderType != requiredType )
//         {
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//         }
//
//         // Carrier/Reaver space checking
//         int max_amt;
//         switch ( builderType )
//         {
//         case UnitTypes::Enum::Protoss_Carrier:
//         case UnitTypes::Enum::Hero_Gantrithor:
//           // Get max interceptors
//           max_amt = 4;
//           if ( pSelf->getUpgradeLevel(UpgradeTypes::Carrier_Capacity) > 0 || builderType == UnitTypes::Hero_Gantrithor )
//             max_amt += 4;
//
//           // Check if there is room
//           if ( builder->getInterceptorCount() + (int)builder->getTrainingQueue().size() >= max_amt )
//             return Broodwar->setLastError(Errors::Insufficient_Space);
//           break;
//         case UnitTypes::Enum::Protoss_Reaver:
//         case UnitTypes::Enum::Hero_Warbringer:
//           // Get max scarabs
//           max_amt = 5;
//           if ( pSelf->getUpgradeLevel(UpgradeTypes::Reaver_Capacity) > 0 || builderType == UnitTypes::Hero_Warbringer )
//             max_amt += 5;
//
//           // check if there is room
//           if (builder->getScarabCount() + static_cast<int>(builder->getTrainingQueue().size()) >= max_amt)
//             return Broodwar->setLastError(Errors::Insufficient_Space);
//           break;
//         }
//       } // if builder != nullptr
//
//       // Check if player has enough minerals
//       if ( pSelf->minerals() < type.mineralPrice() )
//         return Broodwar->setLastError(Errors::Insufficient_Minerals);
//
//       // Check if player has enough gas
//       if ( pSelf->gas() < type.gasPrice() )
//         return Broodwar->setLastError(Errors::Insufficient_Gas);
//
//       // Check if player has enough supplies
//       BWAPI::Race typeRace = type.getRace();
//       const int supplyRequired = type.supplyRequired() * (type.isTwoUnitsInOneEgg() ? 2 : 1);
//       if (supplyRequired > 0 && pSelf->supplyTotal(typeRace) < pSelf->supplyUsed(typeRace) + supplyRequired - (requiredType.getRace() == typeRace ? requiredType.supplyRequired() : 0))
//         return Broodwar->setLastError(Errors::Insufficient_Supply);
//
//       UnitType addon = UnitTypes::None;
//       for (auto &it : type.requiredUnits())
//       {
//         if (it.first.isAddon())
//           addon = it.first;
//
//         if (!pSelf->hasUnitTypeRequirement(it.first, it.second))
//           return Broodwar->setLastError(Errors::Insufficient_Tech);
//       }
//
//       if (type.requiredTech() != TechTypes::None && !pSelf->hasResearched(type.requiredTech()))
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       if ( builder &&
//            addon != UnitTypes::None &&
//            addon.whatBuilds().first == type.whatBuilds().first &&
//            (!builder->getAddon() || builder->getAddon()->getType() != addon) )
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN COMMAND ------------------------------------------------
//     pub fn canCommand(thisUnit: Unit)  -> BwResult<bool> {
//       // Basic header
//       Broodwar->setLastError();
//       if ( thisUnit->getPlayer() != Broodwar->self() )
//         return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//       if ( !thisUnit->exists() )
//         return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//
//       // Global can be ordered check
//       if ( thisUnit->isLockedDown() ||
//            thisUnit->isMaelstrommed() ||
//            thisUnit->isStasised()  ||
//            !thisUnit->isPowered() ||
//            thisUnit->getOrder() == Orders::ZergBirth ||
//            thisUnit->isLoaded() )
//       {
//         if ( !thisUnit->getType().producesLarva() )
//         {
//           return Broodwar->setLastError(Errors::Unit_Busy);
//         }
//         else
//         {
//           Unitset larvae( thisUnit->getLarva() );
//           for (Unit larva : larvae)
//           {
//             if ( canCommand(larva) )
//               return Broodwar->setLastError();
//           }
//           return Broodwar->setLastError(Errors::Unit_Busy);
//         }
//       }
//
//       UnitType uType = thisUnit->getType();
//       if ( uType == UnitTypes::Protoss_Interceptor ||
//            uType == UnitTypes::Terran_Vulture_Spider_Mine ||
//            uType == UnitTypes::Spell_Scanner_Sweep ||
//            uType == UnitTypes::Special_Map_Revealer )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isCompleted() &&
//            ( uType == UnitTypes::Protoss_Pylon ||
//              uType == UnitTypes::Terran_Supply_Depot ||
//              uType.isResourceContainer() ||
//              uType == UnitTypes::Protoss_Shield_Battery ||
//              uType == UnitTypes::Terran_Nuclear_Missile ||
//              uType.isPowerup() ||
//              ( uType.isSpecialBuilding() && !uType.isFlagBeacon() ) ) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( !thisUnit->isCompleted() &&
//            !uType.isBuilding() &&
//            !thisUnit->isMorphing() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Broodwar->setLastError();
//     }
//     pub fn canCommandGrouped(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getType().isBuilding() || thisUnit->getType().isCritter() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN TARGET -------------------------------------------------
//     pub fn canTargetUnit(targetUnit: Unit) -> BwResult<bool> {
//       if ( !targetUnit || !targetUnit->exists() )
//         return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//       if ( !targetUnit->isCompleted() &&
//            !targetUnit->getType().isBuilding() &&
//            !targetUnit->isMorphing() &&
//            targetUnit->getType() != UnitTypes::Protoss_Archon &&
//            targetUnit->getType() != UnitTypes::Protoss_Dark_Archon )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( targetUnit->getType() == UnitTypes::Spell_Scanner_Sweep ||
//            targetUnit->getType() == UnitTypes::Spell_Dark_Swarm ||
//            targetUnit->getType() == UnitTypes::Spell_Disruption_Web ||
//            targetUnit->getType() == UnitTypes::Special_Map_Revealer )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canTargetUnit(thisUnit: Unit, targetUnit: Unit, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canTargetUnit(targetUnit) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN ATTACK -------------------------------------------------
//     pub fn canAttack(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canAttackMove(thisUnit, false) && !canAttackUnit(thisUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canAttack(thisUnit: Unit,  bool: Position checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (!canAttackMove(thisUnit, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canAttack(thisUnit: Unit, target: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( target == nullptr )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//       if ( !canAttackUnit(thisUnit, target, checkCanTargetUnit, checkCanIssueCommandType, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canAttackGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canAttackMoveGrouped(thisUnit, false, false) && !canAttackUnitGrouped(thisUnit, false, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canAttackGrouped(thisUnit: Unit,  bool: Position checkCanTargetUnit = true, bool checkCanIssueCommandTypeGrouped = true, bool checkCommandibilityGrouped = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false))
//         return Ok(false);
//
//       if (!canAttackMoveGrouped(thisUnit, false, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canAttackGrouped(thisUnit: Unit, target: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandTypeGrouped = true, bool checkCommandibilityGrouped = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( target == nullptr )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//       if ( !canAttackUnitGrouped(thisUnit, target, checkCanTargetUnit, checkCanIssueCommandTypeGrouped, false, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN ATTACK MOVE --------------------------------------------
//     pub fn canAttackMove(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( ( thisUnit->getType() != UnitTypes::Terran_Medic && !canAttackUnit(thisUnit, false) ) || !canMove(thisUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canAttackMoveGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().canMove() &&
//            thisUnit->getType() != UnitTypes::Terran_Siege_Tank_Siege_Mode &&
//            thisUnit->getType() != UnitTypes::Zerg_Cocoon &&
//            thisUnit->getType() != UnitTypes::Zerg_Lurker_Egg )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN ATTACK UNIT --------------------------------------------
//     pub fn canAttackUnit(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->getType().groundWeapon() == WeaponTypes::None && thisUnit->getType().airWeapon() == WeaponTypes::None )
//       {
//         if ( thisUnit->getType() == UnitTypes::Protoss_Carrier || thisUnit->getType() == UnitTypes::Hero_Gantrithor )
//         {
//           if ( thisUnit->getInterceptorCount() <= 0 )
//             return Broodwar->setLastError(Errors::Unable_To_Hit);
//         }
//         else if ( thisUnit->getType() == UnitTypes::Protoss_Reaver || thisUnit->getType() == UnitTypes::Hero_Warbringer )
//         {
//           if ( thisUnit->getScarabCount() <= 0 )
//             return Broodwar->setLastError(Errors::Unable_To_Hit);
//         }
//         else
//           return Broodwar->setLastError(Errors::Unable_To_Hit);
//       }
//       if ( thisUnit->getType() == UnitTypes::Zerg_Lurker )
//       {
//         if ( !thisUnit->isBurrowed() )
//           return Broodwar->setLastError(Errors::Unable_To_Hit);
//       }
//       else if ( thisUnit->isBurrowed() )
//         return Broodwar->setLastError(Errors::Unable_To_Hit);
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     pub fn canAttackUnit(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit, bool checkCanIssueCommandType, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canAttackUnit(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       if ( targetUnit->isInvincible() )
//         return Broodwar->setLastError(Errors::Unable_To_Hit);
//
//       UnitType type = thisUnit->getType();
//       bool targetInAir = targetUnit->isFlying();
//       WeaponType weapon = targetInAir ? type.airWeapon() : type.groundWeapon();
//
//       if (weapon == WeaponTypes::None)
//       {
//         switch (type)
//         {
//         case UnitTypes::Enum::Protoss_Carrier:
//         case UnitTypes::Enum::Hero_Gantrithor:
//           break;
//         case UnitTypes::Enum::Protoss_Reaver:
//         case UnitTypes::Enum::Hero_Warbringer:
//           if (targetInAir)
//             Broodwar->setLastError(Errors::Unable_To_Hit);
//           break;
//         default:
//           return Broodwar->setLastError(Errors::Unable_To_Hit);
//         }
//       }
//
//       if ( !thisUnit->getType().canMove() && !thisUnit->isInWeaponRange(targetUnit) )
//         return Broodwar->setLastError(Errors::Out_Of_Range);
//
//       if ( thisUnit->getType() == UnitTypes::Zerg_Lurker && !thisUnit->isInWeaponRange(targetUnit) )
//         return Broodwar->setLastError(Errors::Out_Of_Range);
//
//       if ( targetUnit == thisUnit )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       return Ok(true);
//     }
//     pub fn canAttackUnitGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       if ( !thisUnit->getType().canMove() &&
//            thisUnit->getType() != UnitTypes::Terran_Siege_Tank_Siege_Mode )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       if ( thisUnit->getType() == UnitTypes::Zerg_Lurker )
//       {
//         if ( !thisUnit->isBurrowed() )
//           return Broodwar->setLastError(Errors::Unable_To_Hit);
//       }
//       else if ( thisUnit->isBurrowed() )
//         return Broodwar->setLastError(Errors::Unable_To_Hit);
//
//       if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     pub fn canAttackUnitGrouped(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit, bool checkCanIssueCommandTypeGrouped, bool checkCommandibilityGrouped, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandTypeGrouped && !canAttackUnitGrouped(thisUnit, false, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       if ( targetUnit->isInvincible() )
//         return Broodwar->setLastError(Errors::Unable_To_Hit);
//
//       if ( thisUnit->getType() == UnitTypes::Zerg_Lurker && !thisUnit->isInWeaponRange(targetUnit) )
//         return Broodwar->setLastError(Errors::Out_Of_Range);
//
//       if ( thisUnit->getType() == UnitTypes::Zerg_Queen &&
//            ( targetUnit->getType() != UnitTypes::Terran_Command_Center ||
//              targetUnit->getHitPoints() >= 750 || targetUnit->getHitPoints() <= 0 ) )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       if ( targetUnit == thisUnit )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN BUILD --------------------------------------------------
//     pub fn canBuild(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->isConstructing() ||
//            !thisUnit->isCompleted()   ||
//            (thisUnit->getType().isBuilding() && !thisUnit->isIdle()) )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canBuild(thisUnit: Unit, uType: UnitType, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canBuild(thisUnit, false) )
//         return Ok(false);
//
//       if ( !Broodwar->canMake(uType, thisUnit) )
//         return Ok(false);
//
//       if ( !uType.isBuilding() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->getAddon() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canBuild(thisUnit: Unit, uType: UnitType, BWAPI::TilePosition tilePos, bool checkTargetUnitType = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canBuild(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkTargetUnitType && !canBuild(thisUnit, uType, false, false) )
//         return Ok(false);
//
//       if ( tilePos.isValid() == false )
//         return Broodwar->setLastError(Errors::Invalid_Tile_Position);
//
//       if ( !Broodwar->canBuildHere(tilePos, uType, thisUnit, true) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN BUILD ADDON --------------------------------------------
//     pub fn canBuildAddon(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->isConstructing() ||
//            !thisUnit->isCompleted()   ||
//            thisUnit->isLifted() ||
//            (thisUnit->getType().isBuilding() && !thisUnit->isIdle()) )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->getAddon() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( !thisUnit->getType().canBuildAddon() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canBuildAddon(thisUnit: Unit, uType: UnitType, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canBuildAddon(thisUnit, false) )
//         return Ok(false);
//
//       if ( !Broodwar->canMake(uType, thisUnit) )
//         return Ok(false);
//
//       if ( !uType.isAddon() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       if ( !Broodwar->canBuildHere(thisUnit->getTilePosition(), uType, thisUnit) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN TRAIN --------------------------------------------------
//     pub fn canTrain(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getType().producesLarva() )
//       {
//         if ( !thisUnit->isConstructing() && thisUnit->isCompleted() )
//           return Broodwar->setLastError();
//         Unitset larvae( thisUnit->getLarva() );
//         for (Unit larva : larvae)
//         {
//           if ( !larva->isConstructing() && larva->isCompleted() && canCommand(larva) )
//             return Broodwar->setLastError();
//         }
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       }
//
//       if ( thisUnit->isConstructing() ||
//            !thisUnit->isCompleted()   ||
//            thisUnit->isLifted() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !thisUnit->getType().canProduce() &&
//            thisUnit->getType() != UnitTypes::Enum::Terran_Nuclear_Silo &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Hydralisk &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Mutalisk &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Creep_Colony &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Spire &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Larva )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canTrain(thisUnit: Unit, uType: UnitType, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canTrain(thisUnit, false) )
//         return Ok(false);
//
//       if ( thisUnit->getType().producesLarva() )
//       {
//         if ( uType.whatBuilds().first == UnitTypes::Zerg_Larva )
//         {
//           bool foundCommandableLarva = false;
//           Unitset larvae( thisUnit->getLarva() );
//           for (Unit larva : larvae)
//           {
//             if ( canTrain(larva, true) )
//             {
//               foundCommandableLarva = true;
//               thisUnit = larva;
//               break;
//             }
//           }
//           if (!foundCommandableLarva)
//             return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//         }
//         else if ( thisUnit->isConstructing() ||
//                   !thisUnit->isCompleted() )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//       }
//
//       if ( !Broodwar->canMake(uType, thisUnit) )
//         return Ok(false);
//
//       if ( uType.isAddon() || ( uType.isBuilding() && !thisUnit->getType().isBuilding() ) )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( uType == UnitTypes::Enum::Zerg_Larva || uType == UnitTypes::Enum::Zerg_Egg || uType == UnitTypes::Enum::Zerg_Cocoon )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN MORPH --------------------------------------------------
//     pub fn canMorph(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getType().producesLarva() )
//       {
//         if ( !thisUnit->isConstructing() && thisUnit->isCompleted() && ( !thisUnit->getType().isBuilding() || thisUnit->isIdle() ) )
//           return Broodwar->setLastError();
//         Unitset larvae( thisUnit->getLarva() );
//         for (Unit larva : larvae)
//         {
//           if ( !larva->isConstructing() && larva->isCompleted() && canCommand(larva) )
//             return Broodwar->setLastError();
//         }
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       }
//
//       if ( thisUnit->isConstructing() ||
//            !thisUnit->isCompleted()   ||
//            (thisUnit->getType().isBuilding() && !thisUnit->isIdle()) )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->getType() != UnitTypes::Enum::Zerg_Hydralisk &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Mutalisk &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Creep_Colony &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Spire &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Hatchery &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Lair &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Hive &&
//            thisUnit->getType() != UnitTypes::Enum::Zerg_Larva )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canMorph(thisUnit: Unit, uType: UnitType, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canMorph(thisUnit, false) )
//         return Ok(false);
//
//       if ( thisUnit->getType().producesLarva() )
//       {
//         if ( uType.whatBuilds().first == UnitTypes::Zerg_Larva )
//         {
//           bool foundCommandableLarva = false;
//           Unitset larvae( thisUnit->getLarva() );
//           for (Unit larva : larvae)
//           {
//             if ( canMorph(larva, true) )
//             {
//               foundCommandableLarva = true;
//               thisUnit = larva;
//               break;
//             }
//           }
//           if (!foundCommandableLarva)
//             return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//         }
//         else if ( thisUnit->isConstructing() ||
//                   !thisUnit->isCompleted()   ||
//                   (thisUnit->getType().isBuilding() && !thisUnit->isIdle()) )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//       }
//
//       if ( !Broodwar->canMake(uType, thisUnit) )
//         return Ok(false);
//       if ( uType == UnitTypes::Enum::Zerg_Larva || uType == UnitTypes::Enum::Zerg_Egg || uType == UnitTypes::Enum::Zerg_Cocoon )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN RESEARCH -----------------------------------------------
//     pub fn canResearch(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->isLifted() || !thisUnit->isIdle() || !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     pub fn canResearch(thisUnit: Unit, type: TechType, bool checkCanIssueCommandType = true) -> BwResult<bool> {
//       // Error checking
//       if ( !Broodwar->self() )
//         return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//       if ( thisUnit )
//       {
//         if (thisUnit->getPlayer() != Broodwar->self())
//           return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//         if (!thisUnit->getType().isSuccessorOf(type.whatResearches()))
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//         if ( checkCanIssueCommandType && ( thisUnit->isLifted() || !thisUnit->isIdle() || !thisUnit->isCompleted() ) )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//       }
//       if (Broodwar->self()->isResearching(type))
//         return Broodwar->setLastError(Errors::Currently_Researching);
//
//       if (Broodwar->self()->hasResearched(type))
//         return Broodwar->setLastError(Errors::Already_Researched);
//
//       if ( !Broodwar->self()->isResearchAvailable(type) )
//         return Broodwar->setLastError(Errors::Access_Denied);
//
//       if (Broodwar->self()->minerals() < type.mineralPrice())
//         return Broodwar->setLastError(Errors::Insufficient_Minerals);
//
//       if (Broodwar->self()->gas() < type.gasPrice())
//         return Broodwar->setLastError(Errors::Insufficient_Gas);
//
//       if (!Broodwar->self()->hasUnitTypeRequirement(type.requiredUnit()))
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       return Broodwar->setLastError();
//     }
//     //------------------------------------------- CAN UPGRADE ------------------------------------------------
//     pub fn canUpgrade(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->isLifted() || !thisUnit->isIdle() || !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     pub fn canUpgrade(thisUnit: Unit, type: UpgradeType, bool checkCanIssueCommandType = true) -> BwResult<bool> {
//       Player self = Broodwar->self();
//       if ( !self )
//         return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//       if ( thisUnit )
//       {
//         if (thisUnit->getPlayer() != self)
//           return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//         if (!thisUnit->getType().isSuccessorOf(type.whatUpgrades()))
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//         if ( checkCanIssueCommandType && ( thisUnit->isLifted() || !thisUnit->isIdle() || !thisUnit->isCompleted() ) )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//       }
//       int nextLvl = self->getUpgradeLevel(type) + 1;
//
//       if (!self->hasUnitTypeRequirement(type.whatUpgrades()))
//         return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//
//       if (!self->hasUnitTypeRequirement(type.whatsRequired(nextLvl)))
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       if (self->isUpgrading(type))
//         return Broodwar->setLastError(Errors::Currently_Upgrading);
//
//       if ( self->getUpgradeLevel(type) >= self->getMaxUpgradeLevel(type) )
//         return Broodwar->setLastError(Errors::Fully_Upgraded);
//
//       if ( self->minerals() < type.mineralPrice(nextLvl) )
//         return Broodwar->setLastError(Errors::Insufficient_Minerals);
//
//       if ( self->gas() < type.gasPrice(nextLvl) )
//         return Broodwar->setLastError(Errors::Insufficient_Gas);
//
//       return Broodwar->setLastError();
//     }
//     //------------------------------------------- CAN SET RALLY POINT ----------------------------------------
//     pub fn canSetRallyPoint(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canSetRallyPosition(thisUnit, false) && !canSetRallyUnit(thisUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canSetRallyPoint(thisUnit: Unit, target: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (target == nullptr)
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//       if (!canSetRallyUnit(thisUnit, target, checkCanTargetUnit, checkCanIssueCommandType, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canSetRallyPoint(thisUnit: Unit,  bool: Position checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (!canSetRallyPosition(thisUnit, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN SET RALLY POSITION -------------------------------------
//     pub fn canSetRallyPosition(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().canProduce() || !thisUnit->getType().isBuilding() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isLifted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN SET RALLY UNIT -----------------------------------------
//     pub fn canSetRallyUnit(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().canProduce() || !thisUnit->getType().isBuilding() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isLifted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canSetRallyUnit(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit, bool checkCanIssueCommandType, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canSetRallyUnit(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN MOVE ---------------------------------------------------
//     pub fn canMove(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() )
//       {
//         if ( !thisUnit->isInterruptible() )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//         if ( !thisUnit->getType().canMove() )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//         if ( thisUnit->isBurrowed() )
//           return Broodwar->setLastError(Errors::Incompatible_State);
//         if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//         if ( thisUnit->getType() == UnitTypes::Zerg_Larva )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       }
//       else
//       {
//         if ( !thisUnit->getType().isFlyingBuilding() )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//         if ( !thisUnit->isLifted() )
//           return Broodwar->setLastError(Errors::Incompatible_State);
//       }
//
//       if (!thisUnit->isCompleted())
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canMoveGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().canMove() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( !thisUnit->isCompleted() && !thisUnit->isMorphing() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN PATROL -------------------------------------------------
//     pub fn canPatrol(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canMove(thisUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canPatrolGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canMoveGrouped(thisUnit, false, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN FOLLOW -------------------------------------------------
//     pub fn canFollow(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canMove(thisUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canFollow(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canFollow(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       if ( targetUnit == thisUnit )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN GATHER -------------------------------------------------
//     pub fn canGather(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !thisUnit->getType().isWorker() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->getPowerUp() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isBurrowed() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     pub fn canGather(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canGather(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       UnitType uType = targetUnit->getType();
//       if ( !uType.isResourceContainer() || uType == UnitTypes::Resource_Vespene_Geyser )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       if ( !targetUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       if ( !thisUnit->hasPath(targetUnit->getPosition()) )
//         return Broodwar->setLastError(Errors::Unreachable_Location);
//
//       if ( uType.isRefinery() && targetUnit->getPlayer() != Broodwar->self() )
//         return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN RETURN CARGO -------------------------------------------
//     pub fn canReturnCargo(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !thisUnit->getType().isWorker() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( !thisUnit->isCarryingGas() && !thisUnit->isCarryingMinerals() )
//         return Broodwar->setLastError(Errors::Insufficient_Ammo);
//       if ( thisUnit->isBurrowed() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN HOLD POSITION ------------------------------------------
//     pub fn canHoldPosition(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() )
//       {
//         if ( !thisUnit->getType().canMove() )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//         if ( thisUnit->isBurrowed() && thisUnit->getType() != UnitTypes::Zerg_Lurker )
//           return Broodwar->setLastError(Errors::Incompatible_State);
//         if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//           return Broodwar->setLastError(Errors::Unit_Busy);
//         if ( thisUnit->getType() == UnitTypes::Zerg_Larva )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       }
//       else
//       {
//         if ( !thisUnit->getType().isFlyingBuilding() )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//         if ( !thisUnit->isLifted() )
//           return Broodwar->setLastError(Errors::Incompatible_State);
//       }
//
//       if (!thisUnit->isCompleted())
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN STOP ---------------------------------------------------
//     pub fn canStop(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->isBurrowed() && thisUnit->getType() != UnitTypes::Zerg_Lurker )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->getType().isBuilding() && !thisUnit->isLifted() &&
//            thisUnit->getType() != UnitTypes::Protoss_Photon_Cannon &&
//            thisUnit->getType() != UnitTypes::Zerg_Sunken_Colony &&
//            thisUnit->getType() != UnitTypes::Zerg_Spore_Colony &&
//            thisUnit->getType() != UnitTypes::Terran_Missile_Turret )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN REPAIR -------------------------------------------------
//     pub fn canRepair(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->getType() != BWAPI::UnitTypes::Terran_SCV )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     pub fn canRepair(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canRepair(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       UnitType targType = targetUnit->getType();
//       if ( targType.getRace() != BWAPI::Races::Terran || !targType.isMechanical() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( targetUnit->getHitPoints() == targType.maxHitPoints() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( !targetUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( targetUnit == thisUnit )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN BURROW -------------------------------------------------
//     pub fn canBurrow(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canUseTechWithoutTarget(thisUnit, BWAPI::TechTypes::Burrowing, true, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN UNBURROW -----------------------------------------------
//     pub fn canUnburrow(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBurrowable() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( !thisUnit->isBurrowed() || thisUnit->getOrder() == Orders::Unburrowing )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CLOAK --------------------------------------------------
//     pub fn canCloak(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canUseTechWithoutTarget(thisUnit, thisUnit->getType().cloakingTech(), true, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN DECLOAK ------------------------------------------------
//     pub fn canDecloak(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getType().cloakingTech() == TechTypes::None )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->getSecondaryOrder() != Orders::Cloak )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN SIEGE --------------------------------------------------
//     pub fn canSiege(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canUseTechWithoutTarget(thisUnit, BWAPI::TechTypes::Tank_Siege_Mode, true, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN UNSIEGE ------------------------------------------------
//     pub fn canUnsiege(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->isSieged() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->getOrder() == Orders::Sieging || thisUnit->getOrder() == Orders::Unsieging )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN LIFT ---------------------------------------------------
//     pub fn canLift(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isFlyingBuilding() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( thisUnit->isLifted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( !thisUnit->isIdle() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN LAND ---------------------------------------------------
//     pub fn canLand(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isFlyingBuilding() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( !thisUnit->isLifted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canLand(thisUnit: Unit, target: TilePosition, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if (checkCanIssueCommandType && !canLand(thisUnit, checkCommandibility) )
//         return Ok(false);
//
//       if ( !canBuildHere(nullptr, target, thisUnit->getType(), true) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN LOAD ---------------------------------------------------
//     pub fn canLoad(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->getType() == UnitTypes::Zerg_Overlord && Broodwar->self()->getUpgradeLevel(UpgradeTypes::Ventral_Sacs) == 0 )
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//       if ( thisUnit->isBurrowed() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->getOrder() == Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( thisUnit->getType() == UnitTypes::Zerg_Larva )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canLoad(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canLoad(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       //target must also be owned by self
//       if (targetUnit->getPlayer() != Broodwar->self())
//         return Broodwar->setLastError(Errors::Unit_Not_Owned);
//
//       if ( targetUnit->isLoaded() || !targetUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       // verify upgrade for Zerg Overlord
//       if ( targetUnit->getType() == UnitTypes::Zerg_Overlord && Broodwar->self()->getUpgradeLevel(UpgradeTypes::Ventral_Sacs) == 0 )
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       int thisUnitSpaceProvided = thisUnit->getType().spaceProvided();
//       int targetSpaceProvided = targetUnit->getType().spaceProvided();
//       if ( thisUnitSpaceProvided <= 0 && targetSpaceProvided <= 0 )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       const BWAPI::Unit unitToBeLoaded = ( thisUnitSpaceProvided > 0 ? targetUnit : thisUnit );
//       if ( unitToBeLoaded->getType().canMove() == false || unitToBeLoaded->getType().isFlyer() || unitToBeLoaded->getType().spaceRequired() > 8 )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//       if ( !unitToBeLoaded->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( unitToBeLoaded->isBurrowed() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       const BWAPI::Unit unitThatLoads = ( thisUnitSpaceProvided > 0 ? thisUnit : targetUnit );
//       if ( unitThatLoads->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       if ( unitThatLoads->getType() == UnitTypes::Terran_Bunker )
//       {
//         if ( !unitToBeLoaded->getType().isOrganic() || unitToBeLoaded->getType().getRace() != Races::Terran )
//           return Broodwar->setLastError(Errors::Incompatible_UnitType);
//         if ( !unitToBeLoaded->hasPath(unitThatLoads->getPosition()) )
//           return Broodwar->setLastError(Errors::Unreachable_Location);
//       }
//
//       int freeSpace = ( thisUnitSpaceProvided > 0 ? thisUnitSpaceProvided : targetSpaceProvided );
//       Unitset loadedUnits = unitThatLoads->getLoadedUnits();
//       for (Unit u : loadedUnits)
//       {
//         const int requiredSpace = u->getType().spaceRequired();
//         if ( requiredSpace > 0 && requiredSpace < 8 )
//           freeSpace -= requiredSpace;
//       }
//       if ( unitToBeLoaded->getType().spaceRequired() > freeSpace )
//         return Broodwar->setLastError(Errors::Insufficient_Space);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN UNLOAD -------------------------------------------------
//     pub fn canUnloadWithOrWithoutTarget(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//
//       if ( thisUnit->getLoadedUnits().size() == 0 )
//         return Broodwar->setLastError(Errors::Unit_Does_Not_Exist);
//
//       // Check overlord tech
//       if ( thisUnit->getType() == UnitTypes::Zerg_Overlord && Broodwar->self()->getUpgradeLevel(UpgradeTypes::Ventral_Sacs) == 0)
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       if ( thisUnit->getType().spaceProvided() <= 0 )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canUnloadAtPosition(thisUnit: Unit, targDropPos: Position, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUnloadWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( thisUnit->getType() != UnitTypes::Terran_Bunker )
//       {
//         if ( WalkPosition(targDropPos.x/8, targDropPos.y/8).isValid() == false )
//           return Broodwar->setLastError(Errors::Invalid_Tile_Position);
//         else if ( !Broodwar->isWalkable(targDropPos.x/8, targDropPos.y/8) )
//           return Broodwar->setLastError(Errors::Unreachable_Location);
//       }
//
//       return Ok(true);
//     }
//     pub fn canUnload(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       return canUnloadAtPosition(thisUnit, thisUnit->getPosition(), true, checkCommandibility);
//     }
//     pub fn canUnload(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit = true, bool checkPosition = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUnloadWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkPosition && !canUnloadAtPosition(thisUnit, thisUnit->getPosition(), false, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       if ( !targetUnit->isLoaded() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       if ( targetUnit->getTransport() != thisUnit )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN UNLOAD ALL ---------------------------------------------
//     pub fn canUnloadAll(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       return canUnloadAtPosition(thisUnit, thisUnit->getPosition(), true, checkCommandibility);
//     }
//     //------------------------------------------- CAN UNLOAD ALL POSITION ------------------------------------
//     pub fn canUnloadAllPosition(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool>
//     {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canUnloadWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( thisUnit->getType() == UnitTypes::Terran_Bunker )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canUnloadAllPosition(thisUnit: Unit, targDropPos: Position, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUnloadAllPosition(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canUnloadAtPosition(thisUnit, targDropPos, false, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN RIGHT CLICK --------------------------------------------
//     pub fn canRightClick(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !canRightClickPosition(thisUnit, false) && !canRightClickUnit(thisUnit, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canRightClick(thisUnit: Unit,  bool: Position checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (!canRightClickPosition(thisUnit, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canRightClick(thisUnit: Unit, target: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (target == nullptr)
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//       if (!canRightClickUnit(thisUnit, target, checkCanTargetUnit, checkCanIssueCommandType, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canRightClickGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canRightClickPositionGrouped(thisUnit, false, false) && !canRightClickUnitGrouped(thisUnit, false, false) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canRightClickGrouped(thisUnit: Unit,  bool: Position checkCanTargetUnit = true, bool checkCanIssueCommandTypeGrouped = true, bool checkCommandibilityGrouped = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false))
//         return Ok(false);
//
//       if (!canRightClickPositionGrouped(thisUnit, false, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canRightClickGrouped(thisUnit: Unit, target: Unit, bool checkCanTargetUnit = true, bool checkCanIssueCommandTypeGrouped = true, bool checkCommandibilityGrouped = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false))
//         return Ok(false);
//
//       if (target == nullptr)
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//       if (!canRightClickUnitGrouped(thisUnit, target, checkCanTargetUnit, checkCanIssueCommandTypeGrouped, false, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN RIGHT CLICK POSITION -----------------------------------
//     pub fn canRightClickPosition(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !canMove(thisUnit, false) && !canSetRallyPosition(thisUnit, false) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canRightClickPositionGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !canMoveGrouped(thisUnit, false, false) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN RIGHT CLICK UNIT ---------------------------------------
//     pub fn canRightClickUnit(thisUnit: Unit, checkCommandibility: bool) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !canFollow(thisUnit, false) &&
//            !canAttackUnit(thisUnit, false) &&
//            !canLoad(thisUnit, false) &&
//            !canSetRallyUnit(thisUnit, false) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canRightClickUnit(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit, bool checkCanIssueCommandType, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canRightClickUnit(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       if ( !targetUnit->getPlayer()->isNeutral() && thisUnit->getPlayer()->isEnemy(targetUnit->getPlayer()) &&
//            !canAttackUnit(thisUnit, targetUnit, false, true, false) )
//         return Ok(false);
//
//       if ( !canFollow(thisUnit, targetUnit, false, true, false) &&
//            !canLoad(thisUnit, targetUnit, false, true, false) &&
//            !canSetRallyUnit(thisUnit, targetUnit, false, true, false) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canRightClickUnitGrouped(thisUnit: Unit, checkCommandibilityGrouped: bool, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !canFollow(thisUnit, false) &&
//            !canAttackUnitGrouped(thisUnit, false, false) &&
//            !canLoad(thisUnit, false) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canRightClickUnitGrouped(thisUnit: Unit, targetUnit: Unit, bool checkCanTargetUnit, bool checkCanIssueCommandType, bool checkCommandibilityGrouped, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canRightClickUnitGrouped(thisUnit, false, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       if ( !targetUnit->getPlayer()->isNeutral() && thisUnit->getPlayer()->isEnemy(targetUnit->getPlayer()) &&
//            !canAttackUnitGrouped(thisUnit, targetUnit, false, true, false, false) )
//         return Ok(false);
//
//       if ( !canFollow(thisUnit, targetUnit, false, true, false) &&
//            !canLoad(thisUnit, targetUnit, false, true, false) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN HALT CONSTRUCTION --------------------------------------
//     pub fn canHaltConstruction(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool>
//     {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getOrder() != Orders::ConstructingBuilding )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL CONSTRUCTION ------------------------------------
//     pub fn canCancelConstruction(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       if ( thisUnit->isCompleted() || (thisUnit->getType() == UnitTypes::Zerg_Nydus_Canal && thisUnit->getNydusExit()) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL ADDON -------------------------------------------
//     pub fn canCancelAddon(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getAddon() || thisUnit->getAddon()->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL TRAIN -------------------------------------------
//     pub fn canCancelTrain(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->isTraining() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL TRAIN SLOT --------------------------------------
//     pub fn canCancelTrainSlot(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       return canCancelTrain(thisUnit, checkCommandibility);
//     }
//     pub fn canCancelTrainSlot(thisUnit: Unit, slot: int, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canCancelTrainSlot(thisUnit, false) )
//         return Ok(false);
//
//       if ( !thisUnit->isTraining() || (thisUnit->getTrainingQueue().size() <= (unsigned int)slot && slot >= 0) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL MORPH -------------------------------------------
//     pub fn canCancelMorph(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->isMorphing() || (!thisUnit->isCompleted() && thisUnit->getType() == UnitTypes::Zerg_Nydus_Canal && thisUnit->getNydusExit()) )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL RESEARCH ----------------------------------------
//     pub fn canCancelResearch(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getOrder() != Orders::ResearchTech )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN CANCEL UPGRADE -----------------------------------------
//     pub fn canCancelUpgrade(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( thisUnit->getOrder() != Orders::Upgrade )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN USE TECH -----------------------------------------------
//     pub fn canUseTechWithOrWithoutTarget(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isBuilding() && !thisUnit->isInterruptible() )
//         return Broodwar->setLastError(Errors::Unit_Busy);
//       if ( !thisUnit->isCompleted() )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//       if ( thisUnit->isHallucination() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       return Ok(true);
//     }
//     pub fn canUseTechWithOrWithoutTarget(thisUnit: Unit, :: BWAPITechType tech, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUseTechWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       // researched check
//       if ( !thisUnit->getType().isHero() && !Broodwar->self()->hasResearched(tech) && thisUnit->getType() != UnitTypes::Zerg_Lurker )
//         return Broodwar->setLastError(Errors::Insufficient_Tech);
//
//       // energy check
//       if ( thisUnit->getEnergy() < tech.energyCost() )
//         return Broodwar->setLastError(Errors::Insufficient_Energy);
//
//       // unit check
//       if ( tech != TechTypes::Burrowing && !tech.whatUses().contains(thisUnit->getType()) )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       switch (tech)
//       {
//         case TechTypes::Enum::Spider_Mines:
//           if ( thisUnit->getSpiderMineCount() <= 0 )
//             return Broodwar->setLastError(Errors::Insufficient_Ammo);
//           return Ok(true);
//
//         case TechTypes::Enum::Tank_Siege_Mode:
//           if ( thisUnit->isSieged() )
//             return Broodwar->setLastError(Errors::Incompatible_State);
//           if ( thisUnit->getOrder() == Orders::Sieging || thisUnit->getOrder() == Orders::Unsieging )
//             return Broodwar->setLastError(Errors::Unit_Busy);
//           return Ok(true);
//
//         case TechTypes::Enum::Cloaking_Field:
//         case TechTypes::Enum::Personnel_Cloaking:
//           if ( thisUnit->getSecondaryOrder() == Orders::Cloak )
//             return Broodwar->setLastError(Errors::Incompatible_State);
//           return Ok(true);
//
//         case TechTypes::Enum::Burrowing:
//           if ( !thisUnit->getType().isBurrowable() )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           if ( thisUnit->isBurrowed() || thisUnit->getOrder() == Orders::Burrowing || thisUnit->getOrder() == Orders::Unburrowing )
//             return Broodwar->setLastError(Errors::Incompatible_State);
//           return Ok(true);
//
//         case TechTypes::Enum::None:
//           return Broodwar->setLastError(Errors::Incompatible_TechType);
//
//         case TechTypes::Enum::Nuclear_Strike:
//           if ( thisUnit->getPlayer()->completedUnitCount(UnitTypes::Terran_Nuclear_Missile) == 0 )
//             return Broodwar->setLastError(Errors::Insufficient_Ammo);
//           return Ok(true);
//
//         case TechTypes::Enum::Unknown:
//           return Broodwar->setLastError(Errors::Incompatible_TechType);
//       }
//
//       return Ok(true);
//     }
//     pub fn canUseTech(thisUnit: Unit, :: BWAPITechType tech, Position target, bool checkCanTargetUnit = true, bool checkTargetsType = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (!canUseTechPosition(thisUnit, tech, target, checkTargetsType, checkCanIssueCommandType, false))
//         return Ok(false);
//
//       return Ok(true);
//     }
//     pub fn canUseTech(thisUnit: Unit, :: BWAPITechType tech, Unit target = nullptr, bool checkCanTargetUnit = true, bool checkTargetsType = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if (!checkCommandibility)
//         Broodwar->setLastError();
//       else if (!canCommand(thisUnit))
//         return Ok(false);
//
//       if (target == nullptr)
//       {
//         if (!canUseTechWithoutTarget(thisUnit, tech, checkCanIssueCommandType, false))
//           return Ok(false);
//       }
//       else
//       {
//         if (!canUseTechUnit(thisUnit, tech, target, checkCanTargetUnit, checkTargetsType, checkCanIssueCommandType, false))
//           return Ok(false);
//       }
//
//       return Ok(true);
//     }
//     pub fn canUseTechWithoutTarget(thisUnit: Unit, :: BWAPITechType tech, bool checkCanIssueCommandType, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUseTechWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canUseTechWithOrWithoutTarget(thisUnit, tech, false, false) )
//         return Ok(false);
//       if ( tech.targetsUnit() || tech.targetsPosition() || tech == TechTypes::None || tech == TechTypes::Unknown || tech == TechTypes::Lurker_Aspect)
//         return Broodwar->setLastError(Errors::Incompatible_TechType);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN USE TECH UNIT ------------------------------------------
//     pub fn canUseTechUnit(thisUnit: Unit, :: BWAPITechType tech, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUseTechWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canUseTechWithOrWithoutTarget(thisUnit, tech, false, false) )
//         return Ok(false);
//       if ( !tech.targetsUnit() )
//         return Broodwar->setLastError(Errors::Incompatible_TechType);
//
//       return Ok(true);
//     }
//     pub fn canUseTechUnit(thisUnit: Unit, :: BWAPITechType tech, Unit targetUnit, bool checkCanTargetUnit, bool checkTargetsUnits, bool checkCanIssueCommandType, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUseTechWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkTargetsUnits && !canUseTechUnit(thisUnit, tech, false, false) )
//         return Ok(false);
//
//       if ( checkCanTargetUnit && !canTargetUnit(thisUnit, targetUnit, false) )
//         return Ok(false);
//
//       UnitType targetType = targetUnit->getType();
//
//       switch (tech)
//       {
//         case TechTypes::Enum::Archon_Warp:
//           if ( targetType != UnitTypes::Protoss_High_Templar )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           if ( targetUnit->getPlayer() != thisUnit->getPlayer() )
//             return Broodwar->setLastError(Errors::Unit_Not_Owned);
//           break;
//
//         case TechTypes::Enum::Dark_Archon_Meld:
//           if ( targetType != UnitTypes::Protoss_Dark_Templar )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           if ( targetUnit->getPlayer() != thisUnit->getPlayer() )
//             return Broodwar->setLastError(Errors::Unit_Not_Owned);
//           break;
//
//         case TechTypes::Enum::Consume:
//           if ( targetUnit->getPlayer() != thisUnit->getPlayer() )
//             return Broodwar->setLastError(Errors::Unit_Not_Owned);
//           if ( targetType.getRace() != Races::Zerg || targetType == UnitTypes::Zerg_Larva )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           break;
//
//         case TechTypes::Enum::Spawn_Broodlings:
//           if ( ( !targetType.isOrganic() && !targetType.isMechanical() ) ||
//                targetType.isRobotic() ||
//                targetType.isFlyer() )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           break;
//
//         case TechTypes::Enum::Lockdown:
//           if ( !targetType.isMechanical() )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           break;
//
//         case TechTypes::Enum::Healing:
//           if ( targetUnit->getHitPoints() == targetType.maxHitPoints() )
//             return Broodwar->setLastError(Errors::Incompatible_State);
//           if ( !targetType.isOrganic() ||
//                targetType.isFlyer() )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           if ( !targetUnit->getPlayer()->isNeutral() && thisUnit->getPlayer()->isEnemy(targetUnit->getPlayer()) )
//             return Broodwar->setLastError(Errors::Invalid_Parameter);
//           break;
//
//         case TechTypes::Enum::Mind_Control:
//           if ( targetUnit->getPlayer() == thisUnit->getPlayer() )
//             return Broodwar->setLastError(Errors::Invalid_Parameter);
//           if ( targetType == UnitTypes::Protoss_Interceptor ||
//                targetType == UnitTypes::Terran_Vulture_Spider_Mine ||
//                targetType == UnitTypes::Zerg_Lurker_Egg ||
//                targetType == UnitTypes::Zerg_Cocoon ||
//                targetType == UnitTypes::Zerg_Larva ||
//                targetType == UnitTypes::Zerg_Egg )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           break;
//
//         case TechTypes::Enum::Feedback:
//           if ( !targetType.isSpellcaster() )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           break;
//
//         case TechTypes::Enum::Infestation:
//           if ( targetType != UnitTypes::Terran_Command_Center ||
//                targetUnit->getHitPoints() >= 750 || targetUnit->getHitPoints() <= 0 )
//             return Broodwar->setLastError(Errors::Invalid_Parameter);
//           break;
//       }
//
//       switch (tech)
//       {
//         case TechTypes::Enum::Archon_Warp:
//         case TechTypes::Enum::Dark_Archon_Meld:
//           if ( !thisUnit->hasPath(targetUnit->getPosition()) )
//             return Broodwar->setLastError(Errors::Unreachable_Location);
//           if ( targetUnit->isHallucination() )
//             return Broodwar->setLastError(Errors::Invalid_Parameter);
//           if ( targetUnit->isMaelstrommed() )
//             return Broodwar->setLastError(Errors::Incompatible_State);
//           // Fall through (don't break).
//         case TechTypes::Enum::Parasite:
//         case TechTypes::Enum::Irradiate:
//         case TechTypes::Enum::Optical_Flare:
//         case TechTypes::Enum::Spawn_Broodlings:
//         case TechTypes::Enum::Lockdown:
//         case TechTypes::Enum::Defensive_Matrix:
//         case TechTypes::Enum::Hallucination:
//         case TechTypes::Enum::Healing:
//         case TechTypes::Enum::Restoration:
//         case TechTypes::Enum::Mind_Control:
//         case TechTypes::Enum::Consume:
//         case TechTypes::Enum::Feedback:
//         case TechTypes::Enum::Yamato_Gun:
//           if ( targetUnit->isStasised() )
//             return Broodwar->setLastError(Errors::Incompatible_State);
//           break;
//       }
//
//       switch (tech)
//       {
//         case TechTypes::Enum::Yamato_Gun:
//           if ( targetUnit->isInvincible() )
//             return Broodwar->setLastError(Errors::Invalid_Parameter);
//           break;
//
//         case TechTypes::Enum::Parasite:
//         case TechTypes::Enum::Irradiate:
//         case TechTypes::Enum::Optical_Flare:
//         case TechTypes::Enum::Spawn_Broodlings:
//         case TechTypes::Enum::Lockdown:
//         case TechTypes::Enum::Defensive_Matrix:
//         case TechTypes::Enum::Hallucination:
//         case TechTypes::Enum::Healing:
//         case TechTypes::Enum::Restoration:
//         case TechTypes::Enum::Mind_Control:
//           if ( targetUnit->isInvincible() )
//             return Broodwar->setLastError(Errors::Invalid_Parameter);
//           // Fall through (don't break).
//         case TechTypes::Enum::Consume:
//         case TechTypes::Enum::Feedback:
//           if ( targetType.isBuilding() )
//             return Broodwar->setLastError(Errors::Incompatible_UnitType);
//           break;
//       }
//
//       if ( targetUnit == thisUnit )
//         return Broodwar->setLastError(Errors::Invalid_Parameter);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN USE TECH POSITION --------------------------------------
//     pub fn canUseTechPosition(thisUnit: Unit, :: BWAPITechType tech, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUseTechWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( !canUseTechWithOrWithoutTarget(thisUnit, tech, false, false) )
//         return Ok(false);
//       if ( !tech.targetsPosition() )
//         return Broodwar->setLastError(Errors::Incompatible_TechType);
//
//       return Ok(true);
//     }
//     pub fn canUseTechPosition(thisUnit: Unit, :: BWAPITechType tech, Position target, bool checkTargetsPositions, bool checkCanIssueCommandType, bool checkCommandibility) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCanIssueCommandType && !canUseTechWithOrWithoutTarget(thisUnit, false) )
//         return Ok(false);
//
//       if ( checkTargetsPositions && !canUseTechPosition(thisUnit, tech, false, false) )
//         return Ok(false);
//
//       if ( tech == TechTypes::Enum::Spider_Mines && !thisUnit->hasPath(target) )
//         return Broodwar->setLastError(Errors::Unreachable_Location);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN PLACE COP ----------------------------------------------
//     pub fn canPlaceCOP(thisUnit: Unit, checkCommandibility: bool = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( !thisUnit->getType().isFlagBeacon() )
//         return Broodwar->setLastError(Errors::Incompatible_UnitType);
//
//       if ( static_cast<UnitImpl*>(thisUnit)->self->buttonset == 228 || thisUnit->getOrder() != BWAPI::Orders::CTFCOPInit )
//         return Broodwar->setLastError(Errors::Incompatible_State);
//
//       return Ok(true);
//     }
//     pub fn canPlaceCOP(thisUnit: Unit, target: TilePosition, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if (checkCanIssueCommandType && !canPlaceCOP(thisUnit, checkCommandibility) )
//         return Ok(false);
//
//       if ( !canBuildHere(thisUnit, target, thisUnit->getType(), true) )
//         return Ok(false);
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN ISSUE COMMAND TYPE -------------------------------------
//     pub fn canIssueCommandType(thisUnit: Unit, :: BWAPIUnitCommandType ct, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       switch (ct)
//       {
//         case UnitCommandTypes::Enum::Attack_Move:
//           return canAttackMove(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Attack_Unit:
//           return canAttackUnit(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Build:
//           return canBuild(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Build_Addon:
//           return canBuildAddon(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Train:
//           return canTrain(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Morph:
//           return canMorph(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Research:
//           return canResearch(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Upgrade:
//           return canUpgrade(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Position:
//           return canSetRallyPosition(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Unit:
//           return canSetRallyUnit(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Move:
//           return canMove(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Patrol:
//           return canPatrol(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Hold_Position:
//           return canHoldPosition(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Stop:
//           return canStop(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Follow:
//           return canFollow(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Gather:
//           return canGather(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Return_Cargo:
//           return canReturnCargo(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Repair:
//           return canRepair(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Burrow:
//           return canBurrow(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unburrow:
//           return canUnburrow(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cloak:
//           return canCloak(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Decloak:
//           return canDecloak(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Siege:
//           return canSiege(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unsiege:
//           return canUnsiege(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Lift:
//           return canLift(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Land:
//           return canLand(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Load:
//           return canLoad(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unload:
//           return canUnload(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unload_All:
//           return canUnloadAll(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unload_All_Position:
//           return canUnloadAllPosition(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Right_Click_Position:
//           return canRightClickPosition(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Right_Click_Unit:
//           return canRightClickUnit(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Halt_Construction:
//           return canHaltConstruction(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Construction:
//           return canCancelConstruction(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Addon:
//           return canCancelAddon(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Train:
//           return canCancelTrain(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Train_Slot:
//           return canCancelTrainSlot(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Morph:
//           return canCancelMorph(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Research:
//           return canCancelResearch(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Upgrade:
//           return canCancelUpgrade(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Use_Tech:
//         case UnitCommandTypes::Enum::Use_Tech_Unit:
//         case UnitCommandTypes::Enum::Use_Tech_Position:
//           return canUseTechWithOrWithoutTarget(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Place_COP:
//           return canPlaceCOP(thisUnit, false);
//       }
//
//       return Ok(true);
//     }
//     pub fn canIssueCommandTypeGrouped(thisUnit: Unit, :: BWAPIUnitCommandType ct, bool checkCommandibilityGrouped = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       switch (ct)
//       {
//         case UnitCommandTypes::Enum::Attack_Move:
//           return canAttackMoveGrouped(thisUnit, false, false);
//
//         case UnitCommandTypes::Enum::Attack_Unit:
//           return canAttackUnitGrouped(thisUnit, false, false);
//
//         case UnitCommandTypes::Enum::Build:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Build_Addon:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Train:
//           return canTrain(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Morph:
//           return canMorph(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Research:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Upgrade:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Position:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Unit:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Move:
//           return canMoveGrouped(thisUnit, false, false);
//
//         case UnitCommandTypes::Enum::Patrol:
//           return canPatrolGrouped(thisUnit, false, false);
//
//         case UnitCommandTypes::Enum::Hold_Position:
//           return canHoldPosition(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Stop:
//           return canStop(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Follow:
//           return canFollow(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Gather:
//           return canGather(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Return_Cargo:
//           return canReturnCargo(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Repair:
//           return canRepair(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Burrow:
//           return canBurrow(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unburrow:
//           return canUnburrow(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cloak:
//           return canCloak(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Decloak:
//           return canDecloak(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Siege:
//           return canSiege(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unsiege:
//           return canUnsiege(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Lift:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Land:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Load:
//           return canLoad(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Unload:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Unload_All:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Unload_All_Position:
//           return canUnloadAllPosition(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Right_Click_Position:
//           return canRightClickPositionGrouped(thisUnit, false, false);
//
//         case UnitCommandTypes::Enum::Right_Click_Unit:
//           return canRightClickUnitGrouped(thisUnit, false, false);
//
//         case UnitCommandTypes::Enum::Halt_Construction:
//           return canHaltConstruction(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Construction:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Addon:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Train:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Train_Slot:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Morph:
//           return canCancelMorph(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Cancel_Research:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Upgrade:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Use_Tech:
//         case UnitCommandTypes::Enum::Use_Tech_Unit:
//         case UnitCommandTypes::Enum::Use_Tech_Position:
//           return canUseTechWithOrWithoutTarget(thisUnit, false);
//
//         case UnitCommandTypes::Enum::Place_COP:
//           return Ok(false);
//       }
//
//       return Ok(true);
//     }
//     //------------------------------------------- CAN ISSUE COMMAND ------------------------------------------
//     pub fn canIssueCommand(thisUnit: Unit, c: UnitCommand, bool checkCanUseTechPositionOnPositions = true, bool checkCanUseTechUnitOnUnits = true, bool checkCanBuildUnitType = true, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       BWAPI::UnitCommandType ct = c.type;
//       if ( checkCanIssueCommandType && !canIssueCommandType(thisUnit, ct, false) )
//         return Ok(false);
//
//       switch (ct)
//       {
//         case UnitCommandTypes::Enum::Attack_Move:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Attack_Unit:
//           return canAttackUnit(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Build:
//           return canBuild(thisUnit, c.getUnitType(), BWAPI::TilePosition(c.x, c.y), checkCanBuildUnitType, false, false);
//
//         case UnitCommandTypes::Enum::Build_Addon:
//           return canBuildAddon(thisUnit, c.getUnitType(), false, false);
//
//         case UnitCommandTypes::Enum::Train:
//           return canTrain(thisUnit, c.getUnitType(), false, false);
//
//         case UnitCommandTypes::Enum::Morph:
//           return canMorph(thisUnit, c.getUnitType(), false, false);
//
//         case UnitCommandTypes::Enum::Research:
//           return Broodwar->canResearch(c.getTechType(), thisUnit, false);
//
//         case UnitCommandTypes::Enum::Upgrade:
//           return Broodwar->canUpgrade(c.getUpgradeType(), thisUnit, false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Position:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Set_Rally_Unit:
//           return canSetRallyUnit(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Move:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Patrol:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Hold_Position:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Stop:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Follow:
//           return canFollow(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Gather:
//           return canGather(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Return_Cargo:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Repair:
//           return canRepair(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Burrow:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Unburrow:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cloak:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Decloak:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Siege:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Unsiege:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Lift:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Land:
//           return canLand(thisUnit, BWAPI::TilePosition(c.x, c.y), false, false);
//
//         case UnitCommandTypes::Enum::Load:
//           return canLoad(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Unload:
//           return canUnload(thisUnit, c.target, checkCanTargetUnit, false, false, false);
//
//         case UnitCommandTypes::Enum::Unload_All:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Unload_All_Position:
//           return canUnloadAllPosition(thisUnit, c.getTargetPosition(), false, false);
//
//         case UnitCommandTypes::Enum::Right_Click_Position:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Right_Click_Unit:
//           return canRightClickUnit(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Halt_Construction:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Construction:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Addon:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Train:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Train_Slot:
//           return canCancelTrainSlot(thisUnit, c.extra, false, false);
//
//         case UnitCommandTypes::Enum::Cancel_Morph:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Research:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Upgrade:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Use_Tech:
//           return canUseTechWithoutTarget(thisUnit, c.extra, false, false);
//
//         case UnitCommandTypes::Enum::Use_Tech_Unit:
//           return canUseTechUnit(thisUnit, c.extra, c.target, checkCanTargetUnit, checkCanUseTechUnitOnUnits, false, false);
//
//         case UnitCommandTypes::Enum::Use_Tech_Position:
//           return canUseTechPosition(thisUnit, c.extra, c.getTargetPosition(), checkCanUseTechPositionOnPositions, false, false);
//
//         case UnitCommandTypes::Enum::Place_COP:
//           return canPlaceCOP(thisUnit, BWAPI::TilePosition(c.x, c.y), false, false);
//       }
//
//       return Ok(true);
//     }
//     pub fn canIssueCommandGrouped(thisUnit: Unit, c: UnitCommand, bool checkCanUseTechPositionOnPositions = true, bool checkCanUseTechUnitOnUnits = true, bool checkCanTargetUnit = true, bool checkCanIssueCommandType = true, bool checkCommandibilityGrouped = true, bool checkCommandibility = true) -> BwResult<bool> {
//       if ( !checkCommandibility )
//         Broodwar->setLastError();
//       else if ( !canCommand(thisUnit) )
//         return Ok(false);
//
//       if ( checkCommandibilityGrouped && !canCommandGrouped(thisUnit, false) )
//         return Ok(false);
//
//       BWAPI::UnitCommandType ct = c.type;
//       if ( checkCanIssueCommandType && !canIssueCommandTypeGrouped(thisUnit, ct, false, false) )
//         return Ok(false);
//
//       switch (ct)
//       {
//         case UnitCommandTypes::Enum::Attack_Move:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Attack_Unit:
//           return canAttackUnitGrouped(thisUnit, c.target, checkCanTargetUnit, false, false, false);
//
//         case UnitCommandTypes::Enum::Build:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Build_Addon:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Train:
//           return canTrain(thisUnit, c.getUnitType(), false, false);
//
//         case UnitCommandTypes::Enum::Morph:
//           return canMorph(thisUnit, c.getUnitType(), false, false);
//
//         case UnitCommandTypes::Enum::Research:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Upgrade:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Position:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Set_Rally_Unit:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Move:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Patrol:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Hold_Position:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Stop:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Follow:
//           return canFollow(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Gather:
//           return canGather(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Return_Cargo:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Repair:
//           return canRepair(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Burrow:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Unburrow:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cloak:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Decloak:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Siege:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Unsiege:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Lift:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Land:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Load:
//           return canLoad(thisUnit, c.target, checkCanTargetUnit, false, false);
//
//         case UnitCommandTypes::Enum::Unload:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Unload_All:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Unload_All_Position:
//           return canUnloadAllPosition(thisUnit, c.getTargetPosition(), false, false);
//
//         case UnitCommandTypes::Enum::Right_Click_Position:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Right_Click_Unit:
//           return canRightClickUnitGrouped(thisUnit, c.target, checkCanTargetUnit, false, false, false);
//
//         case UnitCommandTypes::Enum::Halt_Construction:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Construction:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Addon:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Train:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Train_Slot:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Morph:
//           return Ok(true);
//
//         case UnitCommandTypes::Enum::Cancel_Research:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Cancel_Upgrade:
//           return Ok(false);
//
//         case UnitCommandTypes::Enum::Use_Tech:
//           return canUseTechWithoutTarget(thisUnit, c.extra, false, false);
//
//         case UnitCommandTypes::Enum::Use_Tech_Unit:
//           return canUseTechUnit(thisUnit, c.extra, c.target, checkCanTargetUnit, checkCanUseTechUnitOnUnits, false, false);
//
//         case UnitCommandTypes::Enum::Use_Tech_Position:
//           return canUseTechPosition(thisUnit, c.extra, c.getTargetPosition(), checkCanUseTechPositionOnPositions, false, false);
//
//         case UnitCommandTypes::Enum::Place_COP:
//           return Ok(false);
//       }
//
//       return Ok(true);
//     }

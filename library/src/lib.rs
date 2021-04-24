#![feature(never_type)]
#[allow(non_snake_case)]
pub mod bw;
pub mod prelude;

use crate::prelude::{AIModule, BoxedAIModule, Event, Game, GAME};
use cxx::CxxString;
use once_cell::sync::OnceCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::pin::Pin;

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    use std::process;
    process::abort();
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn _Unwind_RaiseException() -> ! {
    use std::process;
    process::abort();
}

#[no_mangle]
pub unsafe extern "C" fn gameInit(game: *const std::ffi::c_void) {
    println!("gameInit called: game = {:?}", game);
    *GAME.lock().unwrap() = Game {
        raw: game as *mut ffi::Game,
    };
}

#[allow(non_snake_case)]
#[cxx::bridge]
pub mod ffi {

    #[namespace = "BWAPI"]
    unsafe extern "C++" {
        include!("library/openbw/include/BWAPI.h");
        pub fn BWAPI_getRevision() -> i32;
        pub fn BWAPI_isDebug() -> bool;

        pub type BulletInterface;
        pub type ForceInterface;
        pub type Game;
        pub type PlayerInterface;
        pub type RegionInterface;
        pub type TournamentModule;
        pub type UnitInterface;
        pub type Unit;

        pub type BulletType;
        pub type Color;
        pub type DamageType;
        pub type Error;
        pub type ExplosionType;
        pub type GameType;
        pub type Order;
        pub type PlayerType;
        pub type Race;
        pub type TechType;
        pub type UnitCommandType;
        pub type UnitSizeType;
        pub type UpgradeType;
        pub type WeaponType;

        type Event;

        pub type Forceset;
        pub type Playerset;
        pub type Unitset;

        type Position = crate::bw::position::Position;
        type TilePosition = crate::bw::position::TilePosition;
        type WalkPosition = crate::bw::position::WalkPosition;
        type UnitType = crate::bw::unit_type::UnitType;
    }
    // bool (BWAPI::Game::*)(::BWAPI::TilePosition, ::BWAPI::UnitType, const ::BWAPI::UnitInterface *, bool) const
    // bool (BWAPI::Game::*)(BWAPI::TilePosition, BWAPI::UnitType, BWAPI::Unit, bool)

    // BWAPI::BulletInterface
    // extern "C++" {
    //     unsafe fn exists(raw: *const BulletInterface) -> bool;
    //     unsafe fn getAngle(raw: *const BulletInterface) -> f32;
    //     unsafe fn getID(raw: *const BulletInterface) -> i32;
    //     unsafe fn getPlayer(raw: *const BulletInterface) -> *const PlayerInterface;
    //     unsafe fn getPosition(raw: *const BulletInterface) -> Position;
    //     unsafe fn getRemoveTimer(raw: *const BulletInterface) -> i32;
    //     unsafe fn getSource(raw: *const BulletInterface) -> *const UnitInterface;
    //     unsafe fn getTarget(raw: *const BulletInterface) -> *const UnitInterface;
    //     unsafe fn getTargetPosition(raw: *const BulletInterface) -> Position;
    //     unsafe fn getType(raw: *const BulletInterface) -> BulletType;
    //     unsafe fn getVelocityX(raw: *const BulletInterface) -> f32;
    //     unsafe fn getVelocityY(raw: *const BulletInterface) -> f32;
    //     unsafe fn isVisible(raw: *const BulletInterface, player: *const PlayerInterface) -> bool;
    // }

    // BWAPI::ForceInterface
    // extern "C++" {
    //     unsafe fn getID () -> i32;
    //     unsafe fn getName () -> *const c_char;
    //     unsafe fn getPlayers () -> Playerset;
    // }

    unsafe extern "C++" {
        // pub type UnitIterator;

        // fn next(iter: IteratorBase) -> &UnitIterator;
        // fn UnitIterator_isValid(it: Pin<&mut UnitIterator>) -> bool;
        // unsafe fn UnitIterator_get(it: Pin<&mut UnitIterator>) -> *mut UnitInterface;
        // fn UnitIterator_next(it: Pin<&mut UnitIterator>);
        // fn UnitIterator_size(it: Pin<&mut UnitIterator>) -> usize;

        pub fn Game_getAllUnits(container: &Unitset) -> &CxxVector<Unit>;
        pub fn Unit_getId(unit: &Unit) -> i32;
    }

    // BWAPI::Game
    extern "C++" {
        // methods that need manual shims to C++
        unsafe fn sendText(game: *mut Game, text: &str);

        unsafe fn getFrameCount(self: &Game) -> i32;
        unsafe fn getForces(self: &Game) -> &Forceset;
        unsafe fn getPlayers(self: &Game) -> &Playerset;
        unsafe fn getAllUnits(self: &Game) -> &Unitset;
        unsafe fn allies(self: Pin<&mut Game>) -> Pin<&mut Playerset>;
        unsafe fn canBuildHere(self: Pin<&mut Game>, position: TilePosition, uType: UnitType, builder: *mut UnitInterface, checkExplored: bool) -> bool;
        unsafe fn enemy(self: &Game) -> *mut PlayerInterface;

        // not implemented yet
        // unsafe fn canMake(game: *mut Game, type_: UnitType, builder: *const UnitInterface) -> bool;
        // unsafe fn canResearch(game: *mut Game, type_: TechType, unit: *const UnitInterface, checkCanIssueCommandType: bool) -> bool;
        // unsafe fn canUpgrade(game: *mut Game, type_: UpgradeType, unit: *const UnitInterface, checkCanIssueCommandType: bool) -> bool;
        // unsafe fn countdownTimer(game: *mut Game) -> i32;
        // unsafe fn elapsedTime(game: *mut Game) -> i32;
        // unsafe fn enableFlag(game: *mut Game, flag: i32);
        // unsafe fn enemies(game: *mut Game) -> Playerset;
        // unsafe fn enemy(game: *mut Game) -> *const PlayerInterface;
        // unsafe fn getAllRegions(game: *mut Game) -> Regionset;
        // unsafe fn getAPM(game: *mut Game, includeSelects: bool) -> i32;
        // unsafe fn getAverageFPS(game: *mut Game) -> f32;
        // unsafe fn getBestUnit(game: *mut Game, best: BestUnitFilter, pred: UnitFilter, center: Position, radius: i32) -> *const UnitInterface;
        // unsafe fn getBuildLocation(game: *mut Game, unitType: UnitType, desiredPosition: TilePosition, maxRange: i32, creep: bool) -> TilePosition;
        // unsafe fn getBullets(game: *mut Game) -> Bulletset;
        // unsafe fn getClientVersion(game: *mut Game) -> i32;
        // unsafe fn getClosestUnit(game: *mut Game, center: Position, pred: UnitFilter, radius: i32) -> *const UnitInterface;
        // unsafe fn getClosestUnitInRectangle(game: *mut Game, center: Position, pred: UnitFilter, left: i32, top: i32, right: i32, bottom: i32) -> *const UnitInterface;
        // unsafe fn getDamageFrom(game: *mut Game, fromType: UnitType, toType: UnitType, fromPlayer: *const PlayerInterface, toPlayer: *const PlayerInterface) -> i32;
        // unsafe fn getDamageTo(game: *mut Game, toType: UnitType, fromType: UnitType, toPlayer: *const PlayerInterface, fromPlayer: *const PlayerInterface) -> i32;
        // unsafe fn getEvents(game: *mut Game) ->  std::list<Event> ;
        // unsafe fn getForce(game: *mut Game, forceID: i32) -> Force;
        // unsafe fn getForces(game: *mut Game) -> Forceset;
        // unsafe fn getFPS(game: *mut Game) -> i32;
        // // unsafe fn getFrameCount(game: *mut Game) -> i32;
        // unsafe fn getGameType(game: *mut Game) -> GameType;
        // unsafe fn getGeysers(game: *mut Game) -> Unitset;
        // unsafe fn getGroundHeight(game: *mut Game, position: TilePosition) -> i32;
        // unsafe fn getInstanceNumber(game: *mut Game) -> i32;
        // unsafe fn getKeyState(game: *mut Game, key: Key) -> bool;
        // unsafe fn getLastError(game: *mut Game) -> Error;
        // unsafe fn getLastEventTime(game: *mut Game) -> i32;
        // unsafe fn getLatency(game: *mut Game) -> i32;
        // unsafe fn getLatencyFrames(game: *mut Game) -> i32;
        // unsafe fn getLatencyTime(game: *mut Game) -> i32;
        // unsafe fn getMinerals(game: *mut Game) -> Unitset;
        // unsafe fn getMousePosition(game: *mut Game) -> Position;
        // unsafe fn getMouseState(game: *mut Game, button: MouseButton) -> bool;
        // unsafe fn getNeutralUnits(game: *mut Game) -> Unitset;
        // unsafe fn getNukeDots(game: *mut Game) -> Position::list;
        // unsafe fn getPlayer(game: *mut Game, playerID: i32) -> *const PlayerInterface;
        // unsafe fn getPlayers(game: *mut Game) -> Playerset;
        // unsafe fn getRandomSeed(game: *mut Game) -> unsigned;
        // unsafe fn getRegion(game: *mut Game, regionID: i32) -> Region;
        // unsafe fn getRegionAt(game: *mut Game, position: Position) -> BWAPI::Region;
        // unsafe fn getRemainingLatencyFrames(game: *mut Game) -> i32;
        // unsafe fn getRemainingLatencyTime(game: *mut Game) -> i32;
        // unsafe fn getReplayFrameCount(game: *mut Game) -> i32;
        // unsafe fn getRevision(game: *mut Game) -> i32;
        // unsafe fn getScreenPosition(game: *mut Game) -> BWAPI::Position;
        // unsafe fn getSelectedUnits(game: *mut Game) -> Unitset;
        // unsafe fn getStartLocations(game: *mut Game) -> TilePosition::list;
        // unsafe fn getStaticGeysers(game: *mut Game) -> Unitset;
        // unsafe fn getStaticMinerals(game: *mut Game) -> Unitset;
        // unsafe fn getStaticNeutralUnits(game: *mut Game) -> Unitset;
        // unsafe fn getUnit(game: *mut Game, unitID: i32) -> *const UnitInterface;
        // unsafe fn getUnitsInRadius(game: *mut Game, center: Position, radius: i32, pred: UnitFilter) -> Unitset;
        // unsafe fn getUnitsInRectangle(game: *mut Game, topLeft: Position, bottomRight: Position, pred: UnitFilter) -> Unitset;
        // unsafe fn getUnitsOnTile(game: *mut Game, tile: TilePosition, pred: UnitFilter) -> Unitset;
        // unsafe fn hasCreep(game: *mut Game, position: TilePosition) -> bool;
        // unsafe fn hasPath(game: *mut Game, source: Position, destination: Position) -> bool;
        // unsafe fn hasPower(game: *mut Game, position: TilePosition, unitType: UnitType) -> bool;
        // unsafe fn hasPowerPrecise(game: *mut Game, position: Position, unitType: UnitType) -> bool;
        // unsafe fn indexToUnit(game: *mut Game, unitIndex: i32) -> *const UnitInterface;
        // unsafe fn isBattleNet(game: *mut Game) -> bool;
        // unsafe fn isBuildable(game: *mut Game, position: TilePosition, includeBuildings: bool) -> bool;
        // unsafe fn isDebug(game: *mut Game) -> bool;
        // unsafe fn isExplored(game: *mut Game, position: TilePosition) -> bool;
        // unsafe fn isFlagEnabled(game: *mut Game, flag: i32) -> bool;
        // unsafe fn isGUIEnabled(game: *mut Game) -> bool;
        // unsafe fn isInGame(game: *mut Game) -> bool;
        // unsafe fn isLatComEnabled(game: *mut Game) -> bool;
        // unsafe fn isMultiplayer(game: *mut Game) -> bool;
        // unsafe fn isPaused(game: *mut Game) -> bool;
        // unsafe fn isReplay(game: *mut Game) -> bool;
        // unsafe fn issueCommand(game: *mut Game, units: Unitset, command: UnitCommand) -> bool;
        // unsafe fn isVisible(game: *mut Game, position: TilePosition) -> bool;
        // unsafe fn isWalkable(game: *mut Game, position: WalkPosition) -> bool;
        // unsafe fn leaveGame(game: *mut Game);
        // unsafe fn mapFileName(game: *mut Game) -> std::string;
        // unsafe fn mapHash(game: *mut Game) -> std::string;
        // unsafe fn mapHeight(game: *mut Game) -> i32;
        // unsafe fn mapName(game: *mut Game) -> std::string;
        // unsafe fn mapPathName(game: *mut Game) -> std::string;
        // unsafe fn mapWidth(game: *mut Game) -> i32;
        // unsafe fn neutral(game: *mut Game) -> *const PlayerInterface;
        // unsafe fn observers(game: *mut Game) -> Playerset;
        // unsafe fn pauseGame(game: *mut Game);
        // unsafe fn pingMinimap(game: *mut Game, p: Position);
        // unsafe fn printf(game: *mut Game, char: *const format);
        // unsafe fn restartGame(game: *mut Game);
        // unsafe fn resumeGame(game: *mut Game);
        // unsafe fn self_player(game: *mut Game) -> *const PlayerInterface;
        // // unsafe fn sendText(game: *mut Game, text: *const c_char);
        // unsafe fn sendTextEx(game: *mut Game, toAllies: bool, char: *const format);
        // unsafe fn setAlliance(game: *mut Game, player: *const PlayerInterface, allied: bool, alliedVictory: bool) -> bool;
        // unsafe fn setCommandOptimizationLevel(game: *mut Game, level: i32);
        // unsafe fn setFrameSkip(game: *mut Game, frameSkip: i32);
        // unsafe fn setGUI(game: *mut Game, enabled: bool);
        // unsafe fn setLatCom(game: *mut Game, isEnabled: bool);
        // unsafe fn setLocalSpeed(game: *mut Game, speed: i32);
        // unsafe fn setMap(game: *mut Game, char: *const mapFileName) -> bool;
        // unsafe fn setRevealAll(game: *mut Game, reveal: bool) -> bool;
        // unsafe fn setScreenPosition(game: *mut Game, p: Position);
        // unsafe fn setVision(game: *mut Game, player: *const PlayerInterface, enabled: bool) -> bool;
        // unsafe fn vPrintf(game: *mut Game, char: *const format, args: va_list);
        // unsafe fn vSendText(game: *mut Game, char: *const format, args: va_list);
        // unsafe fn vSendTextEx(game: *mut Game, toAllies: bool, char: *const format, args: va_list);
        // unsafe fn setTextSize(game: *mut Game, size: TextSizeEnum);
        // unsafe fn drawText(game: *mut Game, ctype: CoordinateTypeEnum, x: i32, y: i32, char: *const format);
        // unsafe fn drawBox(game: *mut Game, ctype: CoordinateTypeEnum, left: i32, top: i32, right: i32, bottom: i32, color: Color, isSolid: bool);
        // unsafe fn drawTriangle(game: *mut Game, ctype: CoordinateTypeEnum, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, isSolid: bool);
        // unsafe fn drawCircle(game: *mut Game, ctype: CoordinateTypeEnum, x: i32, y: i32, radius: i32, color: Color, isSolid: bool);
        // unsafe fn drawEllipse(game: *mut Game, ctype: CoordinateTypeEnum, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, isSolid: bool);
        // unsafe fn drawDot(game: *mut Game, ctype: CoordinateTypeEnum, x: i32, y: i32, color: Color);
        // unsafe fn drawLine(game: *mut Game, ctype: CoordinateTypeEnum, x1: i32, y1: i32, x2: i32, y2: i32, color: Color);
    }

    // BWAPI::PlayerInterface
    // extern "C++" {
    //     fn allUnitCount(unit: UnitType) -> i32;
    //     fn armor(unit: UnitType) -> i32;
    //     fn completedUnitCount(unit: UnitType) -> i32;
    //     fn damage(wpn: WeaponType) -> i32;
    //     fn deadUnitCount(unit: UnitType) -> i32;
    //     fn gas() -> i32;
    //     fn gatheredGas() -> i32;
    //     fn gatheredMinerals() -> i32;
    //     fn getBuildingScore() -> i32;
    //     fn getColor() -> Color;
    //     fn getCustomScore() -> i32;
    //     fn getForce() -> Force;
    //     fn getID() -> i32;
    //     fn getKillScore() -> i32;
    //     fn getMaxUpgradeLevel(upgrade: UpgradeType) -> i32;
    //     fn getName() -> string;
    //     fn getRace() -> Race;
    //     fn getRazingScore() -> i32;
    //     fn getStartLocation() -> TilePosition;
    //     fn getTextColor() -> char;
    //     fn getType() -> PlayerType;
    //     fn getUnits() -> Unitset;
    //     fn getUnitScore() -> i32;
    //     fn getUpgradeLevel(upgrade: UpgradeType) -> i32;
    //     fn hasResearched(tech: TechType) -> bool;
    //     fn hasUnitTypeRequirement (unit: UnitType, amount: i32) -> bool;
    //     fn incompleteUnitCount(unit: UnitType) -> i32;
    //     fn isAlly(player: Player) -> bool;
    //     fn isDefeated() -> bool;
    //     fn isEnemy(player: Player) -> bool;
    //     fn isNeutral() -> bool;
    //     fn isObserver() -> bool;
    //     fn isResearchAvailable(tech: TechType) -> bool;
    //     fn isResearching(tech: TechType) -> bool;
    //     fn isUnitAvailable(unit: UnitType) -> bool;
    //     fn isUpgrading(upgrade: UpgradeType) -> bool;
    //     fn isVictorious() -> bool;
    //     fn killedUnitCount(unit: UnitType) -> i32;
    //     fn leftGame() -> bool;
    //     fn maxEnergy(unit: UnitType) -> i32;
    //     fn minerals() -> i32;
    //     fn refundedGas() -> i32;
    //     fn refundedMinerals() -> i32;
    //     fn repairedGas() -> i32;
    //     fn repairedMinerals() -> i32;
    //     fn sightRange(unit: UnitType) -> i32;
    //     fn spentGas() -> i32;
    //     fn spentMinerals() -> i32;
    //     fn supplyTotal(race: Race) -> i32;
    //     fn supplyUsed(race: Race) -> i32;
    //     fn topSpeed(unit: UnitType) -> f32;
    //     fn visibleUnitCount(unit: UnitType) -> i32;
    //     fn weaponDamageCooldown(unit: UnitType) -> i32;
    //     fn weaponMaxRange(weapon: WeaponType) -> i32;
    // }

    // BWAPI::RegionInterface
    // extern "C++" {
    //     fn getBoundsBottom() -> int;
    //     fn getBoundsLeft() -> int;
    //     fn getBoundsRight() -> int;
    //     fn getBoundsTop() -> int;
    //     fn getCenter() -> Position;
    //     fn getClosestAccessibleRegion() -> Region;
    //     fn getClosestInaccessibleRegion() -> Region;
    //     fn getDefensePriority() -> int;
    //     fn getDistance(other: Region) -> int;
    //     fn getID() -> int;
    //     fn getNeighbors() -> Regionset;
    //     fn getRegionGroupID() -> int;
    //     fn getUnits(pred: UnitFilter) -> Unitset;
    //     fn isAccessible() -> bool;
    //     fn isHigherGround() -> bool;
    // }

    // BWAPI::UnitInterface
    // extern "C++" {
    //     fn exists() -> bool;
    //     fn getAcidSporeCount() -> int;
    //     fn getAddon() -> Unit;
    //     fn getAirWeaponCooldown() -> int;
    //     fn getAngle() -> double;
    //     fn getBottom() -> int;
    //     fn getBuildType() -> UnitType;
    //     fn getBuildUnit() -> Unit;
    //     fn getCarrier() -> Unit;
    //     fn getClosestUnit(pred: UnitFilter, radius: int) -> Unit;
    //     fn getDefenseMatrixPoints() -> int;
    //     fn getDefenseMatrixTimer() -> int;
    //     fn getDistanceP(target: Position) -> int;
    //     fn getDistanceU(target: Unit) -> int;
    //     fn getEnergy() -> int;
    //     fn getEnsnareTimer() -> int;
    //     fn getGroundWeaponCooldown() -> int;
    //     fn getHatchery() -> Unit;
    //     fn getHitPoints() -> int;
    //     fn getID() -> int;
    //     fn getInitialHitPoints() -> int;
    //     fn getInitialPosition() -> Position;
    //     fn getInitialResources() -> int;
    //     fn getInitialTilePosition() -> TilePosition;
    //     fn getInitialType() -> UnitType;
    //     fn getInterceptorCount() -> int;
    //     fn getInterceptors() -> Unitset;
    //     fn getIrradiateTimer() -> int;
    //     fn getKillCount() -> int;
    //     fn getLarva() -> Unitset;
    //     fn getLastAttackingPlayer() -> Player;
    //     fn getLastCommand() -> UnitCommand;
    //     fn getLastCommandFrame() -> int;
    //     fn getLeft() -> int;
    //     fn getLoadedUnits() -> Unitset;
    //     fn getLockdownTimer() -> int;
    //     fn getMaelstromTimer() -> int;
    //     fn getNydusExit() -> Unit;
    //     fn getOrder() -> Order;
    //     fn getOrderTarget() -> Unit;
    //     fn getOrderTargetPosition() -> Position;
    //     fn getOrderTimer() -> int;
    //     fn getPlagueTimer() -> int;
    //     fn getPlayer() -> Player;
    //     fn getPosition() -> Position;
    //     fn getPowerUp() -> Unit;
    //     fn getRallyPosition() -> Position;
    //     fn getRallyUnit() -> Unit;
    //     fn getRegion() -> Region;
    //     fn getRemainingBuildTime() -> int;
    //     fn getRemainingResearchTime() -> int;
    //     fn getRemainingTrainTime() -> int;
    //     fn getRemainingUpgradeTime() -> int;
    //     fn getRemoveTimer() -> int;
    //     fn getReplayID() -> int;
    //     fn getResourceGroup() -> int;
    //     fn getResources() -> int;
    //     fn getRight() -> int;
    //     fn getScarabCount() -> int;
    //     fn getSecondaryOrder() -> Order;
    //     fn getShields() -> int;
    //     fn getSpaceRemaining() -> int;
    //     fn getSpellCooldown() -> int;
    //     fn getSpiderMineCount() -> int;
    //     fn getStasisTimer() -> int;
    //     fn getStimTimer() -> int;
    //     fn getTarget() -> Unit;
    //     fn getTargetPosition() -> Position;
    //     fn getTech() -> TechType;
    //     fn getTilePosition() -> TilePosition;
    //     fn getTop() -> int;
    //     fn getTrainingQueue() -> list<UnitType>;
    //     fn getTransport() -> Unit;
    //     fn getType() -> UnitType;
    //     fn getUnitsInRadius(radius: int, pred: UnitFilter) -> Unitset;
    //     fn getUnitsInWeaponRange(weapon: WeaponType, pred: UnitFilter) -> Unitset;
    //     fn getUpgrade() -> UpgradeType;
    //     fn getVelocityX() -> double;
    //     fn getVelocityY() -> double;
    //     fn hasNuke() -> bool;
    //     fn hasPathP(target: Position) -> bool;
    //     fn hasPathU(target: Unit) -> bool;
    //     fn isAccelerating() -> bool;
    //     fn isAttackFrame() -> bool;
    //     fn isAttacking() -> bool;
    //     fn isBeingConstructed() -> bool;
    //     fn isBeingGathered() -> bool;
    //     fn isBeingHealed() -> bool;
    //     fn isBlind() -> bool;
    //     fn isBraking() -> bool;
    //     fn isBurrowed() -> bool;
    //     fn isCarryingGas() -> bool;
    //     fn isCarryingMinerals() -> bool;
    //     fn isCloaked() -> bool;
    //     fn isCompleted() -> bool;
    //     fn isConstructing() -> bool;
    //     fn isDefenseMatrixed() -> bool;
    //     fn isDetected() -> bool;
    //     fn isEnsnared() -> bool;
    //     fn isFlying() -> bool;
    //     fn isFollowing() -> bool;
    //     fn isGatheringGas() -> bool;
    //     fn isGatheringMinerals() -> bool;
    //     fn isHallucination() -> bool;
    //     fn isHoldingPosition() -> bool;
    //     fn isIdle() -> bool;
    //     fn isInterruptible() -> bool;
    //     fn isInvincible() -> bool;
    //     fn isInWeaponRange(target: Unit) -> bool;
    //     fn isIrradiated() -> bool;
    //     fn isLifted() -> bool;
    //     fn isLoaded() -> bool;
    //     fn isLockedDown() -> bool;
    //     fn isMaelstrommed() -> bool;
    //     fn isMorphing() -> bool;
    //     fn isMoving() -> bool;
    //     fn isParasited() -> bool;
    //     fn isPatrolling() -> bool;
    //     fn isPlagued() -> bool;
    //     fn isPowered() -> bool;
    //     fn isRepairing() -> bool;
    //     fn isResearching() -> bool;
    //     fn isSelected() -> bool;
    //     fn isSieged() -> bool;
    //     fn isStartingAttack() -> bool;
    //     fn isStasised() -> bool;
    //     fn isStimmed() -> bool;
    //     fn isStuck() -> bool;
    //     fn isTargetable() -> bool;
    //     fn isTraining() -> bool;
    //     fn isUnderAttack() -> bool;
    //     fn isUnderDarkSwarm() -> bool;
    //     fn isUnderDisruptionWeb() -> bool;
    //     fn isUnderStorm() -> bool;
    //     fn isUpgrading() -> bool;
    //     fn isVisible(player: Player) -> bool;
    //     fn issueCommand(command: UnitCommand, shiftQueueCommand: bool) -> bool;
    //     fn attackP(target: Position, shiftQueueCommand: bool) -> bool;
    //     fn attackU(target: Unit, shiftQueueCommand: bool) -> bool;
    //     fn build(uType: UnitType, target: TilePosition) -> bool;
    //     fn buildAddon(uType: UnitType) -> bool;
    //     fn train(uType: UnitType) -> bool;
    //     fn morph(uType: UnitType) -> bool;
    //     fn research(tech: TechType) -> bool;
    //     fn upgrade(upgrade: UpgradeType) -> bool;
    //     fn setRallyPoint(target: Position) -> bool;
    //     fn move_(target: Position, shiftQueueCommand: bool) -> bool;
    //     fn patrol(target: Position, shiftQueueCommand: bool) -> bool;
    //     fn holdPosition(shiftQueueCommand: bool) -> bool;
    //     fn stop(shiftQueueCommand: bool) -> bool;
    //     fn follow(target: Unit, shiftQueueCommand: bool) -> bool;
    //     fn gather(target: Unit, shiftQueueCommand: bool) -> bool;
    //     fn returnCargo(shiftQueueCommand: bool) -> bool;
    //     fn repair(target: Unit, shiftQueueCommand: bool) -> bool;
    //     fn burrow() -> bool;
    //     fn unburrow() -> bool;
    //     fn cloak() -> bool;
    //     fn decloak() -> bool;
    //     fn siege() -> bool;
    //     fn unsiege() -> bool;
    //     fn lift() -> bool;
    //     fn land(target: TilePosition) -> bool;
    //     fn load(target: Unit, shiftQueueCommand: bool) -> bool;
    //     fn unload(target: Unit) -> bool;
    //     fn unloadAll(shiftQueueCommand: bool) -> bool;
    //     fn unloadAllP(target: Position, shiftQueueCommand: bool) -> bool;
    //     fn rightClickP(target: Position, shiftQueueCommand: bool) -> bool;
    //     fn rightClickU(target: Unit, shiftQueueCommand: bool) -> bool;
    //     fn haltConstruction() -> bool;
    //     fn cancelConstruction() -> bool;
    //     fn cancelAddon() -> bool;
    //     fn cancelTrain(slot: int) -> bool;
    //     fn cancelMorph() -> bool;
    //     fn cancelResearch() -> bool;
    //     fn cancelUpgrade() -> bool;
    //     fn useTechP(tech: TechType, target: Position) -> bool;
    //     fn useTechU(tech: TechType, target: Unit) -> bool;
    //     fn placeCOP(target: TilePosition) -> bool;
    //     fn canIssueCommand(command: UnitCommand, checkCanUseTechPositionOnPositions: bool, checkCanUseTechUnitOnUnits: bool, checkCanBuildUnitType: bool, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canIssueCommandGrouped(command: UnitCommand, checkCanUseTechPositionOnPositions: bool, checkCanUseTechUnitOnUnits: bool, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canCommand() -> bool;
    //     fn canCommandGrouped(checkCommandibility: bool) -> bool;
    //     fn canIssueCommandType(ct: UnitCommandType, checkCommandibility: bool) -> bool;
    //     fn canIssueCommandTypeGrouped(ct: UnitCommandType, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canTargetUnit(targetUnit: Unit, checkCommandibility: bool) -> bool;
    //     fn canAttack(checkCommandibility: bool) -> bool;
    //     fn canAttackP(target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackU(target: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackGroupedP(target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackGroupedU(target: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackMove(checkCommandibility: bool) -> bool;
    //     fn canAttackMoveGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackUnit(checkCommandibility: bool) -> bool;
    //     fn canAttackUnitU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackUnitGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canAttackUnitGroupedU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canBuild(checkCommandibility: bool) -> bool;
    //     fn canBuildT(uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canBuildU(uType: UnitType, tilePos: TilePosition, checkTargetUnitType: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canBuildAddon(checkCommandibility: bool) -> bool;
    //     fn canBuildAddonT(uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canTrain(checkCommandibility: bool) -> bool;
    //     fn canTrainT(uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canMorph(checkCommandibility: bool) -> bool;
    //     fn canMorphT(uType: UnitType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canResearch(checkCommandibility: bool) -> bool;
    //     fn canResearchT(tType: TechType, checkCanIssueCommandType: bool) -> bool;
    //     fn canUpgrade(checkCommandibility: bool) -> bool;
    //     fn canUpgradeT(tType: UpgradeType, checkCanIssueCommandType: bool) -> bool;
    //     fn canSetRallyPoint(checkCommandibility: bool) -> bool;
    //     fn canSetRallyPointP(target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canSetRallyPointU(target: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canSetRallyPosition(checkCommandibility: bool) -> bool;
    //     fn canSetRallyUnit(checkCommandibility: bool) -> bool;
    //     fn canSetRallyUnitU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canMove(checkCommandibility: bool) -> bool;
    //     fn canMoveGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canPatrol(checkCommandibility: bool) -> bool;
    //     fn canPatrolGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canFollow(checkCommandibility: bool) -> bool;
    //     fn canFollowU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canGather(checkCommandibility: bool) -> bool;
    //     fn canGatherU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canReturnCargo(checkCommandibility: bool) -> bool;
    //     fn canHoldPosition(checkCommandibility: bool) -> bool;
    //     fn canStop(checkCommandibility: bool) -> bool;
    //     fn canRepair(checkCommandibility: bool) -> bool;
    //     fn canRepairU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canBurrow(checkCommandibility: bool) -> bool;
    //     fn canUnburrow(checkCommandibility: bool) -> bool;
    //     fn canCloak(checkCommandibility: bool) -> bool;
    //     fn canDecloak(checkCommandibility: bool) -> bool;
    //     fn canSiege(checkCommandibility: bool) -> bool;
    //     fn canUnsiege(checkCommandibility: bool) -> bool;
    //     fn canLift(checkCommandibility: bool) -> bool;
    //     fn canLand(checkCommandibility: bool) -> bool;
    //     fn canLandP(target: TilePosition, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canLoad(checkCommandibility: bool) -> bool;
    //     fn canLoadU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUnloadWithOrWithoutTarget(checkCommandibility: bool) -> bool;
    //     fn canUnloadAtPosition(targDropPos: Position, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUnload(checkCommandibility: bool) -> bool;
    //     fn canUnloadU(targetUnit: Unit, checkCanTargetUnit: bool, checkPosition: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUnloadAll(checkCommandibility: bool) -> bool;
    //     fn canUnloadAllPosition(checkCommandibility: bool) -> bool;
    //     fn canUnloadAllPositionP(targDropPos: Position, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClick(checkCommandibility: bool) -> bool;
    //     fn canRightClickP(target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickU(target: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickGroupedP(target: Position, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickGroupedU(target: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickPosition(checkCommandibility: bool) -> bool;
    //     fn canRightClickPositionGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickUnit(checkCommandibility: bool) -> bool;
    //     fn canRightClickUnitU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickUnitGrouped(checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canRightClickUnitGroupedU(targetUnit: Unit, checkCanTargetUnit: bool, checkCanIssueCommandType: bool, checkCommandibilityGrouped: bool, checkCommandibility: bool) -> bool;
    //     fn canHaltConstruction(checkCommandibility: bool) -> bool;
    //     fn canCancelConstruction(checkCommandibility: bool) -> bool;
    //     fn canCancelAddon(checkCommandibility: bool) -> bool;
    //     fn canCancelTrain(checkCommandibility: bool) -> bool;
    //     fn canCancelTrainSlot(checkCommandibility: bool) -> bool;
    //     fn canCancelTrainSlotI(slot: int, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canCancelMorph(checkCommandibility: bool) -> bool;
    //     fn canCancelResearch(checkCommandibility: bool) -> bool;
    //     fn canCancelUpgrade(checkCommandibility: bool) -> bool;
    //     fn canUseTechWithOrWithoutTarget(checkCommandibility: bool) -> bool;
    //     fn canUseTechWithOrWithoutTargetT(tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechP(tech: TechType, target: Position, checkCanTargetUnit: bool, checkTargetsType: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechU(tech: TechType, target: Unit, checkCanTargetUnit: bool, checkTargetsType: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechWithoutTarget(tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechUnit(tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechUnitT(tech: TechType, targetUnit: Unit, checkCanTargetUnit: bool, checkTargetsUnits: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechPosition(tech: TechType, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canUseTechPositionP(tech: TechType, target: Position, checkTargetsPositions: bool, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    //     fn canPlaceCOP(checkCommandibility: bool) -> bool;
    //     fn canPlaceCOPP(target: TilePosition, checkCanIssueCommandType: bool, checkCommandibility: bool) -> bool;
    // }

    extern "Rust" {
        type BoxedAIModule<'a>;
    }
    unsafe extern "C++" {
        pub type AIModuleWrapper;
        #[rust_name = "create_ai_module_wrapper"]
        fn createAIModuleWrapper(user_ai: Box<BoxedAIModule>) -> UniquePtr<AIModuleWrapper>;
        #[rust_name = "get_box"]
        fn getBox(self: Pin<&mut AIModuleWrapper>) -> &mut BoxedAIModule;
    }

    extern "Rust" {
        include!("library/src/aim.h");
        // the hack is to create AimBox to create AIModuleWrapper on the C++ side
        unsafe fn hack<'a>() -> &'static BoxedAIModule<'a>;

        unsafe fn on_start(wrapper: Pin<&mut AIModuleWrapper>);
        unsafe fn on_end(wrapper: Pin<&mut AIModuleWrapper>, is_winner: bool);
        unsafe fn on_frame(wrapper: Pin<&mut AIModuleWrapper>);
        unsafe fn on_send_text(wrapper: Pin<&mut AIModuleWrapper>, text: &CxxString);
        unsafe fn on_receive_text(wrapper: Pin<&mut AIModuleWrapper>, player: *const PlayerInterface, text: &CxxString);
        unsafe fn on_player_left(wrapper: Pin<&mut AIModuleWrapper>, player: *const PlayerInterface);
        unsafe fn on_nuke_detect(wrapper: Pin<&mut AIModuleWrapper>, target: Position);
        unsafe fn on_unit_discover(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_evade(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_show(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_hide(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_create(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_destroy(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_morph(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_unit_renegade(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
        unsafe fn on_save_game(wrapper: Pin<&mut AIModuleWrapper>, game_name: &CxxString);
        unsafe fn on_unit_complete(wrapper: Pin<&mut AIModuleWrapper>, unit: *const UnitInterface);
    }
}

//region ----------- Shims to the bw::ai_module::AIModule trait ------------
fn on_start(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    wrapper.get_box().on_event(Event::OnStart());
}
fn on_end(wrapper: Pin<&mut ffi::AIModuleWrapper>, is_winner: bool) {
    wrapper.get_box().on_event(Event::OnEnd(is_winner));
}
fn on_frame(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    wrapper.get_box().on_event(Event::OnFrame());
}
fn on_send_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, text: &CxxString) {
    wrapper.get_box().on_event(Event::OnSendText(text.to_string()));
}
fn on_receive_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface, text: &CxxString) {
    let player = crate::bw::player::Player { raw: player };
    wrapper
        .get_box()
        .on_event(Event::OnReceiveText(player, text.to_string()));
}
fn on_player_left(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface) {
    let player = crate::bw::player::Player { raw: player };
    wrapper.get_box().on_event(Event::OnPlayerLeft(player));
}
fn on_nuke_detect(wrapper: Pin<&mut ffi::AIModuleWrapper>, target: ffi::Position) {
    let target = crate::bw::position::Position {
        x: target.x,
        y: target.y,
    };
    wrapper.get_box().on_event(Event::OnNukeDetect(target));
}
fn on_unit_discover(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitDiscover(unit));
}
fn on_unit_evade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitEvade(unit));
}
fn on_unit_show(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitShow(unit));
}
fn on_unit_hide(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitHide(unit));
}
fn on_unit_create(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitCreate(unit));
}
fn on_unit_destroy(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitDestroy(unit));
}
fn on_unit_morph(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitMorph(unit));
}
fn on_unit_renegade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitRenegade(unit));
}
fn on_save_game(wrapper: Pin<&mut ffi::AIModuleWrapper>, game_name: &CxxString) {
    wrapper.get_box().on_event(Event::OnSaveGame(game_name.to_string()));
}
fn on_unit_complete(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitComplete(unit));
}
//------------------- endregion -------------------

impl Debug for ffi::AIModuleWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ffi::AIModuleWrapper({:p})", self)
    }
}

// used for testing
pub static HACK_BOX: OnceCell<BoxedAIModule> = OnceCell::new();
fn hack() -> &'static BoxedAIModule<'static> {
    &HACK_BOX.get().unwrap()
}

#[cxx::bridge]
pub mod ffi_test {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        fn cpp_test() -> i32;
    }
}

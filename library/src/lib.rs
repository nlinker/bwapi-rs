#![feature(never_type)]
#[allow(non_snake_case)]
pub mod bw;
pub mod prelude;
mod sys;

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
    //use cxx::UniquePtr;

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
        pub type Order;
        pub type PlayerType;
        pub type Race;
        pub type UnitCommandType;
        pub type UnitSizeType;
        pub type WeaponType;

        type Event;
        type UnitFilter;

        pub type Bulletset;
        pub type Forceset;
        pub type Playerset;
        pub type Regionset;
        pub type Unitset;

        type UpgradeType = crate::bw::upgrade_type::UpgradeType;
        type GameType = crate::bw::game_type::GameType;
        type CoordinateType = crate::bw::coordinate_type::CoordinateType;
        type Position = crate::bw::position::Position;
        type TilePosition = crate::bw::position::TilePosition;
        type WalkPosition = crate::bw::position::WalkPosition;
        type UnitType = crate::bw::unit_type::UnitType;
        type UnitCommand = crate::bw::unit_command::UnitCommand;
        type TechType = crate::bw::tech_type::TechType;
        type TextSize = crate::bw::color::TextSize;
        type MouseButton = crate::bw::input::MouseButton;
        type Key = crate::bw::input::KeyButton;

        // type UnitCommandType = BWAPI_UnitCommandTypes_Enum_Enum;
        // type Error = BWAPI_Errors_Enum_Enum;
        // type Flag = BWAPI_Flag_Enum;
        // type Order = BWAPI_Orders_Enum_Enum;
        // type DamageType = BWAPI_DamageTypes_Enum_Enum;
        // type Race = crate::BWAPI_Races_Enum_Enum;
        // type UnitSizeType = BWAPI_UnitSizeTypes_Enum_Enum;
        // type UpgradeType = crate::BWAPI_UpgradeTypes_Enum_Enum;
        // type WeaponType = crate::BWAPI_WeaponTypes_Enum_Enum;
        // type ExplosionType = BWAPI_ExplosionTypes_Enum_Enum;

    }

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
        // unfortunately we have to create our type: https://github.com/dtolnay/cxx/issues/796
        pub type c_void;

        pub type BulletsetIterator;
        pub fn next(self: Pin<&mut BulletsetIterator>) -> *const BulletInterface;
        pub fn sizeHint(self: &BulletsetIterator) -> usize;
        pub fn underlying(self: &BulletsetIterator) -> &Bulletset;
        pub fn createBulletsetIteratorRef(set: &Bulletset) -> UniquePtr<BulletsetIterator>;

        // pub type EventsetIterator;
        // pub fn next(self: Pin<&mut EventsetIterator>) -> *const EventInterface;
        // pub fn sizeHint(self: &EventsetIterator) -> usize;
        // pub fn underlying(self: &EventsetIterator) -> &Eventset;

        pub type ForcesetIterator;
        pub fn next(self: Pin<&mut ForcesetIterator>) -> *const ForceInterface;
        pub fn sizeHint(self: &ForcesetIterator) -> usize;
        pub fn underlying(self: &ForcesetIterator) -> &Forceset;
        pub fn createForcesetIteratorRef(set: &Forceset) -> UniquePtr<ForcesetIterator>;

        pub type PlayersetIterator;
        pub fn next(self: Pin<&mut PlayersetIterator>) -> *const PlayerInterface;
        pub fn sizeHint(self: &PlayersetIterator) -> usize;
        pub fn underlying(self: &PlayersetIterator) -> &Playerset;
        pub fn createPlayersetIteratorRef(set: &Playerset) -> UniquePtr<PlayersetIterator>;

        pub type UnitsetIterator;
        pub fn next(self: Pin<&mut UnitsetIterator>) -> *const UnitInterface;
        pub fn sizeHint(self: &UnitsetIterator) -> usize;
        pub fn underlying(self: &UnitsetIterator) -> &Unitset;
        pub fn createRegionsetIteratorRef(set: &Regionset) -> UniquePtr<RegionsetIterator>;

        pub type RegionsetIterator;
        pub fn next(self: Pin<&mut RegionsetIterator>) -> *const RegionInterface;
        pub fn sizeHint(self: &RegionsetIterator) -> usize;
        pub fn underlying(self: &RegionsetIterator) -> &Regionset;
        pub fn createUnitsetIteratorRef(set: &Unitset) -> UniquePtr<UnitsetIterator>;

        // helpers so far, unit api is coming
        pub unsafe fn Unit_getId(unit: *const UnitInterface) -> i32;
        pub unsafe fn Unit_getType(unit: *const UnitInterface) -> UnitType;
        pub unsafe fn Unit_getPosition(unit: *const UnitInterface) -> Position;
    }

    // region BWAPI::Unitset
    unsafe extern "C++" {
        fn _unitset_getClosestUnit(set: &Unitset, pred: fn(Unit) -> bool, radius: i32) -> *const UnitInterface;
        fn _unitset_getInterceptors(set: &Unitset) -> UniquePtr<UnitsetIterator>;
        fn _unitset_getLarva(set: &Unitset) -> UniquePtr<UnitsetIterator>;
        fn _unitset_getLoadedUnits(set: &Unitset) -> UniquePtr<UnitsetIterator>;
        fn getPosition(self: &Unitset) -> Position;
        fn _unitset_getUnitsInRadius(set: &Unitset, radius: i32, pred: fn(Unit) -> bool) -> UniquePtr<UnitsetIterator>;
        unsafe fn setClientInfo(self: &Unitset, clientInfo: *mut c_void, index: i32);
        #[cxx_name = "setClientInfo"]
        fn setClientInfo1(self: &Unitset, client_info: i32, index: i32);
        fn issueCommand(self: &Unitset, command: UnitCommand) -> bool;
        unsafe fn attack(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        #[cxx_name = "attack"]
        fn attack1(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn build(self: &Unitset, utype: UnitType, target: TilePosition) -> bool;
        fn buildAddon(self: &Unitset, utype: UnitType) -> bool;
        fn train(self: &Unitset, utype: UnitType) -> bool;
        fn morph(self: &Unitset, utype: UnitType) -> bool;
        unsafe fn setRallyPoint(self: &Unitset, target: *mut UnitInterface) -> bool;
        #[cxx_name = "setRallyPoint"]
        fn setRallyPoint1(self: &Unitset, target: Position) -> bool;
        fn _unitset_move(set: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn patrol(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn holdPosition(self: &Unitset, shiftQueueCommand: bool) -> bool;
        fn stop(self: &Unitset, shiftQueueCommand: bool) -> bool;
        unsafe fn follow(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        unsafe fn gather(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn returnCargo(self: &Unitset, shiftQueueCommand: bool) -> bool;
        unsafe fn repair(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn burrow(self: &Unitset) -> bool;
        fn unburrow(self: &Unitset) -> bool;
        fn cloak(self: &Unitset) -> bool;
        fn decloak(self: &Unitset) -> bool;
        fn siege(self: &Unitset) -> bool;
        fn unsiege(self: &Unitset) -> bool;
        fn lift(self: &Unitset) -> bool;
        unsafe fn load(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        fn unloadAll(self: &Unitset, shiftQueueCommand: bool) -> bool;
        #[cxx_name = "unloadAll"]
        fn unloadAll1(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        unsafe fn rightClick(self: &Unitset, target: *mut UnitInterface, shiftQueueCommand: bool) -> bool;
        #[cxx_name = "rightClick"]
        fn rightClick1(self: &Unitset, target: Position, shiftQueueCommand: bool) -> bool;
        fn haltConstruction(self: &Unitset) -> bool;
        fn cancelConstruction(self: &Unitset) -> bool;
        fn cancelAddon(self: &Unitset) -> bool;
        fn cancelTrain(self: &Unitset, slot: i32) -> bool;
        fn cancelMorph(self: &Unitset) -> bool;
        fn cancelResearch(self: &Unitset) -> bool;
        fn cancelUpgrade(self: &Unitset) -> bool;
        unsafe fn useTech(self: &Unitset, tech: TechType, target: *mut UnitInterface) -> bool;
        #[cxx_name = "useTech"]
        fn useTech1(self: &Unitset, tech: TechType, target: Position) -> bool;
    }
    // endregion

    // region BWAPI::Game
    unsafe extern "C++" {
        pub fn _game_debug(game: &Game);

        pub fn allies(self: Pin<&mut Game>) -> Pin<&mut Playerset>;
        pub unsafe fn canBuildHere(self: Pin<&mut Game>, position: TilePosition, uType: UnitType, builder: *mut UnitInterface, checkExplored: bool) -> bool;
        pub unsafe fn canMake(self: &Game, utype: UnitType, builder: *mut UnitInterface) -> bool;
        pub unsafe fn canResearch(self: Pin<&mut Game>, ttype: TechType, unit: *mut UnitInterface, checkCanIssueCommandType: bool) -> bool;
        pub unsafe fn canUpgrade(self: Pin<&mut Game>, utype: UpgradeType, unit: *mut UnitInterface, checkCanIssueCommandType: bool) -> bool;
        pub fn countdownTimer(self: &Game) -> i32;
        pub fn elapsedTime(self: &Game) -> i32;
        pub fn enableFlag(self: Pin<&mut Game>, flag: i32);
        pub fn _game_enemies(game: Pin<&mut Game>) -> UniquePtr<PlayersetIterator>;
        pub unsafe fn enemy(self: &Game) -> *mut PlayerInterface;
        pub fn _game_getAllRegions(game: &Game) -> UniquePtr<RegionsetIterator>;
        pub fn _game_getAllUnits(game: &Game) -> UniquePtr<UnitsetIterator>;
        pub fn getAPM(self: &Game, includeSelects: bool) -> i32;
        pub fn getAverageFPS(self: &Game) -> f64;
        pub fn _game_getBestUnit(game: &Game, best: fn(Unit, Unit) -> Unit, pred: fn(Unit) -> bool, center: Position, radius: i32) -> *mut UnitInterface;
        pub fn getBuildLocation(self: &Game, unitType: UnitType, desiredPosition: TilePosition, maxRange: i32, creep: bool) -> TilePosition;
        pub fn _game_getBullets(game: &Game) -> UniquePtr<BulletsetIterator>;
        pub fn getClientVersion(self: &Game) -> i32;
        pub fn _game_getClosestUnit(game: &Game, center: Position, pred: fn(Unit) -> bool, radius: i32) -> *mut UnitInterface;
        pub fn _game_getClosestUnitInRectangle(game: &Game, center: Position, pred: fn(Unit) -> bool, left: i32, top: i32, right: i32, bottom: i32) -> *mut UnitInterface;
        pub unsafe fn getDamageFrom(self: &Game, fromType: UnitType, toType: UnitType, fromPlayer: *mut PlayerInterface, toPlayer: *mut PlayerInterface) -> i32;
        pub unsafe fn getDamageTo(self: &Game, toType: UnitType, fromType: UnitType, toPlayer: *mut PlayerInterface, fromPlayer: *mut PlayerInterface) -> i32;
        // pub fn getEvents(game: &Game) -> UniquePtr<EventIterator>;
        pub fn getForce(self: &Game, forceId: i32) -> *mut ForceInterface;
        pub fn _game_getForces(game: &Game) -> UniquePtr<ForcesetIterator>;
        pub fn getFPS(self: &Game) -> i32;
        pub fn getFrameCount(self: &Game) -> i32;
        pub fn getGameType(self: &Game) -> GameType;
        pub fn _game_getGeysers(game: &Game) -> UniquePtr<UnitsetIterator>;
        pub fn getGroundHeight(self: &Game, position: TilePosition) -> i32;
        pub fn getInstanceNumber(self: &Game) -> i32;
        pub fn getKeyState(self: &Game, key: Key) -> bool;
        // pub fn getLastError(self: &Game) -> Error;
        pub fn getLastEventTime(self: &Game) -> i32;
        pub fn getLatency(self: &Game) -> i32;
        pub fn getLatencyFrames(self: &Game) -> i32;
        pub fn getLatencyTime(self: &Game) -> i32;
        pub fn _game_getMinerals(game: &Game) -> UniquePtr<UnitsetIterator>;
        pub fn getMousePosition(self: &Game) -> Position;
        pub fn getMouseState(self: &Game, button: MouseButton) -> bool;
        pub fn _game_getNeutralUnits(game: &Game) -> UniquePtr<UnitsetIterator>;
        // pub fn getNukeDots(self: &Game) -> Position::list;
        pub fn getPlayer(self: &Game, playerId: i32) -> *mut PlayerInterface;
        pub fn _game_getPlayers(game: &Game) -> UniquePtr<PlayersetIterator>;
        pub fn getRandomSeed(self: &Game) -> u32;
        pub fn getRegion(self: &Game, regionID: i32) -> *mut RegionInterface;
        pub fn getRegionAt(self: &Game, position: Position) -> *mut RegionInterface;
        pub fn getRemainingLatencyFrames(self: &Game) -> i32;
        pub fn getRemainingLatencyTime(self: &Game) -> i32;
        pub fn getReplayFrameCount(self: &Game) -> i32;
        pub fn getRevision(self: &Game) -> i32;
        pub fn getScreenPosition(self: &Game) -> Position;
        pub fn _game_getSelectedUnits(game: &Game) -> UniquePtr<UnitsetIterator>;
        // pub fn getStartLocations(self: &Game) -> TilePosition::list;
        pub fn _game_getStaticGeysers(game: &Game) -> UniquePtr<UnitsetIterator>;
        pub fn _game_getStaticMinerals(game: &Game) -> UniquePtr<UnitsetIterator>;
        pub fn _game_getStaticNeutralUnits(game: &Game) -> UniquePtr<UnitsetIterator>;
        pub fn getUnit(self: &Game, unitID: i32) -> *mut UnitInterface;
        pub fn _game_getUnitsInRadius(game: &Game, position: Position, radius: i32, pred: fn(Unit) -> bool) -> UniquePtr<UnitsetIterator>;
        pub fn _game_getUnitsInRectangle(game: &Game, topLeft: Position, bottomRight: Position, pred: fn(Unit) -> bool) -> UniquePtr<UnitsetIterator>;
        pub fn _game_getUnitsOnTile(game: &Game, tile: TilePosition, pred: fn(Unit) -> bool) -> UniquePtr<UnitsetIterator>;
        pub fn hasCreep(self: &Game, position: TilePosition) -> bool;
        pub fn hasPath(self: &Game, source: Position, destination: Position) -> bool;
        pub fn hasPower(self: &Game, position: TilePosition, unitType: UnitType) -> bool;
        pub fn hasPowerPrecise(self: &Game, position: Position, unitType: UnitType) -> bool;
        pub fn indexToUnit(self: &Game, unitIndex: i32) -> *mut UnitInterface;
        pub fn isBattleNet(self: &Game) -> bool;
        pub fn isBuildable(self: &Game, position: TilePosition, includeBuildings: bool) -> bool;
        pub fn isDebug(self: &Game) -> bool;
        pub fn isExplored(self: &Game, position: TilePosition) -> bool;
        pub fn isFlagEnabled(self: &Game, flag: i32) -> bool;
        pub fn isGUIEnabled(self: &Game) -> bool;
        pub fn isInGame(self: &Game) -> bool;
        pub fn isLatComEnabled(self: &Game) -> bool;
        pub fn isMultiplayer(self: &Game) -> bool;
        pub fn isPaused(self: &Game) -> bool;
        pub fn isReplay(self: &Game) -> bool;
        pub fn issueCommand(self: Pin<&mut Game>, units: &Unitset, command: UnitCommand) -> bool;
        pub fn isVisible(self: &Game, position: TilePosition) -> bool;
        pub fn isWalkable(self: &Game, position: WalkPosition) -> bool;
        pub fn leaveGame(self: Pin<&mut Game>);
        pub fn _game_mapFileName(game: &Game) -> UniquePtr<CxxString>;
        pub fn _game_mapHash(game: &Game) -> UniquePtr<CxxString>;
        pub fn mapHeight(self: &Game) -> i32;
        pub fn _game_mapName(game: &Game) -> UniquePtr<CxxString>;
        pub fn _game_mapPathName(game: &Game) -> UniquePtr<CxxString>;
        pub fn mapWidth(self: &Game) -> i32;
        pub fn neutral(self: &Game) -> *mut PlayerInterface;
        pub fn _game_observers(game: Pin<&mut Game>) -> UniquePtr<PlayersetIterator>;
        pub fn pauseGame(self: Pin<&mut Game>);
        pub fn pingMinimap(self: Pin<&mut Game>, p: Position);
        pub fn _game_printf(game: Pin<&mut Game>, text: &str);
        pub fn restartGame(self: Pin<&mut Game>);
        pub fn resumeGame(self: Pin<&mut Game>);
        pub fn _game_self(game: &Game) -> *mut PlayerInterface;
        pub fn _game_sendText(game: Pin<&mut Game>, text: &str);
        pub fn _game_sendTextEx(game: Pin<&mut Game>, toAllies: bool, text: &str);
        pub unsafe fn setAlliance(self: Pin<&mut Game>, player: *mut PlayerInterface, allied: bool, alliedVictory: bool) -> bool;
        pub fn setCommandOptimizationLevel(self: Pin<&mut Game>, level: i32);
        pub fn setFrameSkip(self: Pin<&mut Game>, frameSkip: i32);
        pub fn setGUI(self: Pin<&mut Game>, enabled: bool);
        pub fn setLatCom(self: Pin<&mut Game>, isEnabled: bool);
        pub fn setLocalSpeed(self: Pin<&mut Game>, speed: i32);
        pub fn _game_setMap(game: Pin<&mut Game>, text: &str) -> bool;
        pub fn setRevealAll(self: Pin<&mut Game>, reveal: bool) -> bool;
        pub fn setScreenPosition(self: Pin<&mut Game>, p: Position);
        pub unsafe fn setVision(self: Pin<&mut Game>, player: *mut PlayerInterface, enabled: bool) -> bool;

        // pub fn vPrintf(self: &Game, text: &str);
        // pub fn vSendText(self: &Game, text: &str);
        // pub fn vSendTextEx(self: &Game, toAllies: bool, char: *const format, args: va_list);

        // pub fn setTextSize(self: &Game, size: TextSize);
        // pub fn drawText(self: &Game, ctype: CoordinateType, x: i32, y: i32, char: *const format);
        // pub fn drawBox(self: &Game, ctype: CoordinateType, left: i32, top: i32, right: i32, bottom: i32, color: Color, isSolid: bool);
        // pub fn drawTriangle(self: &Game, ctype: CoordinateType, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, isSolid: bool);
        // pub fn drawCircle(self: &Game, ctype: CoordinateType, x: i32, y: i32, radius: i32, color: Color, isSolid: bool);
        // pub fn drawEllipse(self: &Game, ctype: CoordinateType, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, isSolid: bool);
        // pub fn drawDot(self: &Game, ctype: CoordinateType, x: i32, y: i32, color: Color);
        // pub fn drawLine(self: &Game, ctype: CoordinateType, x1: i32, y1: i32, x2: i32, y2: i32, color: Color);
    }
    // endregion

    // region BWAPI::PlayerInterface
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
    // endregion

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
    //     fn getUnits(pred: fn(Unit) -> bool) -> Unitset;
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
    let player = unsafe { crate::bw::player::Player::from_raw(player) };
    wrapper
        .get_box()
        .on_event(Event::OnReceiveText(player, text.to_string()));
}
fn on_player_left(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface) {
    let player = unsafe { crate::bw::player::Player::from_raw(player) };
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
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitDiscover(unit));
}
fn on_unit_evade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitEvade(unit));
}
fn on_unit_show(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitShow(unit));
}
fn on_unit_hide(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitHide(unit));
}
fn on_unit_create(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitCreate(unit));
}
fn on_unit_destroy(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitDestroy(unit));
}
fn on_unit_morph(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitMorph(unit));
}
fn on_unit_renegade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
    wrapper.get_box().on_event(Event::OnUnitRenegade(unit));
}
fn on_save_game(wrapper: Pin<&mut ffi::AIModuleWrapper>, game_name: &CxxString) {
    wrapper.get_box().on_event(Event::OnSaveGame(game_name.to_string()));
}
fn on_unit_complete(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = unsafe { crate::bw::unit::Unit::from_raw(unit) };
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

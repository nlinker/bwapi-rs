use crate::bw::bulletset::Bulletset;
use crate::bw::color::{Color, TextSize};
use crate::bw::coordinate_type::CoordinateType;
use crate::bw::flag::Flag;
use crate::bw::force::Force;
use crate::bw::forceset::Forceset;
use crate::bw::game_type::GameType;
use crate::bw::input::{KeyButton, MouseButton};
use crate::bw::player::Player;
use crate::bw::playerset::Playerset;
use crate::bw::position::{Position, TilePosition};
use crate::bw::region::Region;
use crate::bw::regionset::Regionset;
use crate::bw::tech_type::TechType;
use crate::bw::unit::Unit;
use crate::bw::unit_type::UnitType;
use crate::bw::unitset::Unitset;
use crate::bw::upgrade_type::UpgradeType;
use crate::bw::{with_unit_and_best_filter, with_unit_filter, Handle};
use crate::{ffi, FromRaw};
use std::pin::Pin;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Game {
    pub(crate) raw: Option<NonNull<ffi::Game>>,
}

/// Game object doesn't contain any self-references
impl Unpin for Game {}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl FromRaw<ffi::Game> for Game {
    unsafe fn from_raw(raw: *mut ffi::Game) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: Some(NonNull::new_unchecked(raw)),
        }
    }
}

impl Game {
    pub fn debug(&self) {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        ffi::_game_debug(g);
    }

    pub fn allies(&self) -> Playerset {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        let set: Pin<&mut ffi::Playerset> = g.allies();
        Playerset {
            raw: Handle::BorrowedMut(set),
        }
    }
    pub fn can_build_here(&self, position: TilePosition, utype: UnitType, builder: Unit, check_explored: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        unsafe { g.canBuildHere(position, utype, builder.raw.as_ptr(), check_explored) }
    }
    pub fn can_make(&self, utype: UnitType, builder: Unit) -> bool {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        unsafe { g.canMake(utype, builder.raw.as_ptr()) }
    }
    pub fn can_research(&self, ttype: TechType, unit: Unit, check_can_issue_command_type: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        unsafe { g.canResearch(ttype, unit.raw.as_ptr(), check_can_issue_command_type) }
    }
    pub fn can_upgrade(&self, utype: UpgradeType, unit: Unit, check_can_issue_command_type: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        unsafe { g.canUpgrade(utype, unit.raw.as_ptr(), check_can_issue_command_type) }
    }
    pub fn countdown_timer(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.countdownTimer()
    }
    pub fn elapsed_time(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.elapsedTime()
    }
    pub fn enable_flag(&self, flag: Flag) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        ffi::_game_enableFlag(g, flag)
    }
    pub fn enemies(&self) -> Playerset {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        Playerset {
            raw: Handle::BorrowedMut(g.enemies()),
        }
    }
    pub fn enemy(&self) -> Player {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        unsafe { Player::from_raw(g.enemy()) }
    }
    pub fn get_all_regions(&self) -> Regionset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Regionset {
            raw: Handle::Borrowed(g.getAllRegions()),
        }
    }
    pub fn get_all_units(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getAllUnits()),
        }
    }
    pub fn get_apm(&self, include_selects: bool) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getAPM(include_selects)
    }
    pub fn get_average_fps(&self) -> f64 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getAverageFPS()
    }
    pub fn get_best_unit(
        &self,
        best_fn: impl Fn(Unit, Unit) -> Unit + 'static,
        unit_fn: impl Fn(Unit) -> bool + 'static,
        center: Position,
        radius: i32,
    ) -> Option<Unit> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        with_unit_and_best_filter(unit_fn, best_fn, |uf, bf| {
            Unit::option(ffi::_game_getBestUnit(g, bf, uf, center, radius))
        })
    }
    pub fn get_build_location(
        &self,
        utype: UnitType,
        desired_position: TilePosition,
        max_range: i32,
        creep: bool,
    ) -> TilePosition {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getBuildLocation(utype, desired_position, max_range, creep)
    }
    pub fn get_bullets(&self) -> Bulletset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Bulletset {
            raw: Handle::Borrowed(g.getBullets()),
        }
    }
    pub fn get_client_version(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getClientVersion()
    }
    pub fn get_closest_unit(
        &self,
        center: Position,
        unit_fn: impl Fn(Unit) -> bool + 'static,
        radius: i32,
    ) -> Option<Unit> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        with_unit_filter(unit_fn, |uf| {
            Unit::option(ffi::_game_getClosestUnit(g, center, uf, radius))
        })
    }
    pub fn get_closest_unit_in_rectangle(
        &self,
        center: Position,
        unit_fn: impl Fn(Unit) -> bool + 'static,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) -> Option<Unit> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        with_unit_filter(unit_fn, |uf| {
            Unit::option(ffi::_game_getClosestUnitInRectangle(
                g, center, uf, left, top, right, bottom,
            ))
        })
    }
    pub fn get_damage_from(
        &self,
        from_type: UnitType,
        to_type: UnitType,
        from_player: Player,
        to_player: Player,
    ) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        unsafe { g.getDamageFrom(from_type, to_type, from_player.raw.as_ptr(), to_player.raw.as_ptr()) }
    }
    pub fn get_damage_to(&self, to_type: UnitType, from_type: UnitType, to_player: Player, from_player: Player) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        unsafe { g.getDamageTo(from_type, to_type, from_player.raw.as_ptr(), to_player.raw.as_ptr()) }
    }
    // pub fn get_events(&self) -> &EventList { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getEvents(g) }
    pub fn get_force(&self, force_id: i32) -> Option<Force> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Force::option(g.getForce(force_id))
    }
    pub fn get_forces(&self) -> Forceset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Forceset {
            raw: Handle::Borrowed(g.getForces()),
        }
    }
    pub fn get_fps(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getFPS()
    }
    pub fn get_frame_count(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getFrameCount()
    }
    pub fn get_game_type(&self) -> GameType {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getGameType()
    }
    pub fn get_geysers(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getGeysers()),
        }
    }
    pub fn get_ground_height(&self, position: TilePosition) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getGroundHeight(position)
    }
    pub fn get_instance_number(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getInstanceNumber()
    }
    pub fn get_key_state(&self, key: KeyButton) -> bool {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getKeyState(key)
    }
    pub fn get_last_event_time(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getLastEventTime()
    }
    pub fn get_latency(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getLatency()
    }
    pub fn get_latency_frames(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getLatencyFrames()
    }
    pub fn get_latency_time(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getLatencyTime()
    }
    pub fn get_minerals(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getMinerals()),
        }
    }
    pub fn get_mouse_position(&self) -> Position {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getMousePosition()
    }
    pub fn get_mouse_state(&self, button: MouseButton) -> bool {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getMouseState(button)
    }
    pub fn get_neutral_units(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getNeutralUnits()),
        }
    }
    pub fn get_nuke_dots(&self) -> Vec<Position> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        ffi::_game_getNukeDots(g)
    }
    pub fn get_player(&self, player_id: i32) -> Option<Player> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Player::option(g.getPlayer(player_id))
    }
    pub fn get_players(&self) -> Playerset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Playerset {
            raw: Handle::Borrowed(g.getPlayers()),
        }
    }
    pub fn get_random_seed(&self) -> u32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getRandomSeed()
    }
    pub fn get_region(&self, region_id: i32) -> Option<Region> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Region::option(g.getRegion(region_id))
    }
    pub fn get_region_at(&self, position: Position) -> Option<Region> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Region::option(g.getRegionAt(position))
    }
    pub fn get_remaining_latency_frames(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getRemainingLatencyFrames()
    }
    pub fn get_remaining_latency_time(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getRemainingLatencyTime()
    }
    pub fn get_replay_frame_count(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getReplayFrameCount()
    }
    pub fn get_revision(&self) -> i32 {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getRevision()
    }
    pub fn get_screen_position(&self) -> Position {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        g.getScreenPosition()
    }
    pub fn get_selected_units(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getSelectedUnits()),
        }
    }
    pub fn get_start_locations(&self) -> Vec<TilePosition> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        ffi::_game_getStartLocations(g)
    }
    pub fn get_static_geysers(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getStaticGeysers()),
        }
    }
    pub fn get_static_minerals(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getStaticMinerals()),
        }
    }
    pub fn get_static_neutral_units(&self) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unitset {
            raw: Handle::Borrowed(g.getStaticNeutralUnits()),
        }
    }
    pub fn get_unit(&self, unit_id: i32) -> Option<Unit> {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        Unit::option(g.getUnit(unit_id))
    }
    pub fn get_units_in_radius(
        &self,
        position: Position,
        radius: i32,
        unit_fn: impl Fn(Unit) -> bool + 'static,
    ) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        let set = with_unit_filter(unit_fn, |uf| ffi::_game_getUnitsInRadius(g, position, radius, uf));
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_units_in_rectangle(
        &self,
        top_left: Position,
        bottom_right: Position,
        unit_fn: impl Fn(Unit) -> bool + 'static,
    ) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        let set = with_unit_filter(unit_fn, |uf| {
            ffi::_game_getUnitsInRectangle(g, top_left, bottom_right, uf)
        });
        Unitset {
            raw: Handle::Owned(set),
        }
    }
    pub fn get_units_on_tile(&self, tile: TilePosition, unit_fn: impl Fn(Unit) -> bool + 'static) -> Unitset {
        let g: &ffi::Game = unsafe { self.raw.unwrap().as_ref() };
        let set = with_unit_filter(unit_fn, |uf| ffi::_game_getUnitsOnTile(g, tile, uf));
        Unitset {
            raw: Handle::Owned(set),
        }
    }

    // pub fn has_creep(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.hasCreep()  }                                                                           //                      (self: &Game, position: TilePosition) -> bool
    // pub fn has_path(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.hasPath()  }                                                                             //                       (self: &Game, source: Position, destination: Position) -> bool
    // pub fn has_power(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.hasPower()  }                                                                           //                      (self: &Game, position: TilePosition, unitType: UnitType) -> bool
    // pub fn has_power_precise(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.hasPowerPrecise()  }                                                            //              (self: &Game, position: Position, unitType: UnitType) -> bool
    // pub fn index_to_unit(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.indexToUnit()  }                                                                    //                  (self: &Game, unitIndex: i32) -> *mut UnitInterface
    // pub fn is_battle_net(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isBattleNet()  }                                                                    //                  (self: &Game) -> bool
    // pub fn is_buildable(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isBuildable()  }                                                                     //                   (self: &Game, position: TilePosition, includeBuildings: bool) -> bool
    // pub fn is_debug(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isDebug()  }                                                                             //                       (self: &Game) -> bool
    // pub fn is_explored(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isExplored()  }                                                                       //                    (self: &Game, position: TilePosition) -> bool
    // pub fn is_flag_enabled(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_isFlagEnabled(g) }                                                       //                  (game: &Game, flag: Flag) -> bool; /
    // pub fn is_guienabled(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isGUIEnabled()  }                                                                   //                  (self: &Game) -> bool
    // pub fn is_in_game(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isInGame()  }                                                                          //                     (self: &Game) -> bool
    // pub fn is_lat_com_enabled(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.isLatComEnabled() }                                                           //            (self: &Game) -> bool
    // pub fn is_multiplayer(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isMultiplayer()  }                                                                 //                 (self: &Game) -> bool
    // pub fn is_paused(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isPaused()  }                                                                           //                      (self: &Game) -> bool
    // pub fn is_replay(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isReplay()  }                                                                           //                      (self: &Game) -> bool
    // pub fn issue_command(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.issueCommand()  }                                  //                        (self: Pin<&mut Game>, units: &Unitset, command: UnitCommand) -> bool
    // pub fn is_visible(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isVisible()  }                                                                         //                     (self: &Game, position: TilePosition) -> bool
    // pub fn is_walkable(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.isWalkable()  }                                                                       //                    (self: &Game, position: WalkPosition) -> bool
    // pub fn leave_game(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.leaveGame()  }                                        //                     (self: Pin<&mut Game>)
    // pub fn map_file_name(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_mapFileName(g) }                                                           //                    (game: &Game) -> UniquePtr<CxxString>; /
    // pub fn map_hash(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_mapHash(g) }                                                                    //                         (game: &Game) -> UniquePtr<CxxString>; /
    // pub fn map_height(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.mapHeight()  }                                                                         //                     (self: &Game) -> i32
    // pub fn map_name(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_mapName(g) }                                                                    //                         (game: &Game) -> UniquePtr<CxxString>; /
    // pub fn map_path_name(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_mapPathName(g) }                                                           //                    (game: &Game) -> UniquePtr<CxxString>; /
    // pub fn map_width(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.mapWidth()  }                                                                           //                      (self: &Game) -> i32
    // pub fn neutral(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.neutral()  }                                                                              //                        (self: &Game) -> *mut PlayerInterface
    // pub fn observers(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.observers()  }                                         //                     (self: Pin<&mut Game>) -> Pin<&mut Playerset>
    // pub fn pause_game(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.pauseGame()  }                                        //                     (self: Pin<&mut Game>)
    // pub fn ping_minimap(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.pingMinimap()  }                                    //                       (self: Pin<&mut Game>, p: Position)
    // pub fn printf(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g._game_printf()  }                                         //                        (game: Pin<&mut Game>, text: &str)
    // pub fn restart_game(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.restartGame()  }                                    //                       (self: Pin<&mut Game>)
    // pub fn resume_game(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.resumeGame()  }                                      //                      (self: Pin<&mut Game>)
    // pub fn self_(&self) -> Player { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_self    // pub fn send_text(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g._game_sendText()  }                                    //                          (game: Pin<&mut Game>, text: &str)
    // pub fn send_text_ex(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g._game_sendTextEx()  }                               //                            (game: Pin<&mut Game>, toAllies: bool, text: &str)
    pub fn set_alliance(&self, player: Player, allied: bool, allied_victory: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        unsafe { g.setAlliance(player.raw.as_ptr(), allied, allied_victory) }
    }
    // pub fn set_command_optimization_level(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setCommandOptimizationLevel()  }  //                     (self: Pin<&mut                                       Game>, level: i32)
    // pub fn set_frame_skip(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setFrameSkip()  }                                 //                        (self: Pin<&mut Game>, frameSkip: i32)
    // pub fn set_gui(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setGUI()  }                                              //                  (self: Pin<&mut Game>, enabled: bool)
    // pub fn set_lat_com(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setLatCom()  }                                       //                     (self: Pin<&mut Game>, isEnabled: bool)
    // pub fn set_local_speed(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setLocalSpeed()  }                               //                         (self: Pin<&mut Game>, speed: i32)
    // pub fn set_map(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g._game_setMap()  }                                        //                        (game: Pin<&mut Game>, text: &str) -> bool
    // pub fn set_reveal_all(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setRevealAll()  }                                 //                        (self: Pin<&mut Game>, reveal: bool) -> bool
    // pub fn set_screen_position(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setScreenPosition()  }                       //                         (self: Pin<&mut Game>, p: Position)
    // pub fn set_vision(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setVision()  }                                        //                     (self: Pin<&mut Game>, player: *mut PlayerInterface, enabled: bool) -> bool

    pub fn send_text(&self, text: &str) {
        ffi::_game_sendText(unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) }, text)
    }

    // let ctype = ctype.unwrap_or(CoordinateType::Map);
    pub fn set_text_size(&self, size: TextSize) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.setTextSize(size);
    }
    pub fn draw_text(&self, ctype: CoordinateType, x: i32, y: i32, text: &str) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        ffi::_game_drawText(g, ctype, x, y, text);
    }
    pub fn draw_text_map(&self, x: i32, y: i32, text: &str) {
        self.draw_text(CoordinateType::Map, x, y, text);
    }
    pub fn draw_box(
        &self,
        ctype: CoordinateType,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        color: Color,
        is_solid: bool,
    ) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.drawBox(ctype, left, top, right, bottom, color, is_solid);
    }
    pub fn draw_box_map(&self, left: i32, top: i32, right: i32, bottom: i32, color: Color, is_solid: bool) {
        self.draw_box(CoordinateType::Map, left, top, right, bottom, color, is_solid);
    }
    pub fn draw_triangle(
        &self,
        ctype: CoordinateType,
        ax: i32,
        ay: i32,
        bx: i32,
        by: i32,
        cx: i32,
        cy: i32,
        color: Color,
        is_solid: bool,
    ) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.drawTriangle(ctype, ax, ay, bx, by, cx, cy, color, is_solid);
    }
    pub fn draw_triangle_map(
        &self,
        ax: i32,
        ay: i32,
        bx: i32,
        by: i32,
        cx: i32,
        cy: i32,
        color: Color,
        is_solid: bool,
    ) {
        self.draw_triangle(CoordinateType::Map, ax, ay, bx, by, cx, cy, color, is_solid);
    }
    pub fn draw_circle(&self, ctype: CoordinateType, x: i32, y: i32, radius: i32, color: Color, is_solid: bool) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.drawCircle(ctype, x, y, radius, color, is_solid);
    }
    pub fn draw_circle_map(&self, x: i32, y: i32, radius: i32, color: Color, is_solid: bool) {
        self.draw_circle(CoordinateType::Map, x, y, radius, color, is_solid);
    }
    pub fn draw_ellipse(
        &self,
        ctype: CoordinateType,
        x: i32,
        y: i32,
        xrad: i32,
        yrad: i32,
        color: Color,
        is_solid: bool,
    ) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.drawEllipse(ctype, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_ellipse_map(&self, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, is_solid: bool) {
        self.draw_ellipse(CoordinateType::Map, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_dot(&self, ctype: CoordinateType, x: i32, y: i32, color: Color) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.drawDot(ctype, x, y, color);
    }
    pub fn draw_dot_map(&self, x: i32, y: i32, color: Color) {
        self.draw_dot(CoordinateType::Map, x, y, color);
    }
    pub fn draw_line(&self, ctype: CoordinateType, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw.unwrap().as_ptr()) };
        g.drawLine(ctype, x1, y1, x2, y2, color);
    }
    pub fn draw_line_map(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.draw_line(CoordinateType::Map, x1, y1, x2, y2, color);
    }
}

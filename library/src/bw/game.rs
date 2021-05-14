use crate::bw::color::{Color, TextSize};
use crate::bw::coordinate_type::CoordinateType;
use crate::bw::flag::Flag;
use crate::bw::forceset::Forceset;
use crate::bw::player::Player;
use crate::bw::playerset::Playerset;
use crate::bw::position::{Position, TilePosition};
use crate::bw::regionset::Regionset;
use crate::bw::tech_type::TechType;
use crate::bw::unit::Unit;
use crate::bw::unit_filter::UnitFilter;
use crate::bw::unit_type::UnitType;
use crate::bw::unitset::Unitset;
use crate::bw::upgrade_type::UpgradeType;
use crate::bw::{with_unit_and_best_filter, with_unit_filter, Handle};
use crate::{ffi, FromRaw};
use cxx::UniquePtr;
use std::pin::Pin;

#[derive(Debug)]
pub struct Game {
    pub(crate) raw: *mut ffi::Game,
}

/// Game object doesn't contain any self-references
impl Unpin for Game {}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn debug<F: Fn(Unit) -> bool + 'static>(&self, f: F) {
        let g: &ffi::Game = unsafe { &*self.raw };
        with_unit_filter(f, |uf| ffi::_game_debug(g, uf));
    }

    pub fn allies(&self) -> Playerset {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        let set: Pin<&mut ffi::Playerset> = g.allies();
        Playerset {
            raw: Handle::BorrowedMut(set),
        }
    }
    pub fn can_build_here(&self, position: TilePosition, utype: UnitType, builder: Unit, check_explored: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        unsafe { g.canBuildHere(position, utype, builder.raw.as_ptr(), check_explored) }
    }
    pub fn can_make(&self, utype: UnitType, builder: Unit) -> bool {
        let g: &ffi::Game = unsafe { &*self.raw };
        unsafe { g.canMake(utype, builder.raw.as_ptr()) }
    }
    pub fn can_research(&self, ttype: TechType, unit: Unit, check_can_issue_command_type: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        unsafe { g.canResearch(ttype, unit.raw.as_ptr(), check_can_issue_command_type) }
    }
    pub fn can_upgrade(&self, utype: UpgradeType, unit: Unit, check_can_issue_command_type: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        unsafe { g.canUpgrade(utype, unit.raw.as_ptr(), check_can_issue_command_type) }
    }
    pub fn countdown_timer(&self) -> i32 {
        let g: &ffi::Game = unsafe { &*self.raw };
        g.countdownTimer()
    }
    pub fn elapsed_time(&self) -> i32 {
        let g: &ffi::Game = unsafe { &*self.raw };
        g.elapsedTime()
    }
    pub fn enable_flag(&self, flag: Flag) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        ffi::_game_enableFlag(g, flag)
    }
    pub fn enemies(&self) -> Playerset {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        Playerset {
            raw: Handle::BorrowedMut(g.enemies()),
        }
    }
    pub fn enemy(&self) -> Player {
        let g: &ffi::Game = unsafe { &*self.raw };
        unsafe { Player::from_raw(g.enemy()) }
    }
    pub fn get_all_regions(&self) -> Regionset {
        let g: &ffi::Game = unsafe { &*self.raw };
        Regionset {
            raw: Handle::Borrowed(g.getAllRegions()),
        }
    }
    pub fn get_all_units(&self) -> Unitset {
        let g: &ffi::Game = unsafe { &*self.raw };
        Unitset {
            raw: Handle::Borrowed(g.getAllUnits()),
        }
    }
    pub fn get_apm(&self, include_selects: bool) -> i32 {
        let g: &ffi::Game = unsafe { &*self.raw };
        g.getAPM(include_selects)
    }
    pub fn get_average_fps(&self) -> f64 {
        let g: &ffi::Game = unsafe { &*self.raw };
        g.getAverageFPS()
    }
    pub fn get_best_unit(
        &self,
        best_fn: fn(Unit, Unit) -> Unit,
        unit_fn: fn(Unit) -> bool,
        center: Position,
        radius: i32,
    ) -> Option<Unit> {
        let g: &ffi::Game = unsafe { &*self.raw };
        with_unit_and_best_filter(unit_fn, best_fn, |uf, bf| {
            Unit::option(ffi::_game_getBestUnit(g, bf, uf, center, radius))
        })
    }

    // pub fn get_build_location(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getBuildLocation() }                                                          //            (self: &Game, unitType: UnitType, desiredPosition: TilePosition, maxRange: i32, creep: bool) -> TilePosition
    // pub fn get_bullets(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getBullets()  }                                                                       //                    (self: &Game) -> &Bulletset
    // pub fn get_client_version(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getClientVersion() }                                                          //            (self: &Game) -> i32
    // pub fn get_closest_unit(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getClosestUnit(g) }                                                     //                 (game: &Game, center: Position, pred: fn(Unit) -> bool, radius: i32) -> *mut UnitInterface; /
    // pub fn get_closest_unit_in_rectangle(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getClosestUnitInRectangle(g) }                             //    (game: &Game, center: Position, pred: fn(Unit) -> bool, left: i32, top: i32, right: i32, bottom: i32) -> *mut UnitInterface; /
    // pub fn get_damage_from(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getDamageFrom()  }                                                                //                (self: &Game, fromType: UnitType, toType: UnitType, fromPlayer: *mut PlayerInterface, toPlayer: *mut PlayerInterface) -> i32
    // pub fn get_damage_to(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getDamageTo()  }                                                                    //                  (self: &Game, toType: UnitType, fromType: UnitType, toPlayer: *mut PlayerInterface, fromPlayer: *mut PlayerInterface) -> i32
    // pub fn get_events(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getEvents(g) }                                                                //                       (game: &Game) -> &EventList; /
    // pub fn get_force(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getForce()  }                                                                           //                      (self: &Game, forceId: i32) -> *mut ForceInterface
    // pub fn get_forces(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getForces()  }                                                                         //                     (self: &Game) -> &Forceset
    // pub fn get_fps(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getFPS()  }                                                                               //                        (self: &Game) -> i32
    // pub fn get_frame_count(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getFrameCount()  }                                                                //                (self: &Game) -> i32
    // pub fn get_game_type(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getGameType()  }                                                                    //                  (self: &Game) -> GameType
    // pub fn get_geysers(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getGeysers()  }                                                                       //                    (self: &Game) -> &Unitset
    // pub fn get_ground_height(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getGroundHeight()  }                                                            //              (self: &Game, position: TilePosition) -> i32
    // pub fn get_instance_number(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getInstanceNumber() }                                                        //           (self: &Game) -> i32
    // pub fn get_key_state(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getKeyState()  }                                                                    //                  (self: &Game, key: Key) -> bool
    // pub fn get_last_event_time(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getLastEventTime() }                                                         //           (self: &Game) -> i32
    // pub fn get_latency(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getLatency()  }                                                                       //                    (self: &Game) -> i32
    // pub fn get_latency_frames(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getLatencyFrames() }                                                          //            (self: &Game) -> i32
    // pub fn get_latency_time(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getLatencyTime()  }                                                              //               (self: &Game) -> i32
    // pub fn get_minerals(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getMinerals()  }                                                                     //                   (self: &Game) -> &Unitset
    // pub fn get_mouse_position(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getMousePosition() }                                                          //            (self: &Game) -> Position
    // pub fn get_mouse_state(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getMouseState()  }                                                                //                (self: &Game, button: MouseButton) -> bool
    // pub fn get_neutral_units(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getNeutralUnits()  }                                                            //              (self: &Game) -> &Unitset
    // pub fn get_nuke_dots(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getNukeDots(g) }                                                           //                    (game: &Game) -> Vec<Position>; /
    // pub fn get_player(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getPlayer()  }                                                                         //                     (self: &Game, playerId: i32) -> *mut PlayerInterface
    // pub fn get_players(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getPlayers()  }                                                                       //                    (self: &Game) -> &Playerset
    // pub fn get_random_seed(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getRandomSeed()  }                                                                //                (self: &Game) -> u32
    // pub fn get_region(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getRegion()  }                                                                         //                     (self: &Game, regionID: i32) -> *mut RegionInterface
    // pub fn get_region_at(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getRegionAt()  }                                                                    //                  (self: &Game, position: Position) -> *mut RegionInterface
    // pub fn get_remaining_latency_frames(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getRemainingLatencyFrames() }                                        //  (self: &Game) -> i32
    // pub fn get_remaining_latency_time let g: &ffi::Game = unsafe { &*self.raw }; (&self) { g.getRemainingLatencyTime() }                                           //    (self: &Game) -> i32
    // pub fn get_replay_frame_count(& let g: &ffi::Game = unsafe { &*self.raw }; self) { g.getReplayFrameCount() }                                                   //        (self: &Game) -> i32
    // pub fn get_revision(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getRevision()  }                                                                     //                   (self: &Game) -> i32
    // pub fn get_screen_position(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getScreenPosition() }                                                        //           (self: &Game) -> Position
    // pub fn get_selected_units(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getSelectedUnits() }                                                          //            (self: &Game) -> &Unitset
    // pub fn get_start_locations(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getStartLocations(g) }                                               //              (game: &Game) -> Vec<TilePosition>; /
    // pub fn get_static_geysers(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getStaticGeysers() }                                                          //            (self: &Game) -> &Unitset
    // pub fn get_static_minerals(&self) let g: &ffi::Game = unsafe { &*self.raw };  { g.getStaticMinerals() }                                                        //           (self: &Game) -> &Unitset
    // pub fn get_static_neutral_units(& let g: &ffi::Game = unsafe { &*self.raw }; self) { g.getStaticNeutralUnits() }                                               //      (self: &Game) -> &Unitset
    // pub fn get_unit(&self) { let g: &ffi::Game = unsafe { &*self.raw }; g.getUnit()  }                                                                             //                       (self: &Game, unitID: i32) -> *mut UnitInterface
    // pub fn get_units_in_radius(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getUnitsInRadius(g) }                                                //              (game: &Game, position: Position, radius: i32, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>; /
    // pub fn get_units_in_rectangle(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getUnitsInRectangle(g) }                                          //           (game: &Game, topLeft: Position, bottomRight: Position, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>; /
    // pub fn get_units_on_tile(&self) { let g: &ffi::Game = unsafe { &*self.raw }; ffi::_game_getUnitsOnTile(g) }                                                    //                (game: &Game, tile: TilePosition, pred: fn(Unit) -> bool) -> UniquePtr<Unitset>; /
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
    // pub fn set_alliance(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setAlliance()  }                                    //                       (self: Pin<&mut Game>, player: *mut PlayerInterface, allied: bool, alliedVictory: bool) -> bool
    // pub fn set_command_optimization_level(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setCommandOptimizationLevel()  }  //                     (self: Pin<&mut                                       Game>, level: i32)
    // pub fn set_frame_skip(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setFrameSkip()  }                                 //                        (self: Pin<&mut Game>, frameSkip: i32)
    // pub fn set_gui(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setGUI()  }                                              //                  (self: Pin<&mut Game>, enabled: bool)
    // pub fn set_lat_com(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setLatCom()  }                                       //                     (self: Pin<&mut Game>, isEnabled: bool)
    // pub fn set_local_speed(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setLocalSpeed()  }                               //                         (self: Pin<&mut Game>, speed: i32)
    // pub fn set_map(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g._game_setMap()  }                                        //                        (game: Pin<&mut Game>, text: &str) -> bool
    // pub fn set_reveal_all(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setRevealAll()  }                                 //                        (self: Pin<&mut Game>, reveal: bool) -> bool
    // pub fn set_screen_position(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setScreenPosition()  }                       //                         (self: Pin<&mut Game>, p: Position)
    // pub fn set_vision(&self) { let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) }; g.setVision()  }                                        //                     (self: Pin<&mut Game>, player: *mut PlayerInterface, enabled: bool) -> bool

    pub fn set_alliance(&self, player: Player, allied: bool, allied_victory: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        unsafe { g.setAlliance(player.raw.as_ptr(), allied, allied_victory) }
    }

    pub fn get_forces(&self) -> Forceset {
        let g: &ffi::Game = unsafe { &*self.raw };
        Forceset {
            raw: Handle::Borrowed(g.getForces()),
        }
    }

    pub fn send_text(&self, text: &str) {
        ffi::_game_sendText(unsafe { Pin::new_unchecked(&mut *self.raw) }, text)
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
    }
    pub fn get_units_in_radius(&self, position: Position, radius: i32, _pred: UnitFilter) -> Unitset {
        let g: &ffi::Game = unsafe { &*self.raw };
        let set: UniquePtr<ffi::Unitset> = ffi::_game_getUnitsInRadius(g, position, radius, |_| true); // todo
        Unitset {
            raw: Handle::Owned(set),
        }
    }

    pub fn get_nuke_dots(&self) -> Vec<Position> {
        let g: &ffi::Game = unsafe { &*self.raw };
        ffi::_game_getNukeDots(g)
    }

    pub fn get_start_locations(&self) -> Vec<TilePosition> {
        let g: &ffi::Game = unsafe { &*self.raw };
        ffi::_game_getStartLocations(g)
    }

    // let ctype = ctype.unwrap_or(CoordinateType::Map);
    pub fn set_text_size(&self, size: TextSize) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.setTextSize(size);
    }
    pub fn draw_text(&self, ctype: CoordinateType, x: i32, y: i32, text: &str) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
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
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
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
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
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
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
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
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawEllipse(ctype, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_ellipse_map(&self, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, is_solid: bool) {
        self.draw_ellipse(CoordinateType::Map, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_dot(&self, ctype: CoordinateType, x: i32, y: i32, color: Color) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawDot(ctype, x, y, color);
    }
    pub fn draw_dot_map(&self, x: i32, y: i32, color: Color) {
        self.draw_dot(CoordinateType::Map, x, y, color);
    }
    pub fn draw_line(&self, ctype: CoordinateType, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawLine(ctype, x1, y1, x2, y2, color);
    }
    pub fn draw_line_map(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.draw_line(CoordinateType::Map, x1, y1, x2, y2, color);
    }
}

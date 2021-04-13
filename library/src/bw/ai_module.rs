use crate::bw::player::Player;
use crate::bw::position::Position;
use crate::bw::unit::Unit;

use crate::ffi;
use crate::bw;

pub trait AIMod {
    fn on_start(&mut self);
    fn on_end(&mut self, is_winner: bool);
    fn on_frame(&mut self);
    fn on_send_text(&mut self, text: String);
    fn on_receive_text(&mut self, player: &ffi::Player, text: String);
    fn on_player_left(&mut self, player: &ffi::Player);
    fn on_nuke_detect(&mut self, target: ffi::Position);
    fn on_unit_discover(&mut self, unit: &ffi::Unit);
    fn on_unit_evade(&mut self, unit: &ffi::Unit);
    fn on_unit_show(&mut self, unit: &ffi::Unit);
    fn on_unit_hide(&mut self, unit: &ffi::Unit);
    fn on_unit_create(&mut self, unit: &ffi::Unit);
    fn on_unit_destroy(&mut self, unit: &ffi::Unit);
    fn on_unit_morph(&mut self, unit: &ffi::Unit);
    fn on_unit_renegade(&mut self, unit: &ffi::Unit);
    fn on_save_game(&mut self, game_name: String);
    fn on_unit_complete(&mut self, unit: &ffi::Unit);
}

pub enum Event {
    OnStart(),
    OnEnd(bool),
    OnFrame(),
    OnSendText(String),
    OnReceiveText(Player, String),
    OnPlayerLeft(Player),
    OnNukeDetect(Position),
    OnUnitDiscover(Unit),
    OnUnitEvade(Unit),
    OnUnitShow(Unit),
    OnUnitHide(Unit),
    OnUnitCreate(Unit),
    OnUnitDestroy(Unit),
    OnUnitMorph(Unit),
    OnUnitRenegade(Unit),
    OnSaveGame(String),
    OnUnitComplete(Unit),
}
use crate::bw::player::Player;
use crate::bw::position::Position;
use crate::bw::unit::Unit;
use thin_trait_object::*;

#[thin_trait_object]
pub trait AIModule {
    fn on_event(&mut self, event: Event);
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
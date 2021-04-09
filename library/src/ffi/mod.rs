use cxx::CxxString;

#[cxx::bridge(namespace = BWAPI)]
pub mod root {
    unsafe extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");

        pub fn BWAPI_getRevision() -> i32;
        pub fn BWAPI_isDebug() -> bool;
    }
}

#[cxx::bridge(namespace = bwapi_ffi)]
pub mod ai_module {

    pub struct Position {
        x: i32,
        y: i32,
    }

    extern "Rust" {
        type AIModule;
        type Player;
        type Unit;

        fn onStart(self: &AIModule);
        fn onEnd(self: &AIModule, is_winner: bool);
        fn onFrame(self: &AIModule);
        fn onSendText(self: &AIModule, text: &CxxString);
        fn onReceiveText(self: &AIModule, player: &Player, text: &CxxString);
        fn onPlayerLeft(self: &AIModule, player: &Player);
        fn onNukeDetect(self: &AIModule, target: Position);
        fn onUnitDiscover(self: &AIModule, unit: &Unit);
        fn onUnitEvade(self: &AIModule, unit: &Unit);
        fn onUnitShow(self: &AIModule, unit: &Unit);
        fn onUnitHide(self: &AIModule, unit: &Unit);
        fn onUnitCreate(self: &AIModule, unit: &Unit);
        fn onUnitDestroy(self: &AIModule, unit: &Unit);
        fn onUnitMorph(self: &AIModule, unit: &Unit);
        fn onUnitRenegade(self: &AIModule, unit: &Unit);
        fn onSaveGame(self: &AIModule, game_name: &CxxString);
        fn onUnitComplete(self: &AIModule, unit: &Unit);
    }
}

struct AIModule;
struct Player;
struct Unit;

impl AIModule {
    fn onStart(&self) {}
    fn onEnd(&self, _is_winner: bool) {}
    fn onFrame(&self) {}
    fn onSendText(&self, _text: &CxxString) {}
    fn onReceiveText(&self, _player: &Player, _text: &CxxString) {}
    fn onPlayerLeft(&self, _player: &Player) {}
    fn onNukeDetect(&self, _target: ai_module::Position) {}
    fn onUnitDiscover(&self, _unit: &Unit) {}
    fn onUnitEvade(&self, _unit: &Unit) {}
    fn onUnitShow(&self, _unit: &Unit) {}
    fn onUnitHide(&self, _unit: &Unit) {}
    fn onUnitCreate(&self, _unit: &Unit) {}
    fn onUnitDestroy(&self, _unit: &Unit) {}
    fn onUnitMorph(&self, _unit: &Unit) {}
    fn onUnitRenegade(&self, _unit: &Unit) {}
    fn onSaveGame(&self, _game_name: &CxxString) {}
    fn onUnitComplete(&self, _unit: &Unit) {}
}
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

        #[rust_name = "on_start"]
        fn onStart(self: &mut AIModule);
        #[rust_name = "on_end"]
        fn onEnd(self: &mut AIModule, is_winner: bool);
        #[rust_name = "on_frame"]
        fn onFrame(self: &mut AIModule);
        #[rust_name = "on_send_text"]
        fn onSendText(self: &mut AIModule, text: &CxxString);
        #[rust_name = "on_receive_text"]
        fn onReceiveText(self: &mut AIModule, player: &Player, text: &CxxString);
        #[rust_name = "on_player_left"]
        fn onPlayerLeft(self: &mut AIModule, player: &Player);
        #[rust_name = "on_nuke_detect"]
        fn onNukeDetect(self: &mut AIModule, target: Position);
        #[rust_name = "on_unit_discover"]
        fn onUnitDiscover(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_evade"]
        fn onUnitEvade(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_show"]
        fn onUnitShow(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_hide"]
        fn onUnitHide(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_create"]
        fn onUnitCreate(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_destroy"]
        fn onUnitDestroy(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_morph"]
        fn onUnitMorph(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_unit_renegade"]
        fn onUnitRenegade(self: &mut AIModule, unit: &Unit);
        #[rust_name = "on_save_game"]
        fn onSaveGame(self: &mut AIModule, game_name: &CxxString);
        #[rust_name = "on_unit_complete"]
        fn onUnitComplete(self: &mut AIModule, unit: &Unit);
    }
}

struct AIModule;
struct Player;
struct Unit;

impl AIModule {
    fn on_start(&mut self) {}
    fn on_end(&mut self, _is_winner: bool) {}
    fn on_frame(&mut self) {}
    fn on_send_text(&mut self, _text: &CxxString) {}
    fn on_receive_text(&mut self, _player: &Player, _text: &CxxString) {}
    fn on_player_left(&mut self, _player: &Player) {}
    fn on_nuke_detect(&mut self, _target: ai_module::Position) {}
    fn on_unit_discover(&mut self, _unit: &Unit) {}
    fn on_unit_evade(&mut self, _unit: &Unit) {}
    fn on_unit_show(&mut self, _unit: &Unit) {}
    fn on_unit_hide(&mut self, _unit: &Unit) {}
    fn on_unit_create(&mut self, _unit: &Unit) {}
    fn on_unit_destroy(&mut self, _unit: &Unit) {}
    fn on_unit_morph(&mut self, _unit: &Unit) {}
    fn on_unit_renegade(&mut self, _unit: &Unit) {}
    fn on_save_game(&mut self, _game_name: &CxxString) {}
    fn on_unit_complete(&mut self, _unit: &Unit) {}
}


#[cxx::bridge(namespace = BWAPI)]
pub mod root {
    unsafe extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");

        #[rust_name = "bwapi_get_revision"]
        pub fn BWAPI_getRevision() -> i32;
        #[rust_name = "bwapi_is_debug"]
        pub fn BWAPI_isDebug() -> bool;
    }
}

#[cxx::bridge(namespace = BWAPI)]
pub mod ai_module {

    struct Position {
        x: i32,
        y: i32,
    }

    extern "Rust" {
        type Player;
        type Unit;

        type RustAIModule;
        #[rust_name = "on_start"]
        fn onStart(self: &RustAIModule);
        #[rust_name = "on_end"]
        fn onEnd(self: &RustAIModule, is_winner: bool);
        #[rust_name="on_frame"]
        fn onFrame(self: &RustAIModule);
        #[rust_name="on_send_text"]
        fn onSendText(&self: RustAIModule, text: &CxxString);
        #[rust_name="on_receive_text"]
        fn onReceiveText(&self: RustAIModule, player: &Player, text: &CxxString);
        #[rust_name="on_player_left"]
        fn onPlayerLeft(&self: RustAIModule, player: &Player);
        #[rust_name="on_nuke_detect"]
        fn onNukeDetect(&self: RustAIModule, target: Position);
        #[rust_name="on_unit_discover"]
        fn onUnitDiscover(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_evade"]
        fn onUnitEvade(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_show"]
        fn onUnitShow(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_hide"]
        fn onUnitHide(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_create"]
        fn onUnitCreate(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_destroy"]
        fn onUnitDestroy(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_morph"]
        fn onUnitMorph(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_unit_renegade"]
        fn onUnitRenegade(&self: RustAIModule, unit: &Unit);
        #[rust_name="on_save_game"]
        fn onSaveGame(&self: RustAIModule, game_name: &CxxString);
        #[rust_name="on_unit_complete"]
        fn onUnitComplete(&self: RustAIModule, unit: &Unit);
    }
}

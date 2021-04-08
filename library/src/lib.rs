
#[cxx::bridge]
pub mod ffi_main {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        pub fn cpp_main();
    }
}

#[cxx::bridge(namespace = BWAPI)]
pub mod ffi {

    unsafe extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");

        #[rust_name = "bwapi_get_revision"]
        fn BWAPI_getRevision() -> i32;
        #[rust_name = "bwapi_is_debug"]
        fn BWAPI_isDebug() -> bool;
    }

    extern "Rust" {
        // type AIModule;
        //
        // #[rust_name = "on_start"]
        // fn onStart(self: &AIModule);
        // #[rust_name = "on_end"]
        // fn onEnd(self: &AIModule, is_winner: bool);
        // #[rust_name="on_frame"]
        // fn onFrame(self: &AIModule);
        // #[rust_name="on_send_text"]
        // fn onSendText(&self: AIModule, text: &CxxString);
        // #[rust_name="on_receive_text"]
        // fn onReceiveText(&self: AIModule, player: Player, text: &CxxString);
        // #[rust_name="on_player_left"]
        // fn onPlayerLeft(&self: AIModule, player: Player);
        // #[rust_name="on_nuke_detect"]
        // fn onNukeDetect(&self: AIModule, target: Position);
        // #[rust_name="on_unit_discover"]
        // fn onUnitDiscover(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_evade"]
        // fn onUnitEvade(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_show"]
        // fn onUnitShow(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_hide"]
        // fn onUnitHide(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_create"]
        // fn onUnitCreate(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_destroy"]
        // fn onUnitDestroy(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_morph"]
        // fn onUnitMorph(&self: AIModule, unit: Unit);
        // #[rust_name="on_unit_renegade"]
        // fn onUnitRenegade(&self: AIModule, unit: Unit);
        // #[rust_name="on_save_game"]
        // fn onSaveGame(&self: AIModule, game_name: &CxxString);
        // #[rust_name="on_unit_complete"]
        // fn onUnitComplete(&self: AIModule, unit: Unit);
    }

    // #[namespace = "bwapi"]
    // extern "Rust" {
    // }
}

mod bw {
    trait AIModule {
        fn on_start(&self);
        fn on_frame(&self);
        fn on_end(&self, is_winner: bool);
    }

    struct DemoAI {
        name: String,
    }

    impl AIModule for DemoAI {
        fn on_start(&self) {
            println!("`on_start` called for {}", self.name);
        }

        fn on_frame(&self) {
            println!("`on_frame` called for {}", self.name);
        }

        fn on_end(&self, is_winner: bool) {
            println!("`on_end({})` called for {}", is_winner, self.name);
        }
    }
}

pub fn main() {
    // we don't need unsafe actually, just to satisfy IDE
    unsafe { ffi_main::cpp_main(); }
}

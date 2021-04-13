use cxx::{CxxString, UniquePtr};
use std::pin::Pin;
use std::fmt::{Debug, Formatter};
use std::fmt;
use crate::bw::ai_module::AIMod;
use crate::ffi::{Player, Position, Unit};

pub mod bw;

#[cxx::bridge]
pub mod ffi_main {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        fn cpp_main() -> i32;
    }
}

pub fn main() {
    // we don't need unsafe actually, IDE is mistaking
    ffi_main::cpp_main();
}

/// Box contains arbitrary user AiModule, needed to
/// provide a fixed size object for FFI.
pub struct AimBox(Box<dyn AIMod>);

#[cxx::bridge]
pub mod ffi {

    extern "Rust" {
        // #[namespace = "library::bw::ai_module"]
        type AimBox;
    }

    unsafe
    extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");

        #[namespace = "BWAPI"]
        pub fn BWAPI_getRevision() -> i32;
        #[namespace = "BWAPI"]
        pub fn BWAPI_isDebug() -> bool;

        type AIModuleWrapper;
        #[rust_name = "create_ai_module_wrapper"]
        unsafe fn createAIModuleWrapper(user_ai: &AimBox) -> UniquePtr<AIModuleWrapper>;
        #[rust_name = "aim_box"]
        fn getAimBox(wrapper: &AIModuleWrapper) -> &AimBox;

        #[namespace = "BWAPI"]
        type PlayerInterface;
        #[namespace = "BWAPI"]
        type UnitInterface;
    }

    #[derive(Debug, Clone)]
    pub struct Player { pub raw: *const PlayerInterface }

    #[derive(Debug, Clone)]
    pub struct Unit { pub raw: *const UnitInterface }

    #[derive(Debug, Copy, Clone)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    extern "Rust" {
        include!("library/src/aim.h");

        fn on_start(wrapper: &AIModuleWrapper);
        fn on_end(wrapper: &AIModuleWrapper, is_winner: bool);
        fn on_frame(wrapper: &AIModuleWrapper);
        fn on_send_text(wrapper: &AIModuleWrapper, text: &CxxString);
        fn on_receive_text(wrapper: &AIModuleWrapper, player: &Player, text: &CxxString);
        fn on_player_left(wrapper: &AIModuleWrapper, player: &Player);
        fn on_nuke_detect(wrapper: &AIModuleWrapper, target: Position);
        fn on_unit_discover(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_evade(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_show(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_hide(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_create(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_destroy(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_morph(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_unit_renegade(wrapper: &AIModuleWrapper, unit: &Unit);
        fn on_save_game(wrapper: &AIModuleWrapper, game_name: &CxxString);
        fn on_unit_complete(wrapper: &AIModuleWrapper, unit: &Unit);
    }
}

// region ----------- fn [***](wrapper: Pin<&mut ffi::AIModuleWrapper>) ------------
fn on_start(wrapper: &ffi::AIModuleWrapper) {

}
fn on_end(wrapper: &ffi::AIModuleWrapper, is_winner: bool) {

}
fn on_frame(wrapper: &ffi::AIModuleWrapper) {

}
fn on_send_text(wrapper: &ffi::AIModuleWrapper, text: &CxxString) {

}
fn on_receive_text(wrapper: &ffi::AIModuleWrapper, player: &ffi::Player, text: &CxxString) {

}
fn on_player_left(wrapper: &ffi::AIModuleWrapper, player: &ffi::Player) {

}
fn on_nuke_detect(wrapper: &ffi::AIModuleWrapper, target: ffi::Position) {

}
fn on_unit_discover(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_evade(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_show(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_hide(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_create(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_destroy(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_morph(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_unit_renegade(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
fn on_save_game(wrapper: &ffi::AIModuleWrapper, game_name: &CxxString) {

}
fn on_unit_complete(wrapper: &ffi::AIModuleWrapper, unit: &ffi::Unit) {

}
// ------------------- endregion -------------------

impl Debug for ffi::AIModuleWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AIMW({:p})", self)
    }
}

#[derive(Debug, Clone)]
pub struct RustAIModule(pub String);

impl AIMod for RustAIModule {
    fn on_start(&mut self) {
        println!("fn on_start()");
    }

    fn on_end(&mut self, is_winner: bool) {
        println!("fn on_end(is_winner: {})", is_winner);
    }

    fn on_frame(&mut self) {
        println!("fn on_frame()");
    }

    fn on_send_text(&mut self, text: String) {
        println!("fn on_send_text(text: {})", text);
    }

    fn on_receive_text(&mut self, player: &Player, text: String) {
        println!("fn on_receive_text(player: {:?}, text: {})", player, text)
    }

    fn on_player_left(&mut self, player: &Player) {
        println!("fn on_player_left(player: {:?})", player);
    }

    fn on_nuke_detect(&mut self, target: Position) {
        println!("fn on_nuke_detect(target: {:?})", target);
    }

    fn on_unit_discover(&mut self, unit: &Unit) {
        println!("fn on_unit_discover(unit: {:?})", unit);
    }

    fn on_unit_evade(&mut self, unit: &Unit) {
        println!("fn on_unit_evade(unit: {:?})", unit);
    }

    fn on_unit_show(&mut self, unit: &Unit) {
        println!("fn on_unit_show(unit: {:?})", unit);
    }

    fn on_unit_hide(&mut self, unit: &Unit) {
        println!("fn on_unit_hide(unit: {:?})", unit);
    }

    fn on_unit_create(&mut self, unit: &Unit) {
        println!("fn on_unit_create(unit: {:?})", unit);
    }

    fn on_unit_destroy(&mut self, unit: &Unit) {
        println!("fn on_unit_destroy(unit: {:?})", unit);
    }

    fn on_unit_morph(&mut self, unit: &Unit) {
        println!("fn on_unit_morph(unit: {:?})", unit);
    }

    fn on_unit_renegade(&mut self, unit: &Unit) {
        println!("fn on_unit_renegade(unit: {:?})", unit);
    }

    fn on_save_game(&mut self, game_name: String) {
        println!("fn on_save_game(, text: {})", game_name);
    }

    fn on_unit_complete(&mut self, unit: &Unit) {
        println!("fn on_unit_complete(unit: {:?})", unit);
    }
}

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
#[allow(non_snake_case)]
pub unsafe extern "C" fn gameInit(game: *const std::ffi::c_void) {
    println!("gameInit called: game = {:?}", game);
    // TODO assign game to the BW global
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut ffi::AIModuleWrapper {
    println!("newAIModule called!");
    let r = RustAIModule("RustAIModule here".to_string());
    let b = AimBox(Box::new(r));
    let ai: UniquePtr<ffi::AIModuleWrapper> = ffi::create_ai_module_wrapper(&b);
    ai.into_raw()
}

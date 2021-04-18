#![feature(never_type)]

pub mod bw;
pub mod prelude;

use cxx::CxxString;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::pin::Pin;
use crate::prelude::{AIModule, Event, Game, GAME, BoxedAIModule};
use once_cell::sync::OnceCell;

#[cxx::bridge]
pub mod ffi_test {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        fn cpp_test() -> i32;
    }
}

/// Box contains arbitrary user AiModule, needed to
/// provide a fixed size object for FFI.
// pub struct AimBox(pub Box<dyn AIModule + Send + Sync>);

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug, Copy, Clone)]
    struct Position {
        x: i32,
        y: i32,
    }

    extern "Rust" {
        // #[namespace = "crate::bw::ai_module"]
        type BoxedAIModule<'a>;
    }

    #[namespace = "BWAPI"]
    unsafe extern "C++" {
        include!("library/openbw/bwapilib/include/BWAPI.h");

        pub fn BWAPI_getRevision() -> i32;
        pub fn BWAPI_isDebug() -> bool;

        pub type PlayerInterface;
        pub type UnitInterface;
        pub type Game;
    }

    // Game
    extern "C++" {
        unsafe fn getFrameCount(game: *mut Game) -> i32;
        unsafe fn sendText(game: *mut Game, text: *const c_char);
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

// region ----------- Shims to the bw::ai_module::AIModule trait ------------
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
    let player = crate::bw::player::Player { raw: player };
    wrapper.get_box().on_event(Event::OnReceiveText(player, text.to_string()));
}
fn on_player_left(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface) {
    let player = crate::bw::player::Player { raw: player };
    wrapper.get_box().on_event(Event::OnPlayerLeft(player));
}
fn on_nuke_detect(wrapper: Pin<&mut ffi::AIModuleWrapper>, target: ffi::Position) {
    let target = crate::bw::position::Position { x: target.x, y: target.y };
    wrapper.get_box().on_event(Event::OnNukeDetect(target));
}
fn on_unit_discover(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitDiscover(unit));
}
fn on_unit_evade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitEvade(unit));
}
fn on_unit_show(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitShow(unit));
}
fn on_unit_hide(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitHide(unit));
}
fn on_unit_create(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitCreate(unit));
}
fn on_unit_destroy(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitDestroy(unit));
}
fn on_unit_morph(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitMorph(unit));
}
fn on_unit_renegade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitRenegade(unit));
}
fn on_save_game(wrapper: Pin<&mut ffi::AIModuleWrapper>, game_name: &CxxString) {
    wrapper.get_box().on_event(Event::OnSaveGame(game_name.to_string()));
}
fn on_unit_complete(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.get_box().on_event(Event::OnUnitComplete(unit));
}
// ------------------- endregion -------------------

impl Debug for ffi::AIModuleWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ffi::AIModuleWrapper({:p})", self)
    }
}

// used for testing
pub static HACK_BOX: OnceCell<BoxedAIModule> = OnceCell::new();
fn hack() -> &'static BoxedAIModule<'static> { &HACK_BOX.get().unwrap() }

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
    *GAME.lock().unwrap() = Game { raw: game as *const ffi::Game };
}

use cxx::CxxString;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::pin::Pin;
use once_cell::sync::Lazy;
use crate::bw::ai_module::AIModule;
use crate::bw::player::Player;
use crate::bw::position::Position;
use crate::bw::unit::Unit;

pub mod bw;

#[cxx::bridge]
pub mod ffi_main {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        fn cpp_main() -> i32;
    }
}

/// Box contains arbitrary user AiModule, needed to
/// provide a fixed size object for FFI.
// TODO research why the definition below results in SEGFAULT
pub struct AimBox(pub Box<dyn AIModule + Send + Sync>);
// pub struct AimBox<T: AIModule>(Box<T>);

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

        pub type AIModuleWrapper;
        #[rust_name = "create_ai_module_wrapper"]
        fn createAIModuleWrapper(user_ai: &mut AimBox) -> UniquePtr<AIModuleWrapper>;
        #[rust_name = "aim_box"]
        pub fn getAimBox(self: Pin<&mut AIModuleWrapper>) -> &mut AimBox;

        #[namespace = "BWAPI"]
        type PlayerInterface;
        #[namespace = "BWAPI"]
        type UnitInterface;
    }

    // #[derive(Debug, Clone)]
    // pub struct Player { raw: *const PlayerInterface }
    //
    // #[derive(Debug, Clone)]
    // pub struct Unit { raw: *const UnitInterface }

    #[derive(Debug, Copy, Clone)]
    struct Position {
        x: i32,
        y: i32,
    }

    extern "Rust" {
        include!("library/src/aim.h");
        // the hack is to create AimBox to create AIModuleWrapper on the C++ side
        fn hack() -> &'static AimBox;

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

struct TestAI;
impl AIModule for TestAI {
    fn on_start(&mut self) {}
    fn on_end(&mut self, _is_winner: bool) {}
    fn on_frame(&mut self) {}
    fn on_send_text(&mut self, _text: String) {}
    fn on_receive_text(&mut self, _player: Player, _text: String) {}
    fn on_player_left(&mut self, _player: Player) {}
    fn on_nuke_detect(&mut self, _target: Position) {}
    fn on_unit_discover(&mut self, _unit: Unit) {}
    fn on_unit_evade(&mut self, _unit: Unit) {}
    fn on_unit_show(&mut self, _unit: Unit) {}
    fn on_unit_hide(&mut self, _unit: Unit) {}
    fn on_unit_create(&mut self, _unit: Unit) {}
    fn on_unit_destroy(&mut self, _unit: Unit) {}
    fn on_unit_morph(&mut self, _unit: Unit) {}
    fn on_unit_renegade(&mut self, _unit: Unit) {}
    fn on_save_game(&mut self, _game_name: String) {}
    fn on_unit_complete(&mut self, _unit: Unit) {}
}
static BOX: Lazy<AimBox> = Lazy::new(|| AimBox(Box::new(TestAI)));
fn hack() -> &'static AimBox { &BOX }

// region ----------- Shims to the bw::ai_module::AIModule trait ------------
fn on_start(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    wrapper.aim_box().0.on_start();
}
fn on_end(wrapper: Pin<&mut ffi::AIModuleWrapper>, is_winner: bool) {
    wrapper.aim_box().0.on_end(is_winner);
}
fn on_frame(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    wrapper.aim_box().0.on_frame();
}
fn on_send_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, text: &CxxString) {
    wrapper.aim_box().0.on_send_text(text.to_string());
}
fn on_receive_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface, text: &CxxString) {
    let player = crate::bw::player::Player { raw: player };
    wrapper.aim_box().0.on_receive_text(player, text.to_string());
}
fn on_player_left(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: *const ffi::PlayerInterface) {
    let player = crate::bw::player::Player { raw: player };
    wrapper.aim_box().0.on_player_left(player);
}
fn on_nuke_detect(wrapper: Pin<&mut ffi::AIModuleWrapper>, target: ffi::Position) {
    let target = crate::bw::position::Position { x: target.x, y: target.y };
    wrapper.aim_box().0.on_nuke_detect(target);
}
fn on_unit_discover(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_discover(unit);
}
fn on_unit_evade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_evade(unit);
}
fn on_unit_show(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_show(unit);
}
fn on_unit_hide(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_hide(unit);
}
fn on_unit_create(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_create(unit);
}
fn on_unit_destroy(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_destroy(unit);
}
fn on_unit_morph(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_morph(unit);
}
fn on_unit_renegade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_renegade(unit);
}
fn on_save_game(wrapper: Pin<&mut ffi::AIModuleWrapper>, game_name: &CxxString) {
    wrapper.aim_box().0.on_save_game(game_name.to_string());
}
fn on_unit_complete(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: *const ffi::UnitInterface) {
    let unit = crate::bw::unit::Unit { raw: unit };
    wrapper.aim_box().0.on_unit_complete(unit);
}
// ------------------- endregion -------------------

impl Debug for ffi::AIModuleWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "AIMWrapper({:p})", self)
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

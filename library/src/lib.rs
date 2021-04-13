use cxx::{CxxString, UniquePtr};
use std::pin::Pin;

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

#[cxx::bridge]
pub mod ffi {

    unsafe
    extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");

        #[namespace = "BWAPI"]
        pub fn BWAPI_getRevision() -> i32;
        #[namespace = "BWAPI"]
        pub fn BWAPI_isDebug() -> bool;

        type AIModuleWrapper;
        #[rust_name = "create_ai_module_wrapper"]
        fn createAIModuleWrapper() -> UniquePtr<AIModuleWrapper>;

        #[namespace = "BWAPI"]
        type PlayerInterface;
        #[namespace = "BWAPI"]
        type UnitInterface;
    }

    #[derive(Debug, Clone)]
    struct Player { pub raw: *const PlayerInterface }

    #[derive(Debug, Clone)]
    struct Unit { pub raw: *const UnitInterface }

    #[derive(Debug, Copy, Clone)]
    struct Position {
        pub x: i32,
        pub y: i32,
    }

    extern "Rust" {
        include!("library/src/aim.h");

        fn on_start(wrapper: Pin<&mut AIModuleWrapper>);
        fn on_end(wrapper: Pin<&mut AIModuleWrapper>, is_winner: bool);
        fn on_frame(wrapper: Pin<&mut AIModuleWrapper>);
        fn on_send_text(wrapper: Pin<&mut AIModuleWrapper>, text: &CxxString);
        fn on_receive_text(wrapper: Pin<&mut AIModuleWrapper>, player: &Player, text: &CxxString);
        fn on_player_left(wrapper: Pin<&mut AIModuleWrapper>, player: &Player);
        fn on_nuke_detect(wrapper: Pin<&mut AIModuleWrapper>, target: Position);
        fn on_unit_discover(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_evade(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_show(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_hide(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_create(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_destroy(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_morph(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_unit_renegade(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
        fn on_save_game(wrapper: Pin<&mut AIModuleWrapper>, game_name: &CxxString);
        fn on_unit_complete(wrapper: Pin<&mut AIModuleWrapper>, unit: &Unit);
    }
}

fn on_start(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    println!("fn on_start(self: {:p})", wrapper);
}
fn on_end(wrapper: Pin<&mut ffi::AIModuleWrapper>, is_winner: bool) {
    println!("fn on_end(self: {:p}, is_winner: {})", wrapper, is_winner);
}
fn on_frame(wrapper: Pin<&mut ffi::AIModuleWrapper>) {
    println!("fn on_frame(self: {:p})", wrapper);
}
fn on_send_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, text: &CxxString) {
    println!("fn on_send_text(self: {:p}, text: {})", wrapper, text);
}
fn on_receive_text(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: &ffi::Player, text: &CxxString) {
    println!("fn on_receive_text(wrapper: {:p}, player: {:p}, text: {})", wrapper, player.raw, text)
}
fn on_player_left(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: &ffi::Player) {
    println!("fn on_player_left(wrapper: {:p}, player: {:p})", wrapper, player.raw);
}
fn on_nuke_detect(wrapper: Pin<&mut ffi::AIModuleWrapper>, target: ffi::Position) {
    println!("fn on_nuke_detect(wrapper: {:p}, target: {:?})", wrapper, target);
}
fn on_unit_discover(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_discover(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_evade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_evade(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_show(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_show(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_hide(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_hide(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_create(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_create(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_destroy(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_destroy(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_morph(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_morph(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_unit_renegade(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_renegade(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}
fn on_save_game(wrapper: Pin<&mut ffi::AIModuleWrapper>, game_name: &CxxString) {
    println!("fn on_save_game(self: {:p}, text: {})", wrapper, game_name);
}
fn on_unit_complete(wrapper: Pin<&mut ffi::AIModuleWrapper>, unit: &ffi::Unit) {
    println!("fn on_unit_complete(wrapper: {:p}, unit: {:p})", wrapper, unit.raw);
}




























#[derive(Debug, Clone)]
pub struct RustAIModule(pub String);

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
    let ai: UniquePtr<ffi::AIModuleWrapper> = ffi::create_ai_module_wrapper();
    ai.into_raw()
}

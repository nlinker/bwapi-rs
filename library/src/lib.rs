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

    struct Player { pub raw: *const PlayerInterface }
    struct Unit { pub raw: *const UnitInterface }

    extern "Rust" {
        include!("library/src/aim.h");

        #[rust_name = "on_start"]
        fn onStart(wrapper: Pin<&mut AIModuleWrapper>);
        #[rust_name = "on_end"]
        fn onEnd(wrapper: Pin<&mut AIModuleWrapper>, is_winner: bool);
        #[rust_name = "on_frame"]
        fn onFrame(wrapper: Pin<&mut AIModuleWrapper>);
        #[rust_name = "on_send_text_shim"]
        fn onSendText_shim(wrapper: Pin<&mut AIModuleWrapper>, text: UniquePtr<CxxString>);
        #[rust_name = "on_receive_text_shim"]
        fn onReceiveText_shim(wrapper: Pin<&mut AIModuleWrapper>, player: Player, text: UniquePtr<CxxString>);
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
fn on_send_text_shim(wrapper: Pin<&mut ffi::AIModuleWrapper>, text: UniquePtr<CxxString>) {
    println!("fn on_send_text(self: {:p}, text: {})", wrapper, text);
}
fn on_receive_text_shim(wrapper: Pin<&mut ffi::AIModuleWrapper>, player: ffi::Player, text: UniquePtr<CxxString>) {
    println!("fn on_receive_text(self: {:p}, player: {:p}, text: {})", wrapper, player.raw, text);
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

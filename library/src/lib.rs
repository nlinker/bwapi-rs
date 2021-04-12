use std::pin::Pin;
use cxx::UniquePtr;

pub mod bw;

#[cxx::bridge]
pub mod ffi_main {
    unsafe extern "C++" {
    include!("library/src/lib.h");
    unsafe fn cpp_main() -> i32;
}
}

pub fn main() {
    // we don't need unsafe actually, IDE is mistaking
    unsafe { ffi_main::cpp_main(); }
}

#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");

        #[namespace = "BWAPI"]
        pub fn BWAPI_getRevision() -> i32;
        #[namespace = "BWAPI"]
        pub fn BWAPI_isDebug() -> bool;

        type AIModuleWrapper;
        #[rust_name="create_ai_module_wrapper"]
        fn createAIModuleWrapper() -> UniquePtr<AIModuleWrapper>;
    }

    extern "Rust" {
        #[rust_name="on_start"]
        fn onStart(self: Pin<&mut AIModuleWrapper>);
        #[rust_name="on_end"]
        fn onEnd(self: Pin<&mut AIModuleWrapper>, is_winner: bool);
        #[rust_name="on_frame"]
        fn onFrame(self: Pin<&mut AIModuleWrapper>);
    }
}

impl ffi::AIModuleWrapper {
    fn on_start(this: Pin<&mut ffi::AIModuleWrapper>) {
        println!("fn on_start(self: {:p})", this);
    }
    fn on_end(this: Pin<&mut ffi::AIModuleWrapper>, is_winner: bool) {
        println!("fn on_end(self: {:p}, is_winner: {})", this, is_winner);
    }
    fn on_frame(this: Pin<&mut ffi::AIModuleWrapper>) {
        println!("fn on_frame(self: {:p})", this);
    }
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

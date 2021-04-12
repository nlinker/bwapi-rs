use std::pin::Pin;
use std::ptr::null_mut;

pub mod bw;

#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("library/bwapilib/include/BWAPI.h");
        #[namespace = "BWAPI"]
        pub fn BWAPI_getRevision() -> i32;
        #[namespace = "BWAPI"]
        pub fn BWAPI_isDebug() -> bool;

        include!("library/src/lib.h");
        type AIModuleWrapper;
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
pub unsafe extern "C" fn gameInit(game: *mut std::ffi::c_void) {
    println!("gameInit called: game = {:?}", game);
    println!("std::env::current_dir = {:?}", std::env::current_dir());
    // TODO assign game to the BW global
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut std::ffi::c_void {
    println!("newAIModule called!");
    null_mut()
}

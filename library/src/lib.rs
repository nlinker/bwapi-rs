use std::ptr::null_mut;
use std::ffi::c_void;

pub mod bw;
pub mod ffi;

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

#[cxx::bridge]
pub mod ffi_main {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        pub fn cpp_main() -> i32;
    }
}

pub fn main() {
    // we don't need unsafe actually, IDE is mistaking
    ffi_main::cpp_main();
}

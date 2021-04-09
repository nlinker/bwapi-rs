pub mod bw;
pub mod ffi;

#[cxx::bridge]
pub mod ffi_main {
    unsafe extern "C++" {
        include!("library/src/lib.h");
        pub fn cpp_main();
    }
}

pub fn main() {
    // we don't need unsafe actually, IDE is mistaking
    ffi_main::cpp_main();
}

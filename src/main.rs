#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("bwapi-rice/bwapi/include/BWAPI.h");

        #[namespace="BWAPI"]
        #[rust_name = "get_revision"]
        fn BWAPI_getRevision() -> i32;

        #[namespace="BWAPI"]
        #[rust_name = "is_debug"]
        fn BWAPI_isDebug() -> bool;
    }

    // #[namespace = "bwapi"]
    // extern "Rust" {
    // }
}

fn main() {
    let r = crate::ffi::get_revision();
    let d = crate::ffi::is_debug();
    println!("{} {}", r, d);
}

extern crate cmake;

use std::env;
use std::path::PathBuf;
// use cmake::Config;

// borrowed from tensorflow/rust
macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($expression:expr) => ($expression.unwrap()));
macro_rules! log {
    ($fmt:expr) => (println!(concat!("bwapi-sys/build.rs:{}: ", $fmt), line!()));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("bwapi-sys/build.rs:{}: ", $fmt),
    line!(), $($arg)*));
}
macro_rules! log_var (($var:ident) => (log!(concat!(stringify!($var), " = {:?}"), $var)));

fn main() {
    let output_dir = PathBuf::from(&get!("OUT_DIR"));
    let target = &get!("TARGET");
    let build_mode = &get!("PROFILE");
    let cur_dir = PathBuf::from(&get!("CARGO_MANIFEST_DIR"));
    let subtrees_dir = cur_dir.join("subtrees");

    log_var!(output_dir);
    log_var!(target);
    log_var!(build_mode);
    log_var!(cur_dir);
    log_var!(subtrees_dir);

    // OpenBW/bwapi is being built by cmake
    // let openbw_dir = subtrees_dir.join("openbw");
    // let bwapi_dir = subtrees_dir.join("bwapi");
    // log_var!(openbw_dir);
    // log_var!(bwapi_dir);
    // let bwapi_build_dir = Config::new(&bwapi_dir)
    //     .define("OPENBW_DIR", &openbw_dir)
    //     .define("OPENBW_ENABLE_UI", "0")
    //     .build();
    // log_var!(bwapi_build_dir);

    let bwapi_include_dir = cur_dir.join("bwapi").join("include");
    let bwapi_lib_dir = cur_dir.join("bwapi").join("lib");

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .include(bwapi_include_dir)
        .compile("rice");

    println!("cargo:rustc-link-search=native={}", bwapi_lib_dir.display());
    // Phase `BWAPILIB` here stands for the library name (without lib prefix and without .dylib suffix)
    println!("cargo:rustc-link-lib=dylib=BWAPILIB");
}

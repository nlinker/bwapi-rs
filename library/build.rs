use cmake::Config;
use std::env;
use std::path::PathBuf;

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
    let third_party_dir = cur_dir.join("3rdparty");

    log_var!(output_dir);
    log_var!(target);
    log_var!(build_mode);
    log_var!(cur_dir);
    log_var!(third_party_dir);

    // OpenBW/bwapi is being built by cmake
    let bwapi_build_dir = Config::new(&third_party_dir).build();
    let bwapi_include_dir = cur_dir.join("3rdparty").join("bwapi").join("bwapi").join("include");
    let bwapi_lib_dir = bwapi_build_dir.join("lib");

    log_var!(bwapi_build_dir);
    log_var!(bwapi_include_dir);
    log_var!(bwapi_lib_dir);

    let source_files = vec!["src/lib.rs"];
    cxx_build::bridges(source_files)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .include(bwapi_include_dir.clone())
        .include("src")
        .file("src/lib.cc")
        .compile("bwapi_xi");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.cc");
    println!("cargo:rerun-if-changed=src/lib.h");

    println!("cargo:rustc-link-search=native={}", bwapi_lib_dir.display());
    // Phase `BWAPILIB` here stands for the library name (without lib prefix and without .dylib suffix)
    println!("cargo:rustc-link-lib=dylib=BWAPILIB");
}

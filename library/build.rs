use std::env;
use std::fs;
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

    let bwapi_include_dir = cur_dir.join("openbw").join("bwapilib").join("include");
    let bwapi_lib_dir = cur_dir.join("openbw").join("bwapilib").join("lib");

    log_var!(bwapi_include_dir);

    let source_files = vec!["src/lib.rs"];
    cxx_build::bridges(source_files)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .include(bwapi_include_dir.clone())
        .include("src")
        .file("src/lib.cc")
        .compile("bwapi_xi");

    let openbw_libs = ["libBWAPILIB.dylib", "libBWAPILIB.dylib.txt"];
    for l in &openbw_libs {
        fs::copy(bwapi_lib_dir.join(l), output_dir.join(l)).unwrap();
    }

    // cc::Build::new()
    //     .file("src/lib.cpp")
    //     .include(bwapi_include_dir)
    //     .compile("rice-c");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.cc");
    println!("cargo:rerun-if-changed=src/lib.h");

    println!("cargo:rustc-link-search=native={}", bwapi_lib_dir.display());
    // Phase `BWAPILIB` here stands for the library name (without lib prefix and without .dylib suffix)
    println!("cargo:rustc-link-lib=dylib=BWAPILIB");
}

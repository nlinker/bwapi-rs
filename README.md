# `BWAPI` o`XI`dized library
 
_Oxidized BWAPI_ is the native [Rust](https://github.com/rust-lang/rust) bindings 
to [BWAPI](https://github.com/bwapi/bwapi). Main objectives:

- Create full coverage for BWAPI, but with Rust-friendly data structures. 

- Make it [OpenBW](https://github.com/OpenBW/openbw) first, which means this library
  is cross-platform and oriented for machine learning environments,
  such as PyTorch and Tensorflow.
  
- Make it still possible to run under original Starcraft Broodwar 1.16.1 
  using [sc-docker](https://github.com/basil-ladder/sc-docker).
  
- Support both modes for AI: 
  - _Module library_ mode (more performance) and
  - _Client executable_ mode (more convenient to debug, but less performance, the
  game state is exchanged over network or pipe).

__Warning:__ _the project is on the very early stage and pretty much unstable._

## Installation and run

### MacOS

Standalone test
```shell
cd /Users/nick/rust/scai/bwapi-xi
DYLD_LIBRARY_PATH=library/bwapilib/lib cargo run --package library --bin library
```

To run inside OpenBW you need to take `broodat.mpq`, `patch_rt.mpq` and `stardat.mpq` from BW,
distribution, make sure the version is `1.16.1`.

```shell
cargo build
cp broodat.mpq patch_rt.mpq stardat.mpq target/debug
cd launcher

# make sure this path is correct
# ai = ../target/debug/liblibrary.dylib
vim bwapi-data/bwapi.ini

DYLD_LIBRARY_PATH=library/openbw/bwapilib/lib ./openbw_launcher
```

### Linux
TODO

### Windows
TODO

## Development environment small tasks

### Remove CRLFs from BWAPI include header files
```shell
pwd  # something like ~/rust/scai/bwapi-rice
cd library/openbw/include
find . -type f -print0 | xargs -0 dos2unix
```

### Remove exec flag from the files, copied from Windows
```shell
find /path/to/location -type f -print0 | xargs -0 chmod 644
```

### Clean up CMake build dir
```shell
cd cmake-build-debug
cmake --build . --target clean
```

### Useful links

- Pretty complex `build.rs` to learn from: https://github.com/alexcrichton/curl-rust/blob/master/curl-sys/build.rs

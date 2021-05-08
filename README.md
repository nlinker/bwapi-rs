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
distribution, make sure the version is `1.16.1`. Check their hashes to be ensure they have the
version needed.

```shell
sha1sum *.mpq
# f05fb5bb9bb17d9565f0534609dcbcf221a6721f  broodat.mpq
# e97bfe875b17ca6b85a58026188d814956dff503  patch_rt.mpq
# 25a613851fe9e0d20d2f073525277bc01d291c92  stardat.mpq

cargo build
cp broodat.mpq patch_rt.mpq stardat.mpq target/debug
cd launcher

# make sure this path is correct
# ai = ../target/debug/libdemo_ai.dylib
vim bwapi-data/bwapi.ini

cd ../demo_ai && cargo build && cd ../launcher && ./openbw_launcher
```

### Linux
TODO

### Windows
TODO

## Differences from the original API

1. Method `Game::self() -> Player` renamed to `self_()` since `self` is a keyword in Rust. 
   The same is for `move_`.
2. Many overloaded methods "compressed" to one the most general method, since Rust 
   has the only overloading on traits. Others has suffixes.
3. `Bulletset`, `Forceset`, `Playerset`, `Regionset` and `Unitset` are not sets themselves, but
   have instead `iter()` method, that return standard Rust iterator.


## Development small tasks

- Quick convert C++ code snippets into Rust: https://c2rust.com/

- Remove CRLFs from BWAPI include header files
  ```shell
  cd ~/bwapi-xi/library/openbw/include
  find . -type f -print0 | xargs -0 dos2unix
  ```

- Remove exec flag from the files, copied from Windows
  ```shell
  find . -type f -print0 | xargs -0 chmod 644
  ```

- Clean up CMake build dir
  ```shell
  cd cmake-build-debug
  cmake --build . --target clean
  ```

### Useful links

- Pretty complex `build.rs` to learn from: https://github.com/alexcrichton/curl-rust/blob/master/curl-sys/build.rs
- Excellently described question and answers on passing Rust closures into C, useful for `UnitFilter`s: 
  [Stateful closure as FFI callback](https://users.rust-lang.org/t/stateful-closure-for-ffi-callback/29339)

### Thanks

This piece of software would be impossible without prior work of many people. It is based
on the work by global community behind Rust programming language, enthusiasts that develop BWAPI 
and OpenBW and the last, but not least, gamedev pros from Blizzard, who created the original Starcraft.
So it is pretty hard to enumerate all the people, however I want to notice the latest effort from:

- [@dtolnay](https://github.com/dtolnay) for his wonderful [cxx.rs library](https://github.com/dtolnay/cxx) for the new interop between C++ and Rust;
- [@kpp](https://github.com/kpp) and [@0x7CFE](https://github.com/0x7CFE) for their earlier work on `bwapi-rs` that I learned a lot from.
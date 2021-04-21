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

1. Method `Game::self() -> Player` renamed to `self_player()` since `self` is a keyword in Rust.
2. Many overloaded methods "compressed" to one the most general method, since Rust 
   has the only overloading on traits. Others has suffixes.


## Development small tasks

- Remove CRLFs from BWAPI include header files
  ```shell
  pwd  # something like ~/rust/scai/bwapi-rice
  cd library/openbw/include
  find . -type f -print0 | xargs -0 dos2unix
  ```

- Remove exec flag from the files, copied from Windows
  ```shell
  find /path/to/location -type f -print0 | xargs -0 chmod 644
  ```

- Clean up CMake build dir
  ```shell
  cd cmake-build-debug
  cmake --build . --target clean
  ```

- Regex to swap C++ arguments to Rusty style
  1. `(\w+)\s+(\w+)\s+\(\)` => `fn $2() -> $1;`
  2. `(\w+)\s+(\w+)\s+\((\w+) (\w+)\)` => `fn $2($4: $3) -> $1;` 
  3. `(\w+)\s+(\w+)\s+\((\w+) (\w+), (\w+) (\w+)\)` => `fn $2($4: $3, $6: $5) -> $1;` 
  and so on. As an example, it replaces
  - `bool hasUnitTypeRequirement (UnitType unit, int amount)`
  - to `fn getBestUnit(best: const BestUnitFilter, pred: const UnitFilter, center: Position, radius: int) -> Unit`
  


### Useful links

- Pretty complex `build.rs` to learn from: https://github.com/alexcrichton/curl-rust/blob/master/curl-sys/build.rs

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

## Installation and run

### MacOS
```bash
pwd  # something like ~/rust/scai/bwapi-rice
DYLD_LIBRARY_PATH=library/bwapi/lib cargo run
```

### Linux
TODO

### Windows
TODO

## Development environment small tasks

### Remove CRLFs from BWAPI include header files
```bash
pwd  # something like ~/rust/scai/bwapi-rice
cd library/bwapi/include
find . -type f -print0 | xargs -0 dos2unix
```

### Useful links

- Pretty complex `build.rs` to learn from: https://github.com/alexcrichton/curl-rust/blob/master/curl-sys/build.rs

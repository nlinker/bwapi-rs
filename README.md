# `bwapi-rice` library

Native [Rust](https://github.com/rust-lang/rust) bindings to [BWAPI](https://github.com/bwapi/bwapi). Main objectives:

- Create full coverage for BWAPI, but with Rust-friendly data structures. 

- Make it [OpenBW](https://github.com/OpenBW/openbw) first, this means this library
  is cross-platform and oriented for machine learning environments,
  such as PyTorch and Tensorflow.
  
- Make it still possible to run under original Starcraft Broodwar 1.16.1 
  using [sc-docker](https://github.com/basil-ladder/sc-docker).

## Installation and run

### MacOS

```bash
pwd  # something like /Users/nick/rust/scai/bwapi-rice
DYLD_LIBRARY_PATH=bwapi/lib  cargo run
```

### Linux

### Windows


